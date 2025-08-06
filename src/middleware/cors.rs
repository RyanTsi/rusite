use actix_cors::Cors;
use actix_web::http::{self, header};


pub fn CORS() -> Cors {
    Cors::default()
    .allowed_origin("https://solarain.cloud")
    .allowed_origin("https://www.solarain.cloud")
    .allowed_origin("http://localhost:3000")
    .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
    .allowed_headers(vec![
        http::header::AUTHORIZATION,
        http::header::ACCEPT,
        http::header::CONTENT_TYPE,
    ])
    .supports_credentials()
    .max_age(3600)
}