use serde::{Serialize, Deserialize};
use diesel::{Queryable, Insertable};
use crate::schema::users;

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct User {
    pub id: u64,
    pub name: String,
    pub age: u32
}

pub type Users = Vec<User>;

#[derive(Deserialize, Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub name: &'a str,
    pub age: i32
}