use serde::{Deserialize, Serialize};

use crate::error::ServiceError;
use crate::services::{ClientMessage, Command};
use crate::users::Users;

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

pub async fn login(params: Auth, users: Users) -> Result<warp::reply::Json, warp::Rejection> {
    let reply = users
        .get_reply(&params.u, &params.p)
        .ok_or_else(warp::reject::not_found)?;
    Ok(warp::reply::json(&A {
        t: reply.0,
        r: reply.1,
    }))
}

pub async fn check_auth(
    params: A,
    users: Users,
) -> std::result::Result<warp::reply::Json, warp::Rejection> {
    let result = users
        .get_user(&params.t)
        .map(|u| u.role == params.r)
        .map_or(false, |v| v);
    Ok(warp::reply::json(&C { r: result }))
}

pub fn check(users: &Users, message: ClientMessage) -> Result<Command, ServiceError> {
    let user = users
        .get_user(&message.addon)
        .ok_or(ServiceError::NotAuth)?;
    user.permissions(message.command)
}
