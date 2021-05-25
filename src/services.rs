use hyper::{
    body::to_bytes,
    header::{self, HeaderValue},
    Body, Request, Response,
};
use routerify::ext::RequestExt;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::{
    auth::check,
    dbo::{delete_item, get_item, get_list, insert_item, update_item, DBObject},
};
use crate::{error::ServiceError, users::user_cmd};
use crate::{users::UserObject, State};

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

pub async fn jsonpost(req: Request<Body>) -> Result<Response<Body>, ServiceError> {
    let state = req.data::<State>().ok_or(ServiceError::NoState)?;
    let users = state.users.clone();
    let pool = &state.pool.clone();
    let params: ClientMessage = serde_json::from_slice(&to_bytes(req).await?)?;
    let cmd = check(&users, params)?;
    let msg = match cmd {
        Command::Get(object) => match object {
            Object::Item(item) => {
                WsMsg::from_dbo("Get", item.name.clone(), get_item(&item, pool).await)
            }
            Object::List(obj) => WsMsg::from_dbo("Get", obj.clone(), get_list(&obj, pool).await),
        },
        Command::Insert(dbobject) => WsMsg::from_dbo(
            "Insert",
            dbobject.name(),
            Ok(insert_item(dbobject, pool).await.map(|_| DBObject::Null)?),
        ),
        Command::Update(dbobject) => WsMsg::from_dbo(
            "Update",
            dbobject.name(),
            Ok(update_item(dbobject, pool).await.map(|_| DBObject::Null)?),
        ),
        Command::Delete(item) => WsMsg::from_dbo(
            "Delete",
            item.name.clone(),
            Ok(delete_item(&item, pool).await.map(|_| DBObject::Null)?),
        ),
        Command::User(obj) => return user_cmd(obj, pool).await,
    };
    Ok(json_response(json!(msg))?)
}

pub fn json_response(body: Value) -> Result<Response<Body>, ServiceError> {
    Ok(Response::builder()
        .header("Content-Type", "application/json")
        .status(200)
        .body(Body::from(body.to_string()))?)
}

// pub fn enable_cors_all() -> Middleware<Body, ServiceError> {
//     Middleware::post(enable_cors_all_middleware_handler)
// }

pub async fn enable_cors_all_middleware_handler(
    mut res: Response<Body>,
) -> Result<Response<Body>, ServiceError> {
    let headers = res.headers_mut();

    headers.insert(
        header::ACCESS_CONTROL_ALLOW_ORIGIN,
        HeaderValue::from_static("*"),
    );
    headers.insert(
        header::ACCESS_CONTROL_ALLOW_METHODS,
        HeaderValue::from_static("*"),
    );
    headers.insert(
        header::ACCESS_CONTROL_ALLOW_HEADERS,
        HeaderValue::from_static("*"),
    );
    headers.insert(
        header::ACCESS_CONTROL_EXPOSE_HEADERS,
        HeaderValue::from_static("*"),
    );

    Ok(res)
}
