use std::{collections::HashMap, error::Error};
use crate::dao::database;
use crate::models::struction::{ArticleInfo, Tag, Category};
use sqlx::Row;
use crate::utils::{self, current_time, str_split_to_vec, write_file, delete_file};

impl database::Database {

    pub async fn create_article(
        &self,
        title: &str,
        tags: &Vec<String>,
        categories: &Vec<String>,
        summary: &str,
        content: &str,
        secret: &Option<String>
    ) -> Result<(), Box<dyn Error>> {
        let tagsmap = self.create_tagsmap(tags).await?;
        let categoriesmap = self.create_categoriesmap(categories).await?;
        let current = current_time();
        let file_name = format!("{}_{}.md", &current, &title);
        let save_path = &self.articlies_save_path().join(&file_name);
        utils::write_file(&save_path, content).await?;
        sqlx::query(
            "
            INSERT INTO articles (title, summary, content, secret, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?)
            ",
        )
        .bind(title)
        .bind(summary)
        .bind(file_name)
        .bind(secret)
        .bind(&current)
        .bind(&current)
        .execute(&self.pool)
        .await?;

        let aid = sqlx::query("SELECT aid FROM articles WHERE created_at = ?")
        .bind(current)
        .fetch_one(&self.pool)
        .await?
        .get::<String, _>("aid");

        self.create_article_tags(&aid, &tagsmap).await?;
        self.create_article_categories(&aid, &categoriesmap).await?;
        Ok(())
    }

