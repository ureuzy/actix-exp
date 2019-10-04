use actix_web::{web, Responder, HttpResponse};

use super::super::models::user::{UserJson, User, Users};


pub fn get_all_user() -> impl Responder {

    let _users: Users = vec![
        User{id: 1, name: String::from("test_user1"), age: 10},
        User{id: 2, name: String::from("test_user2"), age: 15}
    ];
    let users: &dyn UserJson = &_users;

    HttpResponse::Ok().json(users.to_json())
}

pub fn get_user(info: web::Path<(u64)>) -> HttpResponse {

    let _user = User{id: info.into_inner(), name: String::from("test_user"), age: 10};
    let user: &dyn UserJson = &_user;

    HttpResponse::Ok().json(user.to_json())
}

//pub fn create_user() -> impl Responder {}
//pub fn update_user() -> impl Responder {}
//pub fn delete_user() -> impl Responder {}
