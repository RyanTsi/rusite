use std::{collections::HashMap, error::Error};
use crate::dao::database;
use sqlx::Row;
use crate::config::POST_PATH;
use crate::utils;

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

    pub async fn create_post(
        &self,
        title: &str,
        tags: &Vec<String>,
        categories: &Vec<String>,
        summary: &str,
        content: &str,
        secret: &Option<String>
    ) -> Result<(), Box<dyn Error>> {
        let mut tagsmap = Vec::new();
        let mut categoriesmap = Vec::new();
        for tag in tags {
            tagsmap.push((self.create_tag(tag).await?, tag));
        }
        for category in categories {
            self.create_category(category).await?;
            categoriesmap.push((self.create_category(category).await?, category));
        }
        let current = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        let content_filepath = POST_PATH.to_string() + &current + "_" + title + ".md";
        utils::write_file(&content_filepath, content).await?;
        sqlx::query(
            "
            INSERT INTO posts (title, summary, content, secret, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?)
            ",
        )
        .bind(title)
        .bind(summary)
        .bind(content_filepath)
        .bind(secret)
        .bind(&current)
        .bind(&current)
        .execute(&self.pool)
        .await?;

        let pid = sqlx::query("SELECT pid FROM posts WHERE created_at = ?")
        .bind(current)
        .fetch_one(&self.pool)
        .await?
        .get::<String, _>("pid");
        for (tid, _) in tagsmap {
            sqlx::query(
                "INSERT INTO post_tags (pid, tid) VALUES (?, ?)",
            )
            .bind(&pid)
            .bind(tid)
            .execute(&self.pool)
            .await?;
        }
        for (cid, _) in categoriesmap {
            sqlx::query(
                "INSERT INTO post_categories (pid, cid) VALUES (?, ?)",
            )
            .bind(&pid)
            .bind(cid)
            .execute(&self.pool)
            .await?;
        }
        Ok(())
    }

    pub async fn create_tag(
        &self,
        tag: &str
    ) -> Result<i32, Box<dyn Error>> {
        sqlx::query(
            "
            INSERT INTO tags (name) VALUES (?)
            ON DUPLICATE KEY UPDATE name = name
            ",
        )
        .bind(tag)
        .execute(&self.pool)
        .await?;
        let tid = sqlx::query(
            "SELECT tid FROM tags WHERE name = ?",
        )
        .bind(tag)
        .fetch_one(&self.pool)
        .await?
        .get::<i32, _>("tid");
        Ok(tid)
    }

    pub async fn create_category(
        &self,
        category: &str
    ) -> Result<i32, Box<dyn Error>> {
        sqlx::query(
            "
            INSERT INTO categories (name) VALUES (?)
            ON DUPLICATE KEY UPDATE name = name
            ",
        )
        .bind(category)
        .execute(&self.pool)
        .await?;
        let cid = sqlx::query(
            "SELECT cid FROM categories WHERE name = ?",
        )
        .bind(category)
        .fetch_one(&self.pool)
        .await?
        .get::<i32, _>("cid");
        Ok(cid)
    }

    pub async fn get_tags(
        &self
    ) -> Result<HashMap<String, i32>, Box<dyn Error>> {
        let tags = sqlx::query(
            "SELECT * FROM tags"
        )
        .map(|row: sqlx::mysql::MySqlRow| {
            let name = row.get::<String, _>("name");
            let tid  = row.get::<i32, _>("tid");
            (name, tid)
        })
        .fetch_all(&self.pool)
        .await?;
        Ok(tags.into_iter().collect())
    }

    pub async fn get_categories(
        &self
    ) -> Result<HashMap<String, i32>, Box<dyn Error>> {
        let categories = sqlx::query(
            "SELECT * FROM categories"
        )
        .map(|row: sqlx::mysql::MySqlRow| {
            let name = row.get::<String, _>("name");
            let cid  = row.get::<i32, _>("cid");
            (name, cid)
        })
        .fetch_all(&self.pool)
        .await?;
        Ok(categories.into_iter().collect())
    }
}