use std::clone::Clone;
use std::collections::HashMap;
use std::iter;
use std::sync::Mutex;

use actix::{fut, Actor, Addr, Context, Handler, ResponseActFuture};
use deadpool_postgres::{Client, Pool};
use once_cell::sync::OnceCell;
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use serde::{Deserialize, Serialize};

use crate::rpel::get_pool;
use crate::rpel::user::UserList;
use crate::auth::check;
use crate::dbo::{delete_item, get_item, get_list, insert_item, update_item, DBObject};
use crate::error::ServiceError;
use crate::server::{Msg, Server};
use crate::users::{user_cmd, UserObject};

#[derive(Clone)]
pub struct UserData {
    pub id: i64,
    pub name: String,
    pub key: String,
    pub role: i64,
}

impl UserData {
    pub fn permissions(&self, command: Command) -> Result<Command, ServiceError> {
        if match &command {
            Command::Get(_) => self.role >> 1 > 0,
            Command::Insert(_) => self.role >> 2 > 0,
            Command::Update(_) => self.role >> 3 > 0,
            Command::Delete(_) => self.role >> 4 > 0,
            Command::User(UserObject::Get(_)) => self.role >> 5 > 0,
            Command::User(UserObject::GetList) => self.role >> 5 > 0,
            Command::User(UserObject::Insert(_)) => self.role >> 6 > 0,
            Command::User(UserObject::Update(_)) => self.role >> 7 > 0,
            Command::User(UserObject::Delete(_)) => self.role >> 8 > 0,
        } {
            Ok(command)
        } else {
            Err(ServiceError::NotPermission)
        }
    }
}

static USERS: OnceCell<Mutex<HashMap<String, UserData>>> = OnceCell::new();

pub async fn global_init() -> Result<(), ServiceError> {
    let mut rng = thread_rng();
    let client = get_pool().get().await?;
    let users = UserList::get_all(&client)
        .await
        .expect("get UserList failed");
    let mut hash_map = HashMap::new();
    for user in users {
        let key = iter::repeat(())
            .map(|()| rng.sample(Alphanumeric))
            .take(20)
            .collect();
        hash_map.insert(
            key,
            UserData {
                id: user.id,
                name: user.name.clone(),
                key: user.key.clone(),
                role: user.role,
            },
        );
    }
    let mutex = Mutex::new(hash_map);
    let _result = USERS.set(mutex);
    Ok(())
}

pub fn check_global() {
    let _users = USERS.get().unwrap().lock().unwrap();
}

pub fn get_user(key: &str) -> Option<UserData> {
    let mutex = USERS.get()?;
    let users = mutex.lock().ok()?;
    let user = users.get(key)?;
    Some(user.clone())
}

pub fn get_reply(username: &str, userkey: &str) -> Option<(String, i64)> {
    let mutex = USERS.get()?;
    let users = mutex.lock().ok()?;
    let reply = users
        .iter()
        .find(|(_key, user)| user.name == username && user.key == userkey)
        .map(|(key, user)| (key, user.role))?;
    Some((reply.0.clone(), reply.1))
}

#[derive(Serialize)]
pub struct WsMsg {
    pub command: String,
    pub name: String,
    pub object: DBObject,
    pub error: String,
}

impl WsMsg {
    pub fn from_dbo(command: &str, name: String, dbo: Result<DBObject, ServiceError>) -> WsMsg {
        match dbo {
            Ok(object) => WsMsg {
                command: command.to_string(),
                name,
                object,
                error: String::new(),
            },
            Err(err) => WsMsg {
                command: command.to_string(),
                name,
                object: DBObject::Null,
                error: err.to_string(),
            },
        }
    }
}

#[derive(Clone)]
pub struct DB {
    pool: Pool,
    server: Addr<Server>,
}

impl Actor for DB {
    type Context = Context<Self>;
}

impl DB {
    pub fn new(server: Addr<Server>) -> DB {
        let pool = get_pool();
        DB { pool, server }
    }

    async fn client(&self) -> Result<Client, ServiceError> {
        Ok(self.pool.get().await?)
    }

    async fn get_reply(self, message: String) -> Result<String, ServiceError> {
        let client_message: ClientMessage = serde_json::from_str(&message)?;
        let cmd: Command = check(client_message)?;
        let client = self.client().await?;
        let msg = match cmd {
            Command::Get(object) => match object {
                Object::Item(item) => {
                    WsMsg::from_dbo("Get", item.name.clone(), get_item(&item, &client).await)
                }
                Object::List(obj) => {
                    WsMsg::from_dbo("Get", obj.clone(), get_list(&obj, &client).await)
                }
            },
            Command::Insert(dbobject) => WsMsg::from_dbo(
                "Insert",
                dbobject.name(),
                Ok(insert_item(dbobject, &client)
                    .await
                    .map(|_| DBObject::Null)?),
            ),
            Command::Update(dbobject) => WsMsg::from_dbo(
                "Update",
                dbobject.name(),
                Ok(update_item(dbobject, &client)
                    .await
                    .map(|_| DBObject::Null)?),
            ),
            Command::Delete(item) => WsMsg::from_dbo(
                "Delete",
                item.name.clone(),
                Ok(delete_item(&item, &client).await.map(|_| DBObject::Null)?),
            ),
            Command::User(obj) => return user_cmd(obj, &client).await,
        };
        Ok(serde_json::to_string(&msg)?)
    }
}

impl Handler<Msg> for DB {
    type Result = ResponseActFuture<Self, Result<String, ServiceError>>;

    fn handle(&mut self, msg: Msg, _: &mut Context<Self>) -> Self::Result {
        let message = msg.0;
        let this = self.clone();
        Box::new(fut::wrap_future(this.get_reply(message)))
    }
}

#[derive(Deserialize)]
pub struct ClientMessage {
    pub command: Command,
    pub addon: String,
}

#[derive(Debug, Deserialize)]
pub struct Item {
    pub name: String,
    pub id: i64,
}

#[derive(Deserialize)]
pub enum Object {
    Item(Item),
    List(String),
}

#[derive(Deserialize)]
pub enum Command {
    Get(Object),
    Insert(DBObject),
    Update(DBObject),
    Delete(Item),
    User(UserObject),
}
