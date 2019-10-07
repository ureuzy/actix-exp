use diesel::prelude::*;
use diesel::mysql::MysqlConnection;
use dotenv::dotenv;
use std::env;

pub struct MysqlClient {
    pub conn: MysqlConnection
}

pub fn new_mysql_client() -> MysqlClient {

    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    let conn: MysqlConnection = MysqlConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url));

    MysqlClient{
        conn
    }
}