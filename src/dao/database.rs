use std::env;

use sqlx::mysql::MySqlPool;

#[derive(Clone)]
pub struct Database {
    pub pool: MySqlPool,
}

impl Database {
    pub async fn new() -> Self {
        let url = env::var("DATABASE_URL").expect("DATABASE_URL is not defined");
        Self {
            pool: MySqlPool::connect(&url).await.expect("Failed to connect to database"),
        }
    }
}