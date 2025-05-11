use actix_web::{web, HttpResponse, Responder};
use serde_json::json;

use crate::dao::database::Database;
use crate::model::request::{PostCreateRequest, PostModifyRequest};
use crate::services::post::create_service;

#[utoipa::path(
    post,
    context_path = "/api/v1",
    path = "/post/create",
    operation_id = "post create",
    request_body = PostCreateRequest,
    responses(
        (status = 200, description = "Success"),
    ),
    tag = "post create",
)]
pub async fn create(
    req: web::Json<PostCreateRequest>,
    data: web::Data<Database>,
) -> impl Responder {
    log::info!("->> {:<12}", "create post");
    match create_service(&req, &data).await {
        Ok(_) => HttpResponse::Ok().json(json!({
            "code": 200,
            "message": "Success",
        })),
        Err(e) => HttpResponse::InternalServerError().json(json!({
            "code": 500,
            "message": e.to_string(),
        })),
    }
}

#[utoipa::path(
    put,
    context_path = "/api/v1",
    path = "/post/modify",
    operation_id = "post modify",
    request_body = PostModifyRequest,
    responses(
        (status = 200, description = "Success"),
    ),
    tag = "post modify",
)]
pub async fn modify(
    req: web::Json<PostModifyRequest>,
    data: web::Data<Database>,
) -> impl Responder {
    log::info!("->> {:<12}", "modify post");
    HttpResponse::Ok().json(json!({
        "code": 200,
        "message": "Success",
    }))
}