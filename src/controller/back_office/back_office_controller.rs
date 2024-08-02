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
use actix_web::{
    http::header::SET_COOKIE,
    web::{Data, Json},
    HttpRequest, HttpResponse,
};
use askama::Template;

/*pub async fn signup(
    user_dto: web::Json<UserDTO>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {
    match account_service::signup(user_dto.0, &pool) {
        Ok(message) => Ok(HttpResponse::Ok().json(ResponseBody::new(&message, constants::EMPTY))),
        Err(err) => Err(err),
    }
}*/

pub async fn homepage(req: HttpRequest) -> HttpResponse {
    if let Some(token) = req.cookie("token") {
        if let Ok(login_info_token) = token_utils::get_data_token_to_login_info(token) {
            let template = HomepageBackOfficeTemplate {
                username: login_info_token.username,
                company_name: login_info_token
                    .company
                    .clone()
                    .unwrap_or_else(|| "".to_string()),
                role: &login_info_token.role,
                login_session: &login_info_token.login_session,
            };
            let response_body = template.render().unwrap();

            return HttpResponse::Ok()
                .content_type("text/html")
                .body(response_body);
        }
        HttpResponse::InternalServerError().body(constants::MESSAGE_PROCESS_TOKEN_ERROR.to_string())
    } else {
        HttpResponse::BadRequest().body(constants::MESSAGE_TOKEN_MISSING.to_string())
    }
}

// GET & POST admin/login
pub async fn login(
    req: HttpRequest,
    payload: Option<Json<LoginDTO>>,
    pool: Data<Pool>,
) -> HttpResponse {
    match *req.method() {
        actix_web::http::Method::GET => {
            let template = LoginBackOfficeTemplate {};
            let response_body = template.render().unwrap();
            HttpResponse::Ok()
                .content_type("text/html")
                .body(response_body)
        }
        actix_web::http::Method::POST => {
            let login_dto: LoginDTO = payload.expect("REASON").into_inner();
            if User::find_user_by_username_or_email(
                &login_dto.username_or_email,
                &mut pool.get().unwrap(),
            ) {
                if let Some(logged_user) = User::login(login_dto, &mut pool.get().unwrap()) {
                    let jwt_token = UserToken::generate_token(&logged_user);
                    HttpResponse::Found()
                        .append_header((
                            SET_COOKIE,
                            format!("token={}; Path=/; HttpOnly", jwt_token),
                        ))
                        .append_header(("Location", "/admin/"))
                        .finish()
                } else {
                    HttpResponse::Unauthorized().body(constants::MESSAGE_LOGIN_FAILED.to_string())
                }
            } else {
                HttpResponse::Unauthorized().body(constants::MESSAGE_USER_NOT_FOUND.to_string())
            }
        }
        _ => HttpResponse::MethodNotAllowed().finish(),
    }
}

pub async fn logout(req: HttpRequest, pool: Data<Pool>) -> HttpResponse {
    if let Some(token) = req.cookie("token") {
        if let Ok(login_info_token) = token_utils::get_data_token_to_login_info(token) {
            if let Ok(user) =
                User::find_user_by_username(&login_info_token.username, &mut pool.get().unwrap())
            {
                println!("id user : {:?}", user.id);
                User::logout(user.id, &mut pool.get().unwrap());
                return HttpResponse::Found()
                    .append_header((SET_COOKIE, "token=; Max-Age=0; Path=/; HttpOnly"))
                    .append_header(("Location", "/"))
                    .finish();
            }
            HttpResponse::InternalServerError()
                .body(constants::MESSAGE_PROCESS_TOKEN_ERROR.to_string())
        } else {
            HttpResponse::InternalServerError().body(constants::MESSAGE_USER_NOT_FOUND.to_string())
        }
    } else {
        HttpResponse::BadRequest().body(constants::MESSAGE_TOKEN_MISSING.to_string())
    }
}
