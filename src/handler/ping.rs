use actix_web::Responder;
use log;
#[utoipa::path(
    get,
    path = "/ping",
    responses(
        (status = 200, description = "pong", body = String),
    ),
)]
pub async fn ping() -> impl Responder {
    log::info!("->> {:<12}", "ping");
    "pong"
}