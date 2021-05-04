use hyper::{Body, Request, Response, Server, StatusCode};
use routerify::ext::RequestExt;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::error::ServiceError;
use crate::services::{ClientMessage, Command};
use crate::users::Users;

#[derive(Debug, Deserialize, Serialize)]
pub struct Auth {
    u: String,
    p: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct A {
    t: String,
    r: i64,
}

#[derive(Serialize)]
struct C {
    r: bool,
}

// pub async fn login(params: Auth, users: Users) -> Result<warp::reply::Json, warp::Rejection> {
//     let reply = users
//         .get_reply(&params.u, &params.p)
//         .ok_or_else(warp::reject::not_found)?;
//     Ok(warp::reply::json(&A {
//         t: reply.0,
//         r: reply.1,
//     }))
// }

// pub async fn check_auth(
//     params: A,
//     users: Users,
// ) -> std::result::Result<warp::reply::Json, warp::Rejection> {
//     let result = users
//         .get_user(&params.t)
//         .map(|u| u.role == params.r)
//         .map_or(false, |v| v);
//     Ok(warp::reply::json(&C { r: result }))
// }

pub fn check123(users: &Users, message: ClientMessage) -> Result<Command, ServiceError> {
    let user = users
        .get_user(&message.addon)
        .ok_or(ServiceError::NotAuth)?;
    user.permissions(message.command)
}

async fn check(req: Request<Body>) -> Result<Response<Body>, ServiceError> {
    let users = req.data::<Users>().ok_or(ServiceError::NoUsers)?;
    let response = if let Some(a) = req.context::<A>() {
        let result = users
            .get_user(&a.t)
            .map(|u| u.role == a.r)
            .map_or(false, |v| v);
        Response::new(Body::from(json!(&C { r: result }).to_string()))
    } else {
        let r = Response::builder().status(404).body(());
    };

    Ok(Response::new(Body::from("Home page")))
}
