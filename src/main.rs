use std::io;

use actix::Actor;
use actix_cors::Cors;
use actix_web::{middleware, web, App, HttpServer};

use auth::{check_auth, login};
use db::{check_global, global_init};
use server::Server;
use session::wsroute;

mod auth;
mod db;
mod dbo;
mod error;
mod server;
mod session;
mod users;
mod rpel;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info,actix_server=info");

    env_logger::init();

    global_init().await.unwrap();
    check_global();
    let addr = dotenv::var("BIND_ADDR").expect("BIND_ADDR must be set");

    let server = Server::default().start();

    HttpServer::new(move || {
        App::new()
            .data(server.clone())
            .wrap(Cors::new().max_age(3600).finish())
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            .service(web::resource("/api/go/check").route(web::post().to(check_auth)))
            .service(web::resource("/api/go/login").route(web::post().to(login)))
            .service(web::resource("/api/go").route(web::get().to(wsroute)))
    })
    .bind(addr)?
    .run()
    .await
}
