use serde::{Serialize, Deserialize};

pub trait UserJson {
    fn to_json(&self) -> String;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: u64,
    pub name: String,
    pub age: u32
}

pub type Users = Vec<User>;

impl UserJson for User {
    fn to_json(&self) -> String {
        return serde_json::to_string(&self).unwrap();
    }
}

impl UserJson for Users {
    fn to_json(&self) -> String {
        return serde_json::to_string(&self).unwrap();
    }
}