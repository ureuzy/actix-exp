mod controllers;
mod models;

use actix_web::{web, App, HttpServer};

fn main() -> std::io::Result<()> {
    HttpServer::new(
        || App::new()
            .route("/", web::get().to(|| "ok"))
            .route("/health", web::get().to(|| "healthy"))
            .service(web::scope("/users")
                .route("/", web::get().to(controllers::user_controller::get_all_user))
                .route("/{id}", web::get().to(controllers::user_controller::get_user))
                .route("/{id}", web::post().to(controllers::user_controller::create_user()))
                .route("/{id}", web::put().to(controllers::user_controller::update_user()))
                .route("/{id}", web::delete().to(controllers::user_controller::delete_user()))
            )
    ).bind("127.0.0.1:8080")?.run()

}
