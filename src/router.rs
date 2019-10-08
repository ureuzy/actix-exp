use actix_web::{web};
use crate::controllers;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg
        .route("/", web::get().to(|| "ok"))
        .route("/health", web::get().to(|| "healthy"))
        .service(web::scope("/users")
                     .route("", web::get().to(controllers::user_controller::get_all_user))
                     .route("/{id}", web::get().to(controllers::user_controller::get_user))
                     .route("/{id}", web::post().to(controllers::user_controller::create_user))
//                     .route("/{id}", web::put().to(controllers::user_controller::update_user()))
//                     .route("/{id}", web::delete().to(controllers::user_controller::delete_user()));
);
}