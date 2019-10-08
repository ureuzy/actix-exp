use crate::models::user::{User, Users, NewUser};
//use crate::repository::db::MysqlClient;
use crate::schema::users::dsl::users;
use serde::export::fmt::Debug;
use diesel::{RunQueryDsl, MysqlConnection};

pub struct UserRepo{}

pub trait UserRepository {
    fn find_user(&self) -> User;
    fn find_all_user(&self) -> Users;
    fn store_user(&self, connection: &MysqlConnection, new_user: &NewUser) -> Result<usize, diesel::result::Error>;
    fn delete_user(&self);
}

impl UserRepository for UserRepo {

    fn find_user(&self) -> User {
        User{id: 1, name: String::from("test_user1"), age: 10}
    }

    fn find_all_user(&self) -> Users {
        vec![
            User{id: 1, name: String::from("test_user1"), age: 10},
            User{id: 2, name: String::from("test_user2"), age: 15}
        ] as Users
    }

    fn store_user(&self, connection: &MysqlConnection, new_user: &NewUser) -> Result<usize, diesel::result::Error> {
        use diesel::RunQueryDsl;

        diesel::insert_into(users)
            .values(new_user)
            .execute(connection)
    }

    fn delete_user(&self) {}
}


