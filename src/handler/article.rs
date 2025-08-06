use actix_web::{web, HttpResponse, Responder};
use serde_json::json;

use crate::dao::database::Database;
use crate::models::params::AidParams;
use crate::models::requests::{ArticleCreateRequest, ArticleModifyRequest};
use crate::services::article::*;

#[utoipa::path(
    post,
    context_path = "/api/v1",
    path = "/article/create",
    operation_id = "article create",
    request_body = ArticleCreateRequest,
    responses(
        (status = 200, description = "Success"),
    ),
    tag = "article create",
)]
pub async fn create(
    req: web::Json<ArticleCreateRequest>,
    data: web::Data<Database>,
) -> impl Responder {
    log::info!("->> {:<12}", "create article");
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
    delete,
    context_path = "/api/v1",
    path = "/article/{aid}/delete",
    operation_id = "article delete",
    params(AidParams),
    responses(
        (status = 200, description = "Success"),
    ),
    tag = "article delete",
)]
pub async fn delete(
    req : web::Path<AidParams>,
    data: web::Data<Database>,
) -> impl Responder {
    log::info!("->> {:<12}", "delete article");
    match delete_service(&req, &data).await {
        Ok(_) => HttpResponse::Ok().json(json!({
            "code": 200,
            "message": "Success",
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
    path = "/article/modify",
    operation_id = "article modify",
    request_body = ArticleModifyRequest,
    responses(
        (status = 200, description = "Success"),
    ),
    tag = "article modify",
)]
pub async fn modify(
    req: web::Json<ArticleModifyRequest>,
    data: web::Data<Database>,
) -> impl Responder {
    log::info!("->> {:<12}", "modify article");
    match modify_service(&req, &data).await {
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
    get,
    context_path = "/api/v1",
    path = "/article/list",
    operation_id = "article list",
    responses(
        (status = 200, description = "Success"),
    ),
    tag = "article list",
)]
pub async fn list(
    data: web::Data<Database>,
) -> impl Responder { 
    log::info!("->> {:<12}", "list articles");
    match list_service(&data).await {
        Ok(list) => HttpResponse::Ok().json(json!({
            "code": 200,
            "message": "Success",
            "data": list,
        })),
        Err(e) => HttpResponse::InternalServerError().json(json!({
            "code": 500,
            "message": e.to_string(),
        })),
    }
}

#[utoipa::path(
    get,
    context_path = "/api/v1",
    path = "/article/{aid}/content/path",
    operation_id = "article content path",
    params(AidParams),
    responses(
        (status = 200, description = "Success"),
    ),
    tag = "article content path",
)]
pub async fn content_path(
    req: web::Path<AidParams>,
    data: web::Data<Database>
) -> impl Responder {
    log::info!("->> {:<12}", "get content path");
    match content_path_service(&req, &data).await {
        Ok(path) => HttpResponse::Ok().json(json!({
            "code": 200,
            "message": "Success",
            "data": path,
        })),
        Err(e) => HttpResponse::InternalServerError().json(json!({
            "code": 500,
            "message": e.to_string(),
        })),
    }
}


#[utoipa::path(
    get,
    context_path = "/api/v1",
    path = "/article/{aid}/content",
    operation_id = "article content",
    params(AidParams),
    responses(
        (status = 200, description = "Success"),
    ),
    tag = "article content",
)]
pub async fn content(
    req: web::Path<AidParams>,
    data: web::Data<Database>
) -> impl Responder {
    log::info!("->> {:<12}", "get content");
    match content_service(&req, &data).await {
        Ok(content) => HttpResponse::Ok().json(json!({
            "code": 200,
            "message": "Success",
            "data": content,
        })),
        Err(e) => HttpResponse::InternalServerError().json(json!({
            "code": 500,
            "message": e.to_string(),
        })),
    }
}

#[utoipa::path(
    get,
    context_path = "/api/v1",
    path = "/article/tags",
    operation_id = "article tags",
    responses(
        (status = 200, description = "Success"),
    ),
    tag = "article tags",
)]
pub async fn tags(
    data: web::Data<Database>
) -> impl Responder {
    log::info!("->> {:<12}", "article tags");
    match tags_service(&data).await {
        Ok(tags) => HttpResponse::Ok().json(json!({
            "code": 200,
            "message": "Success",
            "data": tags,
        })),
        Err(e) => HttpResponse::InternalServerError().json(json!({
            "code": 500,
            "message": e.to_string(),
        })),
    }
}

#[utoipa::path(
    get,
    context_path = "/api/v1",
    path = "/article/categories",
    operation_id = "article categories",
    responses(
        (status = 200, description = "Success"),
    ),
    tag = "article categories",
)]
pub async fn categories(
    data: web::Data<Database>
) -> impl Responder {
    log::info!("->> {:<12}", "article categories");
    match categories_service(&data).await {
        Ok(categories) => HttpResponse::Ok().json(json!({
            "code": 200,
            "message": "Success",
            "data": categories,
        })),
        Err(e) => HttpResponse::InternalServerError().json(json!({
            "code": 500,
            "message": e.to_string(),
        })),
    }
}