    pub async fn delete_article(
        &self,
        aid: &str
    ) -> Result<(), Box<dyn Error>> {
        let save_path = &self.articlies_save_path().join(&self.get_content_path(aid).await?.as_str());
        delete_file(&save_path).await?;
        sqlx::query("DELETE FROM articles WHERE aid = ?")
        .bind(aid)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn modify_article(
        &self,
        aid: &str,
        title: Option<&str>,
        tags: Option<&Vec<String>>,
        categories: Option<&Vec<String>>,
        summary: Option<&str>,
        content: Option<&str>,
        secret: Option<&str>
    ) -> Result<(), Box<dyn Error>> {
        if let Some(tags) = tags {
            self.modify_article_tags(aid, tags).await?;
        }
        if let Some(categories) = categories {
            self.modify_article_categories(aid, categories).await?;
        }
        sqlx::query(
            "
            UPDATE articles SET 
                title = COALESCE(?, title),
                summary = COALESCE(?, summary),
                secret = COALESCE(?, secret),
                updated_at = ?
            WHERE aid = ?
            "
        )
        .bind(title)
        .bind(summary)
        .bind(secret)
        .bind(current_time())
        .bind(aid)
        .execute(&self.pool)
        .await?;
        if let Some(content) = content {
            let save_path = &self.articlies_save_path().join(&self.get_content_path(aid).await?.as_str());
            write_file(&save_path, content).await?;
        }
        Ok(())
    }

    pub async fn article_exists(
        &self,
        aid: &str
    ) -> Result<bool, Box<dyn Error>> {
        let (count,):(i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM articles WHERE aid = ?",
        )
        .bind(aid)
        .fetch_one(&self.pool)
        .await?;
        Ok(count == 1)
    }

    pub async fn get_content_path(
        &self,
        aid: &str
    ) -> Result<String, Box<dyn Error>> {
        let contentpath = sqlx::query(
            "
            SELECT content FROM articles WHERE aid = ?
            "
        )
        .bind(aid)
        .fetch_one(&self.pool)
        .await?
        .get::<String, _>("content");

        Ok(contentpath)
    }

    pub async fn get_articleinfo_list(
        &self,
    ) -> Result<Vec<ArticleInfo>, Box<dyn Error>> {
        let articleinfo_list = sqlx::query(
            "
        SELECT 
            a.aid,
            a.title,
            a.summary,
            a.content,
            a.secret,
            a.created_at,
            a.updated_at,
            GROUP_CONCAT(DISTINCT t.name ORDER BY t.name SEPARATOR ', ') AS tags,
            GROUP_CONCAT(DISTINCT c.name ORDER BY c.name SEPARATOR ', ') AS categories
        FROM 
            articles a
        LEFT JOIN 
            article_tags at ON a.aid = at.aid
        LEFT JOIN 
            tags t ON at.tid = t.tid
        LEFT JOIN 
            article_categories ac ON a.aid = ac.aid
        LEFT JOIN 
            categories c ON ac.cid = c.cid
        GROUP BY 
            a.aid, a.title, a.summary, a.content, a.secret, a.created_at, a.updated_at
        ORDER BY 
            a.updated_at DESC;
            "
        ).map(|raw: sqlx::mysql::MySqlRow| {
            ArticleInfo {
                aid: raw.get("aid"),
                title: raw.get("title"),
                tags: str_split_to_vec(&raw.get::<String, _>("tags"), ","),
                categories: str_split_to_vec(&raw.get::<String, _>("categories"), ","),
                summary: raw.get("summary"),
                secret: raw.get("secret"),
                created_at: raw.get("created_at"),
                updated_at: raw.get("updated_at"),
            }
        })
        .fetch_all(&self.pool)
        .await?;
        Ok(articleinfo_list)
    }

    pub async fn get_tagsmap(
        &self,
        tags: &Vec<String>
    ) -> Result<Vec<(i32, String)>, Box<dyn Error>> {
        let mut tagsmap = Vec::new();
        for tag in tags {
            tagsmap.push((self.get_tag(tag).await?, tag.clone()));
        }
        Ok(tagsmap)
    }

    pub async fn get_categoriesmap(
        &self,
        categories: &Vec<String>
    ) -> Result<Vec<(i32, String)>, Box<dyn Error>> {
        let mut categoriesmap = Vec::new();
        for category in categories {
            categoriesmap.push((self.get_category(category).await?, category.clone()));
        }
        Ok(categoriesmap)
    }

    pub async fn create_tagsmap(
        &self,
        tags: &Vec<String>
    ) -> Result<Vec<(i32, String)>, Box<dyn Error>> {
        let mut tagsmap = Vec::new();
        for tag in tags {
            tagsmap.push((self.create_tag(tag).await?, tag.clone()));
        }
        Ok(tagsmap)
    }

    pub async fn create_categoriesmap(
        &self,
        categories: &Vec<String>
    ) -> Result<Vec<(i32, String)>, Box<dyn Error>> {
        let mut categoriesmap = Vec::new();
        for category in categories {
            categoriesmap.push((self.create_category(category).await?, category.clone()));
        }
        Ok(categoriesmap)
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

    pub async fn get_tag(
        &self,
        tag: &str
    ) -> Result<i32, Box<dyn Error>> {
        let tid = sqlx::query(
            "SELECT tid FROM tags WHERE name = ?",
        )
        .bind(tag)
        .fetch_one(&self.pool)
        .await?
        .get::<i32, _>("tid");
        Ok(tid)
    }
 
    pub async fn get_category(
        &self,
        category: &str
    ) -> Result<i32, Box<dyn Error>> {
        let cid = sqlx::query(
            "SELECT cid FROM categories WHERE name = ?",
        )
        .bind(category)
        .fetch_one(&self.pool)
        .await?
        .get::<i32, _>("cid");
        Ok(cid)
    }

    pub async fn get_tags_list(
        &self
    ) -> Result<Vec<Tag>, Box<dyn Error>> {
        let tags = sqlx::query(
            "
            SELECT t.name AS tag_name, COUNT(at.tid) AS count FROM tags t
            JOIN article_tags at ON t.tid = at.tid
            GROUP BY t.tid 
            ORDER BY count DESC
            "
        )
        .map(|row: sqlx::mysql::MySqlRow| {
            let name = row.get::<String, _>("tag_name");
            let count  = row.get::<i32, _>("count");
            Tag {
                name,
                count: count as usize,
            }
        })
        .fetch_all(&self.pool)
        .await?;
        Ok(tags)
    }

    pub async fn get_categories_list(
        &self
    ) -> Result<Vec<Category>, Box<dyn Error>> {
        let categories = sqlx::query(
            "
            SELECT c.name AS category_name, COUNT(ac.cid) AS count FROM categories c
            JOIN article_categories ac ON c.cid = ac.cid
            GROUP BY c.cid 
            ORDER BY count DESC
            "
        )
        .map(|row: sqlx::mysql::MySqlRow| {
            let name = row.get::<String, _>("category_name");
            let count  = row.get::<i32, _>("count");
            Category {
                name,
                count: count as usize,
            }
        })
        .fetch_all(&self.pool)
        .await?;
        Ok(categories)
    }

    async fn del_article_tags(
        &self,
        aid: &str
    ) -> Result<(), Box<dyn Error>> {
        sqlx::query(
            "DELETE FROM article_tags WHERE aid = ?",
        )
        .bind(aid)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn del_article_categories(
        &self,
        aid: &str
    ) -> Result<(), Box<dyn Error>> {
        sqlx::query(
            "DELETE FROM article_categories WHERE aid = ?",
        )
        .bind(aid)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn create_article_tags(
        &self,
        aid: &str,
        tagsmap: &Vec<(i32, String)>
    ) -> Result<(), Box<dyn Error>> {
        for (tid, _) in tagsmap {
            sqlx::query(
                "INSERT INTO article_tags (aid, tid) VALUES (?, ?)",
            )
            .bind(&aid)
            .bind(tid)
            .execute(&self.pool)
            .await?;
        }
        Ok(())
    }

    async fn create_article_categories(
        &self,
        aid: &str,
        categoriesmap: &Vec<(i32, String)>
    ) -> Result<(), Box<dyn Error>> {
        for (cid, _) in categoriesmap {
            sqlx::query(
                "INSERT INTO article_categories (aid, cid) VALUES (?, ?)",
            )
            .bind(&aid)
            .bind(cid)
            .execute(&self.pool)
            .await?;
        }
        Ok(())
    }

    async fn modify_article_tags(
        &self,
        aid: &str,
        tags: &Vec<String>
    ) -> Result<(), Box<dyn Error>> {
        for tag in tags {
            self.create_tag(tag).await?;
        }
        self.del_article_tags(aid).await?;
        let tagsmap = self.get_tagsmap(tags).await?;
        self.create_article_tags(aid, &tagsmap).await?;
        Ok(())
    }

    async fn modify_article_categories(
        &self,
        aid: &str,
        categories: &Vec<String>
    ) -> Result<(), Box<dyn Error>> {
        for category in categories {
            self.create_category(category).await?;
        }
        self.del_article_categories(aid).await?;
        let categoriesmap = self.get_categoriesmap(categories).await?;
        self.create_article_categories(aid, &categoriesmap).await?;
        Ok(())
    }

}