use std::net::SocketAddr;

use env_logger::Env;
use error::ServiceError;
use hyper::Server;
use routerify::{Router, RouterService};
use rpel::{get_pool, RpelPool};

use auth::{check_auth, login};
use services::jsonpost;
use users::Users;

mod auth;
mod dbo;
mod error;
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

    // let pool = warp::any().map(move || pool.clone());
    // let u2 = users.clone();
    // let u3 = users.clone();
    // let check_users = warp::any().map(move || users.clone());
    // let login_users = warp::any().map(move || u2.clone());
    // let json_users = warp::any().map(move || u3.clone());
    // let json_length = warp::body::content_length_limit(1024 * 16);

    // .allow_headers(vec![http::header::
    //     "User-Agent",
    //     "Sec-Fetch-Mode",
    //     "Referer",
    //     "Origin",
    //     "Content-Type",
    // ])

    // let cors = warp::cors()
    //     .allow_origins(vec![
    //         "http://localhost:3000",
    //         "chrome-extension://bnmefgocpeggmnpkglmkfoidibbcogcf",
    //         "moz-extension://4b800887-ba22-4cb5-a284-41421b565e0e",
    //     ])
    //     .allow_headers(vec!["content-type", "content-length"])
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
    //     .with(warp::log("cors test"));

    let router = Router::builder()
        .data(State { pool, users })
        .post("go/check", check_auth)
        .post("go/login", login)
        .post("go/json", jsonpost)
        .build()?;

    let service = RouterService::new(router)?;

    let addr = addr.parse::<SocketAddr>()?;

    // Create a server by passing the created service to `.serve` method.
    let server = Server::bind(&addr).serve(service);

    // let addr = SocketAddr::from(addr);

    // let make_svc = make_service_fn(|_conn| async {
    //     // service_fn converts our function into a `Service`
    //     Ok::<_, Infallible>(service_fn(hello_world))
    // });

    // let server = Server::bind(&addr).serve(make_svc);

    Ok(server.await?)
}

fn main() -> Result<(), ServiceError> {
    let rt = tokio::runtime::Runtime::new()?;
    rt.block_on(run_server())
}
