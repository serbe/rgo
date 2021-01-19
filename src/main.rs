use std::net::SocketAddr;
use std::convert::Infallible;

use env_logger::Env;
use hyper::{Body, Request, Response, Server};
use hyper::service::{make_service_fn, service_fn};

use auth::{check_auth, login};
use error::Result;
use rpel::get_pool;
use services::jsonpost;
use users::Users;

mod auth;
mod dbo;
mod error;
mod rpel;
mod services;
mod users;

async fn hello_world(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    Ok(Response::new("Hello, World".into()))
}

async fn run_server() -> Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init();

    let pool = get_pool();
    let users = Users::new(&pool).await?;

    let addr = dotenv::var("BIND_ADDR").expect("BIND_ADDR must be set");
    let pool = warp::any().map(move || pool.clone());
    let u2 = users.clone();
    let u3 = users.clone();
    let check_users = warp::any().map(move || users.clone());
    let login_users = warp::any().map(move || u2.clone());
    let json_users = warp::any().map(move || u3.clone());
    let json_length = warp::body::content_length_limit(1024 * 16);

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

        let make_svc = make_service_fn(|_conn| async {
            // service_fn converts our function into a `Service`
            Ok::<_, Infallible>(service_fn(hello_world))
        });

    let addr = SocketAddr::from(addr);

    let make_svc = make_service_fn(|_conn| async {
        // service_fn converts our function into a `Service`
        Ok::<_, Infallible>(service_fn(hello_world))
    });

    let server = Server::bind(&addr).serve(make_svc);

    Ok(())
}

fn main() -> Result<()> {
    let mut rt = tokio::runtime::Runtime::new()?;
    rt.block_on(run_server())
}
