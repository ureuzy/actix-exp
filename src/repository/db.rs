use std::env;
use actix::{Actor, SyncContext};
use diesel::prelude::*;
use diesel::mysql::MysqlConnection;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection, PoolError};
use dotenv::dotenv;

pub type MysqlPool = Pool<ConnectionManager<MysqlConnection>>;
pub type MysqlPooledConnection = PooledConnection<ConnectionManager<MysqlConnection>>;

fn init_pool(database_url: &str) -> Result<MysqlPool, PoolError> {
        let manager = ConnectionManager::<MysqlConnection>::new(database_url);
        Pool::builder().build(manager)
}

pub fn establish_connection() -> MysqlPool {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    init_pool(&database_url).expect("Failed to create pool")
}
