use std::error::Error;

use actix_web::web;

use crate::models::params::{AidParams, CidParams};
use crate::models::requests::{CommentCreateRequest, CommentModifyRequest};
use crate::dao::database::Database;
use crate::models::struction::Comment;

pub async fn create_service(
    req: &web::Json<CommentCreateRequest>,
    db:  &Database
) -> Result<(), Box<dyn Error>> {
    db.create_comment(&req.uid, &req.aid, &req.content).await?;
    Ok(())
}

pub async fn delete_service(
    req: &web::Path<CidParams>,
    db:  &Database
) -> Result<(), Box<dyn Error>> {
    db.delete_comment(&req.cid).await?;
    Ok(())
}

pub async fn modify_service(
    req: &web::Json<CommentModifyRequest>,
    db:  &Database
) -> Result<(), Box<dyn Error>> {
    db.modify_comment(&req.cid, &req.content).await?;
    Ok(())
}

pub async fn list_service(
    req: &web::Path<AidParams>,
    db:  &Database
) -> Result<Vec<Comment>, Box<dyn Error>> {
    db.list_comment(&req.aid).await
}