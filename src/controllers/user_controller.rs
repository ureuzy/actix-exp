use actix_web::{web, HttpResponse, HttpRequest};

use crate::models::user::{User, Users};

pub fn get_all_user() -> HttpResponse {

    let users: Users = vec![
        User{id: 1, name: String::from("test_user1"), age: 10},
        User{id: 2, name: String::from("test_user2"), age: 15}
    ];

    HttpResponse::Ok().json(users)
}

pub fn get_user(info: web::Path<(u64)>) -> HttpResponse {

    let user: User = User{id: 1, name: String::from("test_user1"), age: 10};

    HttpResponse::Ok().json(user)
}

pub fn create_user(req: HttpRequest) -> HttpResponse {

    HttpResponse::Created().finish()
}
