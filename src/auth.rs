use serde::{Deserialize, Serialize};

use crate::error::ServiceError;
use crate::messages::{ClientMessage, Command};
use crate::users::Users;

#[derive(Debug, Deserialize, Serialize)]
pub struct Auth {
    pub u: String,
    pub p: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct A {
    pub t: String,
    pub r: i64,
}

#[derive(Serialize)]
pub struct C {
    pub r: bool,
}

pub fn check(users: &Users, message: ClientMessage) -> Result<Command, ServiceError> {
    let user = users
        .get_user(&message.addon)
        .ok_or(ServiceError::NotAuth)?;
    user.permissions(message.command)
}
