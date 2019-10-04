use actix_web::{web, Responder};
use super::super::models;

pub fn get_all_user() -> impl Responder {
    let a = "" ;
    format!("{:?}", a)
}

pub fn get_user(info: web::Path<(u64)>) -> impl Responder {

    let _name: String = String::from("test");
    let _user = models::user::User{id: info.into_inner(), name: _name, age: 10};

    format!("{:?}", _user)
}

//pub fn create_user() -> impl Responder {}
//pub fn update_user() -> impl Responder {}
//pub fn delete_user() -> impl Responder {}
