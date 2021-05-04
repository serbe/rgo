use deadpool_postgres::Pool;
use serde::{Deserialize, Serialize};

use crate::auth::check;
use crate::dbo::{delete_item, get_item, get_list, insert_item, update_item, DbObject};
use crate::error::ServiceError;
use crate::users::{UserObject, Users};

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
pub struct WebMsg {
    pub command: String,
    pub name: String,
    pub object: DbObject,
    pub error: String,
}

// impl WebMsg {
//     pub fn from_dbo(command: &str, name: String, dbo: Result<DbObject, ServiceError>) -> WebMsg {
//         match dbo {
//             Ok(object) => WebMsg {
//                 command: command.to_string(),
//                 name,
//                 object,
//                 error: String::new(),
//             },
//             Err(err) => WebMsg {
//                 command: command.to_string(),
//                 name,
//                 object: DbObject::Null,
//                 error: err.to_string(),
//             },
//         }
//     }
// }

// pub async fn jsonpost(
//     params: ClientMessage,
//     pool: Pool,
//     users: Users,
// ) -> Result<warp::reply::Json, warp::Rejection> {
//     let msg: ClientMessage = params;
//     let cmd = check(&users, msg)?;
//     let client = pool.get().await.map_err(ServiceError::PoolError)?;
//     let msg = match cmd {
//         Command::Get(object) => match object {
//             Object::Item(item) => {
//                 WebMsg::from_dbo("Get", item.name.clone(), get_item(&item, &client).await)
//             }
//             Object::List(obj) => {
//                 WebMsg::from_dbo("Get", obj.clone(), get_list(&obj, &client).await)
//             }
//         },
//         Command::Insert(dbobject) => WebMsg::from_dbo(
//             "Insert",
//             dbobject.name(),
//             Ok(insert_item(dbobject, &client)
//                 .await
//                 .map(|_| DbObject::Null)?),
//         ),
//         Command::Update(dbobject) => WebMsg::from_dbo(
//             "Update",
//             dbobject.name(),
//             Ok(update_item(dbobject, &client)
//                 .await
//                 .map(|_| DbObject::Null)?),
//         ),
//         Command::Delete(item) => WebMsg::from_dbo(
//             "Delete",
//             item.name.clone(),
//             Ok(delete_item(&item, &client).await.map(|_| DbObject::Null)?),
//         ),
//         Command::User(obj) => return user_cmd(obj, &client).await,
//     };
//     Ok(warp::reply::json(&msg))
// }
