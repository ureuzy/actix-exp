use super::super::models::user::{User, Users, NewUser};
use crate::repository::mysql_client::MysqlClient;
use diesel::RunQueryDsl;

pub trait UserRepository {
    fn find_user(&self) -> User;
    fn find_all_user(&self) -> Users;
    fn store_user(&self);
    fn delete_user(&self);
}

impl UserRepository for MysqlClient {

    fn find_user(&self) -> User {
        User{id: 1, name: String::from("test_user1"), age: 10}
    }

    fn find_all_user(&self) -> Users {
        vec![
            User{id: 1, name: String::from("test_user1"), age: 10},
            User{id: 2, name: String::from("test_user2"), age: 15}
        ] as Users
    }

    fn store_user(&self) {
        use crate::schema;

        let new_user = NewUser{ name: "hoge", age: 10};

        diesel::insert_into(user::table)
            .values(&new_user)
            .get_result(self.conn)
            .expect("Error saving new user")

    }

    fn delete_user(&self) {}
}


