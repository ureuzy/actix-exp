use actix_web::{web, HttpResponse};
use crate::models::user::{UserForm};
use crate::repository::db::{MysqlPool, MysqlPooledConnection};
use crate::repository::user::{UserRepository, UserRepo};
use actix_web::dev::HttpResponseBuilder;

fn mysql_pool_handler(pool: web::Data<MysqlPool>) -> Result<MysqlPooledConnection, HttpResponse> {
    pool.get().map_err(|e| {
        HttpResponse::InternalServerError().json(e.to_string())
    })
}

pub fn get_all_user(pool: web::Data<MysqlPool>) -> Result<HttpResponse, HttpResponse> {
    let connection = mysql_pool_handler(pool)?;
    let user_repo = UserRepo{conn: &connection};
    user_repo.find_all()
        .map(|users| HttpResponse::Ok().json(users))
        .map_err( |e| HttpResponse::InternalServerError().json(e.to_string()))
}

pub fn get_user(info: web::Path<i32>, pool: web::Data<MysqlPool>) -> Result<HttpResponse, HttpResponse> {
    let connection = mysql_pool_handler(pool)?;
    let user_repo = UserRepo{conn: &connection};
    user_repo.find(&info.into_inner())
        .map(|user| HttpResponse::Ok().json(user))
        .map_err( |e| HttpResponse::InternalServerError().json(e.to_string()))
}

pub fn create_user(user_form: web::Json<UserForm>, pool: web::Data<MysqlPool>) -> Result<HttpResponseBuilder, HttpResponse> {
    let connection = mysql_pool_handler(pool)?;
    let user_repo = UserRepo{conn: &connection};
    user_repo.store(&user_form)
        .map(| _ | HttpResponse::Created())
        .map_err(|e| HttpResponse::InternalServerError().json(e.to_string()))
}

pub fn update_user(info: web::Path<i32>, user_form: web::Json<UserForm>, pool: web::Data<MysqlPool>) -> Result<HttpResponse, HttpResponse> {
    let connection = mysql_pool_handler(pool)?;
    let user_repo = UserRepo{conn: &connection};
    user_repo.update(&info.into_inner(), &user_form)
        .map(| user | HttpResponse::Ok().json(user))
        .map_err(|e| HttpResponse::InternalServerError().json(e.to_string()))
}

pub fn delete_user(info: web::Path<i32>, pool: web::Data<MysqlPool>) -> Result<HttpResponseBuilder, HttpResponse> {
    let connection = mysql_pool_handler(pool)?;
    let user_repo = UserRepo{conn: &connection};
    user_repo.delete(&info.into_inner())
        .map(| _ | HttpResponse::Ok())
        .map_err(|e| HttpResponse::InternalServerError().json(e.to_string()))
}