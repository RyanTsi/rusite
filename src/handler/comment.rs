use actix_web::{web, HttpResponse, Responder};
use serde_json::json;

use crate::dao::database::Database;
use crate::models::params::{CidParams, AidParams};
use crate::models::requests::{CommentCreateRequest, CommentModifyRequest};
use crate::services::comment::*;

#[utoipa::path(
    post,
    context_path = "/api/v1",
    path = "/comment/create",
    operation_id = "comment create",
    request_body = CommentCreateRequest,
    responses(
        (status = 200, description = "Success"),
    ),
    tag = "comment create",
)]
pub async fn create(
    req: web::Json<CommentCreateRequest>,
    data: web::Data<Database>,
) -> impl Responder{
    log::info!("->> {:<12}", "create comment");
    match create_service(&req,&data).await {
        Ok(_) => HttpResponse::Ok().json(json!({
            "code": 200,
            "message": "success",
        })),
        Err(e) => HttpResponse::InternalServerError().json(json!({
            "code": 500,
            "message": e.to_string(),
        }))
    }
}

#[utoipa::path(
    delete,
    context_path = "/api/v1",
    path = "/comment/{cid}/delete",
    operation_id = "comment delete",
    params(CidParams),
    responses(
        (status = 200, description = "Success"),
    ),
    tag = "comment delete",
)]
pub async fn delete(
    req: web::Path<CidParams>,
    data: web::Data<Database>,
) -> impl Responder{
    log::info!("->> {:<12}", "delete comment");
    match delete_service(&req, &data).await {
        Ok(_) => HttpResponse::Ok().json(json!({
            "code": 200,
            "message": "success",
        })),
        Err(e) => HttpResponse::InternalServerError().json(json!({
            "code": 500,
            "message": e.to_string(),
        }))
    }
}

#[utoipa::path(
    put,
    context_path = "/api/v1",
    path = "/comment/modify",
    request_body = CommentModifyRequest,
    responses(
        (status = 200, description = "Success"),
    ),
    tag = "comment modfiy",
)]
pub async fn modify(
    req: web::Json<CommentModifyRequest>,
    data: web::Data<Database>,
) -> impl Responder{
    log::info!("->> {:<12}", "modify comment");
    match modify_service(&req,&data).await {
        Ok(_) => HttpResponse::Ok().json(json!({
            "code": 200,
            "message": "success",
        })),
        Err(e) => HttpResponse::InternalServerError().json(json!({
            "code": 500,
            "message": e.to_string(),
        }))
    }
}

#[utoipa::path(
    get,
    context_path = "/api/v1",
    path = "/article/{aid}/comment",
    params(AidParams),
    operation_id = "comment list",
    responses(
        (status = 200, description = "success"),
    ),
    tag = "comment list",
)]
pub async fn list(
    req: web::Path<AidParams>,
    data: web::Data<Database>,
) -> impl Responder{
    log::info!("->> {:<12}", "list comment");
    match list_service(&req,&data).await {
        Ok(comments) => HttpResponse::Ok().json(json!({
            "code": 200,
            "message": "success",
            "data": comments,
        })),
        Err(e) => HttpResponse::InternalServerError().json(json!({
            "code": 500,
            "message": e.to_string(),
        }))
    }
}