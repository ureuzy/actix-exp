use serde::{Serialize, Deserialize};
use diesel::{Queryable, Insertable};
use crate::schema::users;

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct User {
    pub id: i32,
    pub name: Option<String>,
    pub age: Option<i32>
}

pub type Users = Vec<User>;

#[derive(Insertable, Deserialize, AsChangeset)]
#[table_name = "users"]
pub struct UserForm {
    pub name: String,
    pub age: i32
}