use std::net::SocketAddr;

use env_logger::Env;
use error::ServiceError;
use hyper::Server;
use routerify::{Middleware, Router, RouterService};
use rpel::{get_pool, RpelPool};

use services::{check_auth, enable_cors_all_middleware_handler, jsonpost, logger, login};
use users::Users;

mod auth;
mod dbo;
mod error;
mod messages;
mod services;
mod users;

pub struct State {
    pub pool: RpelPool,
    pub users: Users,
}

async fn run_server() -> Result<(), ServiceError> {
    dotenv::dotenv().ok();
    env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init();
    let addr = dotenv::var("RGO_ADDR").expect("RGO_ADDR must be set");
    let pg_cfg = dotenv::var("RGO_DB").expect("RGO_DB must be set");
    let pool = get_pool(&pg_cfg)?;
    let users = Users::new(&pool).await?;

    let router = Router::builder()
        .data(State { pool, users })
        .middleware(Middleware::pre(logger))
        .middleware(Middleware::post(enable_cors_all_middleware_handler))
        .post("/go/check", check_auth)
        .post("/go/login", login)
        .post("/go/json", jsonpost)
        .build()?;

    let service = RouterService::new(router)?;

    let addr = addr.parse::<SocketAddr>()?;

    let server = Server::bind(&addr).serve(service);

    Ok(server.await?)
}

fn main() -> Result<(), ServiceError> {
    let rt = tokio::runtime::Runtime::new()?;
    rt.block_on(run_server())
}
