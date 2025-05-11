use std::error::Error;

use actix_web::web;

use crate::model::request::PostCreateRequest;
use crate::dao::database::Database;

pub async fn create_service(
    req: &web::Json<PostCreateRequest>,
    db:  &Database
) -> Result<(), Box<dyn Error>> {
    db.create_post(&req.title, &req.tags, &req.categories, &req.summary, &req.content, &req.secret).await?;
    Ok(())
}