use std::io;

use actix_cors::Cors;
use actix_web::{middleware, web, App, HttpServer};
use deadpool_postgres::Pool;

use auth::{check_auth, login};
use rpel::get_pool;
use services::jsonpost;
use users::Users;

mod auth;
mod dbo;
mod error;
mod rpel;
mod services;
mod users;

pub struct AppData {
    pool: Pool,
    users: Users,
}

#[actix_rt::main]
async fn main() -> io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info,actix_server=info");
    env_logger::init();

    let pool = get_pool();
    let users = Users::new(&pool).await.expect("no get users");
    let addr = dotenv::var("BIND_ADDR").expect("BIND_ADDR must be set");

    HttpServer::new(move || {
        App::new()
            .data(AppData {
                pool: pool.clone(),
                users: users.clone(),
            })
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
