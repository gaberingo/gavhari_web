mod middleware;
pub mod models;

use diesel::{
    prelude::*,
    r2d2::{self, ConnectionManager, Pool, PooledConnection},
};
use dotenvy::dotenv;
use std::env;

pub use middleware::DbCheck;
pub type DbPool = Pool<ConnectionManager<PgConnection>>;
pub type DbPooled = PooledConnection<ConnectionManager<PgConnection>>;

pub fn establish_connection() -> DbPool {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(db_url);
    r2d2::Pool::builder()
        .max_size(10)
        .build(manager)
        .expect("Failed to create DB pool")
}
