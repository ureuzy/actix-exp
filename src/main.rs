mod controllers;
mod models;
mod router;

use actix_web::{App, HttpServer, middleware::Logger};

fn main() -> std::io::Result<()> {

    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .configure(router::routes)
    }).bind("0.0.0.0:8080")?.run()

}