use actix_web::{HttpResponse, Responder};
use log;

use crate::utils::{get_cpu_usage, get_memory_usage};
#[utoipa::path(
    get,
    path = "/ping",
    responses(
        (status = 200, description = "pong", body = String),
    ),
)]
pub async fn ping() -> impl Responder {
    log::info!("->> {:<12}", "ping");
    match (get_cpu_usage().await, get_memory_usage().await) {
        (Ok(cpu_usage), Ok((used_mem, total_mem))) => {
            let response = format!(
                "Online\nCPU Usage: {:.2}%\nMemory Usage: {} KB / {} KB ({:.2} %)",
                cpu_usage, used_mem, total_mem, (used_mem as f64 / total_mem as f64 * 100.0)
            );
            HttpResponse::Ok().body(response)
        }
        _ => {
            HttpResponse::Ok().body("Offline")
        }
    }
}
