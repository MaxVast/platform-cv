use actix_web::{http::header::SET_COOKIE, web, HttpRequest, HttpResponse};
use askama::Template;

use crate::{
    config::db::Pool,
    constants,
    models::{
        user::{LoginDTO, User},
        user_token::UserToken,
    },
    templates::back_office_template::*,
    utils::token_utils,
};

/*pub async fn signup(
    user_dto: web::Json<UserDTO>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {
    match account_service::signup(user_dto.0, &pool) {
        Ok(message) => Ok(HttpResponse::Ok().json(ResponseBody::new(&message, constants::EMPTY))),
        Err(err) => Err(err),
    }
}*/

pub async fn homepage() -> HttpResponse {
    let template = HomepageBackOfficeTemplate {};
    let response_body = template.render().unwrap();
    HttpResponse::Ok()
        .content_type("text/html")
        .body(response_body)
}

pub async fn get_login() -> HttpResponse {
    let template = LoginBackOfficeTemplate {};
    let response_body = template.render().unwrap();
    HttpResponse::Ok()
        .content_type("text/html")
        .body(response_body)
}

// POST admin/login
pub async fn post_login(payload: web::Json<LoginDTO>, pool: web::Data<Pool>) -> HttpResponse {
    let login_dto: LoginDTO = payload.into_inner();

    match User::login(login_dto, &mut pool.get().unwrap()) {
        Some(logged_user) => {
            let jwt_token = UserToken::generate_token(&logged_user);
            HttpResponse::Found()
                .append_header((SET_COOKIE, format!("token={}; Path=/; HttpOnly", jwt_token)))
                .append_header(("Location", "/admin/"))
                .finish()
        }
        None => HttpResponse::Unauthorized().body(constants::MESSAGE_USER_NOT_FOUND.to_string()),
    }
}

pub async fn logout(req: HttpRequest, pool: web::Data<Pool>) -> HttpResponse {
    if let Some(token) = req.cookie("token") {
        if let Ok(token_data) = token_utils::decode_token(token.value().to_string()) {
            if let Ok(username) = token_utils::verify_token(&token_data, &pool) {
                if let Ok(user) = User::find_user_by_username(&username, &mut pool.get().unwrap()) {
                    println!("id user : {:?}", user.id);
                    User::logout(user.id, &mut pool.get().unwrap());
                    return HttpResponse::Found()
                        .append_header((SET_COOKIE, "token=; Max-Age=0; Path=/; HttpOnly"))
                        .append_header(("Location", "/"))
                        .finish();
                }
            }
        }

        HttpResponse::InternalServerError().body(constants::MESSAGE_PROCESS_TOKEN_ERROR.to_string())
    } else {
        HttpResponse::BadRequest().body(constants::MESSAGE_TOKEN_MISSING.to_string())
    }
}
