#[macro_use]
extern crate diesel;

mod controllers;
mod models;
mod router;
mod repository;
pub mod schema;

use std::env;
use actix_web::{App, web, HttpServer, middleware::Logger};
use crate::repository::db::{establish_connection};

fn main() -> std::io::Result<()> {

    env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let pool = establish_connection();

    HttpServer::new(move || {
        App::new()
            .register_data(web::Data::new(pool.clone()))
            .wrap(Logger::default())
            .configure(router::routes)
    }).bind("0.0.0.0:8080")?.run()

}