use actix_web::HttpResponse;
use askama::Template;

use crate::templates::front_template::*;

// GET HOMEPAGE
pub async fn homepage() -> HttpResponse {
    let template = HomeTemplate {};
    let response_body = template.render().unwrap();
    HttpResponse::Ok()
        .content_type("text/html")
        .body(response_body)
}

// GET HEALTH CHECK
pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().body("Health check OK")
}

// fallback route
pub async fn handler_404() -> HttpResponse {
    HttpResponse::NotFound().body("404 : Nothing here..")
}
