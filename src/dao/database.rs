use std::{env, path::Path};

use sqlx::mysql::MySqlPool;

#[derive(Clone)]
pub struct Database {
    pub pool: MySqlPool,
    articlies_save_path: String,
}

impl Database {
    pub async fn new() -> Self {
        let url = env::var("DATABASE_URL").expect("DATABASE_URL is not defined");
        Self {
            pool: MySqlPool::connect(&url).await.expect("Failed to connect to database"),
            articlies_save_path: env::var("ARTICLIES_SAVE_PATH").expect("ARTICLIES_SAVE_PATH is not defined")
        }
    }
    pub fn articlies_save_path(&self) -> &Path {
        Path::new(&self.articlies_save_path)
    }
}