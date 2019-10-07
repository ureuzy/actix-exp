mod controllers;
mod models;
mod router;
mod repository;
mod schema;

use actix_web::{App, HttpServer, middleware::Logger};
use crate::repository::mysql_client::{new_mysql_client, MysqlClient};
use crate::repository::user::UserRepository;

fn main() -> std::io::Result<()> {

    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let client: MysqlClient = new_mysql_client();

    let user = client.store_user();

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .configure(router::routes)
    }).bind("0.0.0.0:8080")?.run()
//
}