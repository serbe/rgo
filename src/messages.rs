use serde::{Deserialize, Serialize};

use crate::{dbo::DbObject, error::ServiceError, users::UserObject};

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
    Insert(DbObject),
    Update(DbObject),
    Delete(Item),
    User(UserObject),
}

#[derive(Serialize)]
pub struct WsMsg {
    pub command: String,
    pub name: String,
    pub object: DbObject,
    pub error: String,
}

impl WsMsg {
    pub fn from_dbo(command: &str, name: String, dbo: Result<DbObject, ServiceError>) -> WsMsg {
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
                object: DbObject::Null,
                error: err.to_string(),
            },
        }
    }
}
