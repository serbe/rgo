use hyper::{
    body::to_bytes,
    header::{self, HeaderValue},
    Body, Request, Response,
};
use log::debug;
use routerify::ext::RequestExt;
use serde_json::{from_slice, json, Value};

use crate::{
    auth::{check, C},
    dbo::{delete_item, get_item, get_list, insert_item, update_item, DbObject},
    messages::{ClientMessage, Command, WsMsg},
};
use crate::{
    auth::{Auth, A},
    State,
};
use crate::{error::ServiceError, users::user_cmd};

pub async fn jsonpost(req: Request<Body>) -> Result<Response<Body>, ServiceError> {
    let state = req.data::<State>().ok_or(ServiceError::NoState)?;
    let users = state.users.clone();
    let pool = &state.pool.clone();
    let params: ClientMessage = from_slice(&to_bytes(req).await?)?;
    let cmd = check(&users, params)?;
    let msg = match cmd {
        Command::GetItem(item) => {
            WsMsg::from_dbo("GetItem", item.name.clone(), get_item(&item, pool).await)
        }
        Command::GetList(list) => {
            WsMsg::from_dbo("GetList", list.clone(), get_list(&list, pool).await)
        }
        Command::InsertItem(dbobject) => WsMsg::from_dbo(
            "InsertItem",
            dbobject.name(),
            Ok(insert_item(dbobject, pool).await.map(|_| DbObject::Null)?),
        ),
        Command::UpdateItem(dbobject) => WsMsg::from_dbo(
            "UpdateItem",
            dbobject.name(),
            Ok(update_item(dbobject, pool).await.map(|_| DbObject::Null)?),
        ),
        Command::DeleteItem(item) => WsMsg::from_dbo(
            "DeleteItem",
            item.name.clone(),
            Ok(delete_item(&item, pool).await.map(|_| DbObject::Null)?),
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
        // HeaderValue::from_static("http://localhost:3000"),
    );
    // HeaderValue::from_static("http://localhost:3000"),
    // http://localhost:3000, chrome-extension://bnmefgocpeggmnpkglmkfoidibbcogcf, moz-extension://4b800887-ba22-4cb5-a284-41421b565e0e
    headers.insert(
        header::ACCESS_CONTROL_ALLOW_METHODS,
        HeaderValue::from_static("POST, OPTIONS"),
    );
    headers.insert(
        header::ACCESS_CONTROL_ALLOW_HEADERS,
        HeaderValue::from_static("*"),
    );
    // debug!("{:?}", headers);

    Ok(res)
}

pub async fn logger(req: Request<Body>) -> Result<Request<Body>, ServiceError> {
    debug!(
        "{} {} {}",
        req.remote_addr(),
        req.method(),
        req.uri().path()
    );
    Ok(req)
}

pub async fn check_auth(req: Request<Body>) -> Result<Response<Body>, ServiceError> {
    let users = req
        .data::<State>()
        .ok_or(ServiceError::NoState)?
        .users
        .clone();
    let params: A = serde_json::from_slice(&to_bytes(req).await?)?;
    let result = users
        .get_user(&params.t)
        .map(|u| u.role == params.r)
        .map_or(false, |v| v);
    Ok(json_response(json!(&C { r: result }))?)
}

pub async fn login(req: Request<Body>) -> Result<Response<Body>, ServiceError> {
    let users = req
        .data::<State>()
        .ok_or(ServiceError::NoState)?
        .users
        .clone();
    let params: Auth = serde_json::from_slice(&to_bytes(req).await?)?;
    let reply = users
        .get_reply(&params.u, &params.p)
        .ok_or(ServiceError::NotAuth)?;
    Ok(json_response(json!(&A {
        t: reply.0,
        r: reply.1,
    }))?)
}
