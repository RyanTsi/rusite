use std::env;

use sqlx::mysql::MySqlPool;

#[derive(Clone)]
pub struct Database {
    pub pool: MySqlPool,
}

impl Database {
    /// 创建一个新的数据库连接池实例。
    ///
    /// 该函数从环境变量中读取数据库URL，并异步连接到数据库。
    /// 如果环境变量DATABASE_URL未定义或连接失败，程序将panic。
    ///
    /// # Returns
    ///
    /// 返回一个新的数据库连接池实例。
    pub async fn new() -> Self {
        let url = env::var("DATABASE_URL").expect("DATABASE_URL is not defined");
        Self {
            pool: MySqlPool::connect(&url).await.expect("Failed to connect to database"),
        }
    }
}