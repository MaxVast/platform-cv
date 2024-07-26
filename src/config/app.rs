use actix_files::Files;
use actix_web::{web};
use log::info;

use crate::controller::*;
//Config server
pub fn config_services(conf: &mut web::ServiceConfig) {
    info!("Configuring routes...");
    conf.service(
        web::resource("/health-check").route(web::get().to(front_controller::health_check)),
    )
    .service(web::resource("/").route(web::get().to(front_controller::homepage)))
    .service(Files::new("/uploads", "uploads").show_files_listing())
    .default_service(web::to(front_controller::handler_404));
}
