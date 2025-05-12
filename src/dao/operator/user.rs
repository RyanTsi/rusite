use std::error::Error;
use crate::dao::database;

impl database::Database {
    pub async fn user_exists(
        &self,
        username: &str
    ) -> Result<bool, Box<dyn Error>> {
        let (count,):(i32,) = sqlx::query_as(
            "SELECT COUNT(*) FROM users WHERE username = ?",
        )
        .bind(username)
        .fetch_one(&self.pool)
        .await?;
        Ok(count == 1)
    }
    pub async fn create_user(
        &self,
        username: &str,
        password: &str,
        email: &Option<String>
    ) -> Result<(), Box<dyn Error>> {
        sqlx::query(
            "INSERT INTO users (username, password, email) VALUES (?, ?, ?)",
        )
        .bind(username)
        .bind(password)
        .bind(email)
        .execute(&self.pool)
        .await?;
        Ok(())
    }
}