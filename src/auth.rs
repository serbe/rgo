use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};

use crate::error::ServiceError;
use crate::services::{ClientMessage, Command};
use crate::users::Users;
use crate::AppData;

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

pub async fn login(
    data: web::Data<AppData>,
    params: web::Json<Auth>,
) -> Result<HttpResponse, ServiceError> {
    let reply = data
        .users
        .get_reply(&params.u, &params.p)
        .ok_or(ServiceError::NotAuth)?;
    Ok(HttpResponse::Ok().json(A {
        t: reply.0,
        r: reply.1,
    }))
}

pub async fn check_auth(
    data: web::Data<AppData>,
    params: web::Json<A>,
) -> Result<HttpResponse, ServiceError> {
    let result = data
        .users
        .get_user(&params.t)
        .map(|u| u.role == params.r)
        .ok_or(ServiceError::NotAuth)?;
    Ok(HttpResponse::Ok().json(C { r: result }))
}

pub fn check(users: &Users, message: ClientMessage) -> Result<Command, ServiceError> {
    let user = users
        .get_user(&message.addon)
        .ok_or(ServiceError::NotAuth)?;
    user.permissions(message.command)
}
