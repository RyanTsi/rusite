use actix_cors::Cors;
use actix_web::http::header;


pub fn CORS() -> Cors {
    Cors::default()
        .allow_any_origin()
        .allowed_methods(vec!["GET", "POST"])
        .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
        .allowed_header(header::CONTENT_TYPE)
        .supports_credentials()
        .max_age(3600)
}