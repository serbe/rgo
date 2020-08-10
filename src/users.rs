use std::collections::HashMap;
use std::iter;
use std::sync::Mutex;

use actix_web::HttpResponse;
use deadpool_postgres::Client;
use once_cell::sync::OnceCell;
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use serde::{Deserialize, Serialize};

use crate::error::ServiceError;
use crate::rpel::get_pool;
use crate::rpel::user::{User, UserList};
use crate::services::Command;

pub static USERS: OnceCell<Mutex<HashMap<String, UserData>>> = OnceCell::new();

#[derive(Clone)]
pub struct UserData {
    pub id: i64,
    pub name: String,
    pub key: String,
    pub role: i64,
}

#[derive(Serialize, Deserialize)]
pub enum UserObject {
    Get(i64),
    GetList,
    Insert(User),
    Update(User),
    Delete(i64),
}

#[derive(Serialize, Deserialize)]
pub enum DBUserObject {
    Null,
    User(User),
    UserList(Vec<UserList>),
    ID(i64),
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

#[derive(Serialize, Deserialize)]
pub struct WsUserMsg {
    pub command: String,
    pub object: DBUserObject,
    pub error: String,
}

impl WsUserMsg {
    fn from_get(object: User) -> Self {
        WsUserMsg {
            command: "Get".to_string(),
            object: DBUserObject::User(object),
            error: String::new(),
        }
    }

    fn from_list(object: Vec<UserList>) -> Self {
        WsUserMsg {
            command: "GetList".to_string(),
            object: DBUserObject::UserList(object),
            error: String::new(),
        }
    }

    fn from_insert(object: User) -> Self {
        WsUserMsg {
            command: "Insert".to_string(),
            object: DBUserObject::ID(object.id),
            error: String::new(),
        }
    }

    fn from_update(object: u64) -> Self {
        WsUserMsg {
            command: "Update".to_string(),
            object: DBUserObject::ID(object as i64),
            error: String::new(),
        }
    }

    fn from_delete(object: u64) -> Self {
        WsUserMsg {
            command: "Delete".to_string(),
            object: DBUserObject::ID(object as i64),
            error: String::new(),
        }
    }
}

pub async fn user_cmd(obj: UserObject, client: &Client) -> Result<HttpResponse, ServiceError> {
    let a = match obj {
        UserObject::Get(id) => WsUserMsg::from_get(User::get(&client, id).await?),
        UserObject::GetList => WsUserMsg::from_list(UserList::get_all(&client).await?),
        UserObject::Insert(item) => WsUserMsg::from_insert(User::insert(&client, item).await?),
        UserObject::Update(item) => WsUserMsg::from_update(User::update(&client, item).await?),
        UserObject::Delete(id) => WsUserMsg::from_delete(User::delete(&client, id).await?),
    };
    Ok(HttpResponse::Ok().json(&a))
}
