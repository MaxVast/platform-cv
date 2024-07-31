use actix_files::Files;
use actix_web::web;
use log::info;

use crate::controller::{*};
use crate::controller::back_office::back_office_controller;
//Config server
pub fn config_services(conf: &mut web::ServiceConfig) {
    info!("Configuring routes...");
    conf.service(
        web::scope("/admin")
            .service(
                web::resource("/login")
                    .route(web::post().to(back_office::back_office_controller::post_login))
                    .route(web::get().to(back_office::back_office_controller::get_login))
            )
            .service(
                web::resource("/").route(web::get().to(back_office_controller::homepage))
            )
            .service(
                web::resource("/logout").route(web::post().to(back_office_controller::logout))
            )
            /*.service(
                web::resource("/signup").route(web::post().to(back_office_controller::post_signup)),
                web::resource("/signup").route(web::get().to(back_office_controller::get_signup)),
            )

            .service(
                web::resource("/me").route(web::get().to(back_office_controller::me)),
            )*/
    )
    .service(web::resource("/health-check").route(web::get().to(front_controller::health_check)))
    .service(web::resource("/").route(web::get().to(front_controller::homepage)))
    .service(Files::new("/uploads", "uploads").show_files_listing())
    .service(Files::new("/assets", "assets").show_files_listing())
    .default_service(web::to(front_controller::handler_404));
}
