use crate::models::user::{User, Users, UserForm};
use diesel::{QueryDsl, RunQueryDsl, QueryResult, MysqlConnection};
use crate::schema::users;

pub struct UserRepo<'a>{
    pub conn: &'a MysqlConnection
}

pub trait UserRepository {
    fn find(&self, id: &i32) -> QueryResult<User>;
    fn find_all(&self) -> QueryResult<Users>;
    fn store(&self, new_user: &UserForm) -> QueryResult<usize>;
    fn update(&self, id: &i32, user: &UserForm) -> QueryResult<User>;
    fn delete(&self, id: &i32) -> QueryResult<usize>;
}

impl<'a> UserRepository for UserRepo<'a> {

    fn find(&self, id: &i32) -> QueryResult<User> {
        users::table.find(id).get_result::<User>(self.conn)
    }

    fn find_all(&self) -> QueryResult<Users> {
        users::table.load(self.conn)
    }

    fn store(&self, user_form: &UserForm) -> QueryResult<usize> {
        diesel::insert_into(users::table)
            .values(user_form)
            .execute(self.conn)
    }

    fn update(&self, id: &i32, user_form: &UserForm) -> QueryResult<User> {
        diesel::update(users::table.find(id))
            .set(user_form)
            .execute(self.conn)?;

        users::table.find(id).get_result::<User>(self.conn)
    }

    fn delete(&self, id: &i32) -> QueryResult<usize> {
        diesel::delete(users::table.find(id))
            .execute(self.conn)
    }
}


