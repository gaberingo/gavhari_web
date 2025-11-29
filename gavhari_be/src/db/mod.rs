mod middleware;
mod models;

use diesel::{
    prelude::*,
    r2d2::{self, ConnectionManager, Pool},
};
use dotenvy::dotenv;
pub use middleware::DbCheck;
use std::env;

pub type DbPool = Pool<ConnectionManager<PgConnection>>;

pub fn establish_connection() -> DbPool {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(db_url);
    r2d2::Pool::builder()
        .max_size(10)
        .build(manager)
        .expect("Failed to create DB pool")
}
