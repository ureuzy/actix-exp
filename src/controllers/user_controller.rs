use actix_web::{web, HttpResponse, HttpRequest};

use crate::models::user::{User, Users, NewUser};
use crate::repository::db::{MysqlPool, MysqlPooledConnection};
use crate::repository::user::{UserRepository, UserRepo};

fn mysql_pool_handler(pool: web::Data<MysqlPool>) -> Result<MysqlPooledConnection, HttpResponse> {
    pool.get().map_err(|e| {
        HttpResponse::InternalServerError().json(e.to_string())
    })
}

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

pub fn create_user(new_user: web::Json<NewUser>, pool: web::Data<MysqlPool>) -> Result<HttpResponse, HttpResponse> {

    let pool = mysql_pool_handler(pool)?;

    let a = UserRepo{};
    let new_user = NewUser{ name: String::from("aaaaaaa"), age: 10};

    a.store_user(&pool, &new_user)
        .map(|user| HttpResponse::Ok().json(user))
        .map_err(|e| HttpResponse::InternalServerError().json(e.to_string()))
}
