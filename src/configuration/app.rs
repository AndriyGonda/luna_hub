use actix_cors::Cors;
use actix_web::{http, web};

pub fn cors_configuration() -> Cors {
    let cors = Cors::default()
        .allow_any_origin()
        .allow_any_method()
        .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
        .allowed_header(http::header::CONTENT_TYPE);
    cors
}

pub fn configure(cfg: &mut web::ServiceConfig) {}
