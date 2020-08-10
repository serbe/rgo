use std::io;

use actix_cors::Cors;
use actix_web::{middleware, web, App, HttpServer};

use auth::{check_auth, login};
use rpel::get_pool;
use services::jsonpost;
use users::{check_global, global_init};

mod auth;
mod dbo;
mod error;
mod rpel;
mod services;
mod users;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info,actix_server=info");

    env_logger::init();

    global_init().await.unwrap();
    check_global();
    let addr = dotenv::var("BIND_ADDR").expect("BIND_ADDR must be set");

    HttpServer::new(move || {
        App::new()
            .data(get_pool())
            .wrap(Cors::new().max_age(3600).finish())
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            .service(web::resource("/api/go/check").route(web::post().to(check_auth)))
            .service(web::resource("/api/go/login").route(web::post().to(login)))
            .service(web::resource("/api/go/json").route(web::post().to(jsonpost)))
    })
    .bind(addr)?
    .run()
    .await
}
