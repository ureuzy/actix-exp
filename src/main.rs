mod controllers;
mod models;
mod router;

use actix_web::{App, HttpServer};

fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().configure(router::routes)
    }).bind("0.0.0.0:8080")?.run()
}