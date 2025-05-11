use actix_web::{web, HttpResponse, Responder};
use serde_json::json;
use crate::model::request::UserCreateRequest;
use crate::dao::database::Database;
use crate::services::user::create_service;

#[utoipa::path(
    post,
    context_path = "/api/v1",
    path = "/user/create",
    operation_id = "user create",
    request_body = UserCreateRequest,
    responses(
        (status = 200, description = "Success"),
    ),
    tag = "user create",
)]
pub async fn create(
    req: web::Json<UserCreateRequest>,
    data: web::Data<Database>,
) -> impl Responder {
    log::info!("->> {:<12}", "create user");
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