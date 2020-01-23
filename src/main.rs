#[macro_use]
extern crate diesel;

mod controllers;
mod models;
mod router;
mod repository;
pub mod schema;

use std::env;
use actix_cors::Cors;
use actix_web::{http::header, App, web, HttpServer, middleware::Logger};
use crate::repository::db::{establish_connection};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {

    env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let pool = establish_connection();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(
                Cors::new()
                    .send_wildcard()
                    .allowed_methods(vec!["GET", "POST"])
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                    .allowed_header(header::CONTENT_TYPE)
                    .max_age(3600)
                    .finish(),
            )
            .wrap(Logger::default())
            .configure(router::routes)
    }).bind("0.0.0.0:8080")?.run().await

}