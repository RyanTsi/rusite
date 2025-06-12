use std::error::Error;

use actix_web::web;

use crate::config::ARTICLIE_SAVE_PATH;
use crate::models::params::AidParams;
use crate::models::requests::{ArticleCreateRequest, ArticleModifyRequest};
use crate::dao::database::Database;
use crate::models::struction::ArticleInfo;
use crate::utils::read_file;

pub async fn create_service(
    req: &web::Json<ArticleCreateRequest>,
    db:  &Database
) -> Result<(), Box<dyn Error>> {
    db.create_article(&req.title, &req.tags, &req.categories, &req.summary, &req.content, &req.secret).await?;
    Ok(())
}

pub async fn modify_service(
    req: &web::Json<ArticleModifyRequest>,
    db:  &Database
) -> Result<(), Box<dyn Error>> {
    db.modify_article(&req.aid, &req.title, &req.tags, &req.categories, &req.summary, &req.content, &req.secret).await?;
    Ok(())
}

pub async fn delete_service(
    req: &web::Path<AidParams>,
    db:  &Database
) -> Result<(), Box<dyn Error>> {
    db.delete_article(&req.aid).await?;
    Ok(())
}

pub async fn list_service(
    db:  &Database
) -> Result<Vec<ArticleInfo>, Box<dyn Error>> {
    db.get_articleinfo_list().await
}

pub async fn content_path_service(
    req: &web::Path<AidParams>,
    db:  &Database
) -> Result<String, Box<dyn Error>> {
    db.get_content_path(&req.aid).await
}

pub async fn content_service(
    req: &web::Path<AidParams>,
    db:  &Database
) -> Result<String, Box<dyn Error>> {
    let file_path = ARTICLIE_SAVE_PATH.to_string() + &db.get_content_path(&req.aid).await?;
    read_file(&file_path).await
}
