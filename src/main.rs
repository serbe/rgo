use env_logger::Env;
use std::net::SocketAddr;

use warp::{http::Method, Filter};

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

async fn run_warp() -> Result<()> {
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

    let cors = warp::cors()
        .allow_origin("http://localhost:3000")
        .allow_headers(vec!["content-type"])
        .allow_methods(&[Method::GET, Method::POST, Method::DELETE, Method::OPTIONS])
        .max_age(3600);

    let check = warp::path!("api" / "go " / "check")
        .and(json_length)
        .and(warp::body::json())
        .and(check_users)
        .and_then(check_auth);

    let login = warp::path!("api" / "go" / "check")
        .and(json_length)
        .and(warp::body::json())
        .and(login_users)
        .and_then(login);

    let json = warp::path!("api" / "go" / "json")
        .and(json_length)
        .and(warp::body::json())
        .and(pool)
        .and(json_users)
        .and_then(jsonpost);

    let routes = warp::post()
        .and(check.or(login).or(json))
        .with(cors)
        .with(warp::log("cors test"));

    warp::serve(routes).run(addr.parse::<SocketAddr>()?).await;

    Ok(())
}

fn main() -> Result<()> {
    let mut rt = tokio::runtime::Runtime::new()?;
    rt.block_on(run_warp())
}
