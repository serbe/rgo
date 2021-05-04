use std::net::SocketAddr;

use deadpool_postgres::{Manager, Pool};
use env_logger::Env;
use hyper::{Body, Request, Response, Server, StatusCode};
use routerify::{ext::RequestExt, Middleware, RequestInfo, Router, RouterService};
use tokio::runtime::Runtime;

// use auth::{check_auth, login};
use error::ServiceError;
use rpel::get_pool;
// use services::jsonpost;
use users::Users;

mod auth;
mod dbo;
mod error;
mod rpel;
mod services;
mod users;

async fn run_server() -> Result<(), ServiceError> {
    env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init();

    let pool = get_pool();
    let users = Users::new(&pool).await?;

    let addr = dotenv::var("BIND_ADDR").expect("empty BIND_ADDR");

    // let u2 = users.clone();
    // let u3 = users.clone();
    // let check_users = warp::any().map(move || users.clone());
    // let login_users = warp::any().map(move || u2.clone());
    // let json_users = warp::any().map(move || u3.clone());
    // let json_length = warp::body::content_length_limit(1024 * 16);

    // let cors = warp::cors()
    //     .allow_origins(vec![
    //         "http://localhost:8080",
    //         "http://localhost:3000",
    //         "chrome-extension://bnmefgocpeggmnpkglmkfoidibbcogcf",
    //         "moz-extension://4b800887-ba22-4cb5-a284-41421b565e0e",
    //         "https://udds.ru",
    //     ])
    //     .allow_headers(vec![
    //         "content-type",
    //         "content-length",
    //         "access-control-allow-origin",
    //     ])
    //     .allow_methods(&[Method::GET, Method::POST, Method::DELETE, Method::OPTIONS])
    //     .max_age(3600);

    // let check = warp::path!("go" / "check")
    //     .and(json_length)
    //     .and(warp::body::json())
    //     .and(check_users)
    //     .and_then(check_auth);

    // let login = warp::path!("go" / "login")
    //     .and(json_length)
    //     .and(warp::body::json())
    //     .and(login_users)
    //     .and_then(login);

    // let json = warp::path!("go" / "json")
    //     .and(json_length)
    //     .and(warp::body::json())
    //     .and(pool)
    //     .and(json_users)
    //     .and_then(jsonpost);

    // let routes = warp::post()
    //     .and(check.or(login).or(json))
    //     .with(cors)
    //     .with(warp::log("cors test"))
    //     .with(warp::compression::deflate());

    let router: Router<Body, ServiceError> = Router::builder()
        .data(pool)
        .data(users)
        .post("/go/check", check)
        .build()
        .map_err(|err| ServiceError::Router(err.to_string()))?;

    let service = RouterService::new(router).unwrap();

    let server = Server::bind(&addr.parse::<SocketAddr>()?).serve(service);

    Ok(server.await?)
}

fn main() -> Result<(), ServiceError> {
    let rt = Runtime::new()?;
    rt.block_on(run_server())
}
