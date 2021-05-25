use std::collections::HashMap;

use hyper::{Body, Response};
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use rpel::{
    user::{User, UserList},
    RpelPool,
};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::services::Command;
use crate::{error::ServiceError, services::json_response};

#[derive(Clone)]
pub struct Users {
    values: HashMap<String, UserData>,
}

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

impl Users {
    pub async fn new(pool: &RpelPool) -> Result<Users, ServiceError> {
        let mut rng = thread_rng();
        let users = UserList::get_all(pool).await?;
        let mut hash_map = HashMap::new();
        for user in users {
            let key = (&mut rng)
                .sample_iter(Alphanumeric)
                .take(20)
                .map(char::from)
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
        Ok(Users { values: hash_map })
    }

    pub fn get_user(&self, key: &str) -> Option<UserData> {
        self.values.get(key).cloned()
    }

    pub fn get_reply(&self, username: &str, userkey: &str) -> Option<(String, i64)> {
        let reply = self
            .values
            .iter()
            .find(|(_key, user)| user.name == username && user.key == userkey)
            .map(|(key, user)| (key, user.role))?;
        Some((reply.0.clone(), reply.1))
    }
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

pub async fn user_cmd(obj: UserObject, pool: &RpelPool) -> Result<Response<Body>, ServiceError> {
    let a = match obj {
        UserObject::Get(id) => WsUserMsg::from_get(User::get(pool, id).await?),
        UserObject::GetList => WsUserMsg::from_list(UserList::get_all(pool).await?),
        UserObject::Insert(item) => WsUserMsg::from_insert(User::insert(pool, item).await?),
        UserObject::Update(item) => WsUserMsg::from_update(User::update(pool, item).await?),
        UserObject::Delete(id) => WsUserMsg::from_delete(User::delete(pool, id).await?),
    };
    Ok(json_response(json!(a))?)
}
