use actix_web::{web, HttpResponse, Responder};
use serde_json::json;
use crate::model::admin::UserCreateRequest;
use crate::dao::database::Database;


#[utoipa::path(
    post,
    context_path = "/api/v1",
    path = "/admin/user/create",
    request_body = UserCreateRequest,
    responses(
        (status = 200, description = "Success"),
    ),
    tag = "admin-user",
)]
pub async fn create(
    req: web::Json<UserCreateRequest>,
    data: web::Data<Database>,
) -> impl Responder {
    log::info!("->> {:<12}", "user.create");
    // match data.user_exists(req.username).await {
    //     Ok(true)
    // }
    HttpResponse::Ok().json(json!({
        "code": 0,
        "msg": "success",
        "data": {
            "username": req.username,
            "password": req.password,
        }
    }))
}