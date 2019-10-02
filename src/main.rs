use actix_web::{web, App, HttpServer, Responder};

fn main() -> std::io::Result<()> {
    HttpServer::new(
        || App::new()
            .route("/", web::get().to(|| "ok"))
            .route("/health", web::get().to(|| "ok"))
            .service(web::scope("/users").route("aaa", web::get().to(|| "aaa")))
    ).bind("127.0.0.1:8080")?.run()
}