use std::error::Error;
use crate::dao::database;
use sqlx::Row;
use crate::models::struction::Comment;

impl database::Database {
    pub async fn comment_exists(
        &self,
        cid: &str
    ) -> Result<bool, Box<dyn Error>> {
        let (count,):(i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM comments WHERE cid = ?",
        )
        .bind(cid)
        .fetch_one(&self.pool)
        .await?;
        Ok(count == 1)
    }
    pub async fn create_comment(
        &self,
        uid: &str,
        aid: &str,
        content: &str
    ) -> Result<(), Box<dyn Error>> {
        if !self.article_exists(aid).await? {
            return Err("article not exists".into());
        }
        if !self.user_exists(uid).await? {
            return Err("user not exists".into());
        }
        sqlx::query(
            "INSERT INTO comments (aid, uid, content) VALUES (?, ?, ?)",
        )
        .bind(aid)
        .bind(uid)
        .bind(content)
        .execute(&self.pool)
        .await?;
        Ok(())
    }
    pub async fn delete_comment(
        &self,
        cid: &str
    ) -> Result<(), Box<dyn Error>> {
        sqlx::query(
            "DELETE FROM comments WHERE cid = ?",
        )
        .bind(cid)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn modify_comment(
        &self,
        cid: &str,
        content: &str
    ) -> Result<(), Box<dyn Error>> {
        if !self.comment_exists(cid).await? {
            return Err("comment not exists".into());
        }
        sqlx::query(
            "UPDATE comments SET content = ? WHERE cid = ?",
        )
        .bind(content)
        .bind(cid)
        .execute(&self.pool)
        .await?;
        Ok(())
    }
    pub async fn list_comment(
        &self,
        aid: &str
    ) -> Result<Vec<Comment>, Box<dyn Error>> {
        let comment_list = sqlx::query(
            "
            SELECT cid, uid, content, created_at, updated_at FROM comments WHERE aid = ?
            "
        )
        .bind(aid)
        .map(|raw: sqlx::mysql::MySqlRow| {
            Comment {
                cid: raw.get("cid"),
                uid: raw.get("uid"),
                content: raw.get("content"),
                created_at: raw.get("created_at"),
                updated_at: raw.get("updated_at"),
            }
        })
        .fetch_all(&self.pool)
        .await?;
        Ok(comment_list)
    }
}