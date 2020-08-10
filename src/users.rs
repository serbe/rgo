use deadpool_postgres::Client;
use serde::{Deserialize, Serialize};

use crate::rpel::user::{User, UserList};

use crate::error::ServiceError;

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

pub async fn user_cmd(obj: UserObject, client: &Client) -> Result<String, ServiceError> {
    let a = match obj {
        UserObject::Get(id) => WsUserMsg::from_get(User::get(&client, id).await?),
        UserObject::GetList => WsUserMsg::from_list(UserList::get_all(&client).await?),
        UserObject::Insert(item) => WsUserMsg::from_insert(User::insert(&client, item).await?),
        UserObject::Update(item) => WsUserMsg::from_update(User::update(&client, item).await?),
        UserObject::Delete(id) => WsUserMsg::from_delete(User::delete(&client, id).await?),
    };
    Ok(serde_json::to_string(&a)?)
}
