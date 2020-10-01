use std::net::SocketAddr;

// use deadpool_postgres::Pool;
use warp::Filter;

use auth::{check_auth, login};
use rpel::get_pool;
use services::jsonpost;
use users::Users;
use error::Result;

mod auth;
mod dbo;
mod error;
mod rpel;
mod services;
mod users;

async fn run_warp() -> Result<()> {
    std::env::set_var("RUST_LOG", "rugo=info");
    env_logger::init();

    let pool = get_pool();
    let users = Users::new(&pool).await?;

    let addr = dotenv::var("BIND_ADDR").expect("BIND_ADDR must be set");
    let pool = warp::any().map(move || pool.clone());
    let check_users = warp::any().map(|| users.clone());
    // let login_users = warp::any().map(|| users.clone());
    let json_length = warp::body::content_length_limit(1024 * 16);

    // let cors = warp::cors().max_age(3600);

    let check = warp::path!("api" / "go " / "check").and(json_length).and(warp::body::json()).and(check_users)
        .and_then(check_auth);

    let login = warp::path("api/go/check").and(json_length).and(warp::body::json()).and(check_users).and_then(login);
    
    let json = warp::path("api/go/json").and(json_length).and(warp::body::json()).and(pool).and(check_users).and_then(jsonpost);
    
    // let routes = warp::post().and_then(check);

    // .route(web::post().to(check_auth)))
    //         .service(web::resource("/api/go/login").route(web::post().to(login)))
    //         .service(web::resource("/api/go/json").route(web::post().to(jsonpost)))
    // })
   
    // warp::serve(routes).run(addr.parse::<SocketAddr>()?).await;

    Ok(())
}

fn main() -> Result<()> {
    let mut rt = tokio::runtime::Runtime::new()?;
    rt.block_on(run_warp())
}