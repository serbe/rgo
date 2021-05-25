use hyper::{body::to_bytes, Body, Request, Response};
use routerify::ext::RequestExt;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::services::{json_response, ClientMessage, Command};
use crate::users::Users;
use crate::{error::ServiceError, State};

#[derive(Debug, Deserialize, Serialize)]
pub struct Auth {
    u: String,
    p: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct A {
    t: String,
    r: i64,
}

#[derive(Serialize)]
struct C {
    r: bool,
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

pub fn check(users: &Users, message: ClientMessage) -> Result<Command, ServiceError> {
    let user = users
        .get_user(&message.addon)
        .ok_or(ServiceError::NotAuth)?;
    user.permissions(message.command)
}
