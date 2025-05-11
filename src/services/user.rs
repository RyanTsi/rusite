use std::error::Error;

use actix_web::web;

use crate::model::request::UserCreateRequest;
use crate::dao::database::Database;

pub async fn create_service(
    req: &web::Json<UserCreateRequest>,
    db:  &Database
) -> Result<(), Box<dyn Error>> {
    match db.user_exists(&req.username).await {
        Ok(exists) => {
            if exists {
                return Err("This user already exists".into());
            } else {
                db.create_user(&req.username, &req.password, &req.email).await?;
                return Ok(());
            }
        },
        Err(e) => {
            return Err(e);
        }
    }
}