use crate::{
    config::db::Pool,
    constants,
    models::{
        company::Company,
        user::{LoginDTO, User, UserDTO},
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

pub async fn signup(
    req: HttpRequest,
    payload: Option<Json<UserDTO>>,
    pool: Data<Pool>,
) -> HttpResponse {
    if let Some(token) = req.cookie("token") {
        if token_utils::get_role_superadmin(token.clone()) {
            if let Ok(login_info_token) = token_utils::get_data_token_to_login_info(token) {
                match *req.method() {
                    actix_web::http::Method::GET => {
                        let company_data = Company::find_all(&mut pool.get().unwrap());
                        let companies = company_data.unwrap_or_else(|_| Vec::new());
                        let template = AddUserBackOfficeTemplate {
                            list_company: &companies,
                            role: &login_info_token.role,
                        }
                        .render()
                        .unwrap();

                        return HttpResponse::Ok().content_type("text/html").body(template);
                    }
                    actix_web::http::Method::POST => {
                        let user_dto: UserDTO = payload.expect("REASON").into_inner();
                        match User::signup(user_dto, &mut pool.get().unwrap()) {
                            Ok(message) => HttpResponse::Created().body(message),
                            Err(message) => HttpResponse::BadRequest().body(message),
                        }
                    }
                    _ => HttpResponse::MethodNotAllowed().finish(),
                }
            } else {
                HttpResponse::InternalServerError()
                    .body(constants::MESSAGE_PROCESS_TOKEN_ERROR.to_string())
            }
        } else {
            HttpResponse::Unauthorized().body(constants::MESSAGE_SUPERADMIN_NOT_FOUND.to_string())
        }
    } else {
        HttpResponse::Unauthorized().body(constants::MESSAGE_TOKEN_MISSING.to_string())
    }
}

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
            }
            .render()
            .unwrap();

            return HttpResponse::Ok().content_type("text/html").body(template);
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
            let template = LoginBackOfficeTemplate {}.render().unwrap();
            HttpResponse::Ok().content_type("text/html").body(template)
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

#[cfg(test)]
mod tests {
    use super::*;
    use actix_cors::Cors;
    use actix_http::header::COOKIE;
    use actix_web::cookie::Cookie;
    use actix_web::{
        http::{header, StatusCode},
        test, web, App,
    };
    use testcontainers::clients;
    use testcontainers::images::postgres::Postgres;

    #[test]
    async fn test_login_get() {
        let docker = clients::Cli::default();
        let postgres = docker.run(Postgres::default());
        let pool = crate::config::db::init_db_pool(
            format!(
                "postgres://postgres:postgres@127.0.0.1:{}/postgres",
                postgres.get_host_port_ipv4(5432)
            )
            .as_str(),
        );
        crate::config::db::run_migration(&mut pool.get().unwrap());

        let app = test::init_service(
            App::new()
                .wrap(
                    Cors::default() // allowed_origin return access-control-allow-origin: * by default
                        .send_wildcard()
                        .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                        .allowed_headers(vec![
                            header::CONTENT_TYPE,
                            header::AUTHORIZATION,
                            header::ACCEPT,
                        ])
                        .max_age(3600),
                )
                .app_data(web::Data::new(pool.clone()))
                .wrap(actix_web::middleware::Logger::default())
                .configure(crate::config::app::config_services),
        )
        .await;

        let resp = test::TestRequest::get()
            .uri("/admin/login")
            .send_request(&app)
            .await;

        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[test]
    async fn test_login_post_with_valid_credentials() {
        let docker = clients::Cli::default();
        let postgres = docker.run(Postgres::default());
        let pool = crate::config::db::init_db_pool(
            format!(
                "postgres://postgres:postgres@127.0.0.1:{}/postgres",
                postgres.get_host_port_ipv4(5432)
            )
            .as_str(),
        );
        crate::config::db::run_migration(&mut pool.get().unwrap());

        let app = test::init_service(
            App::new()
                .wrap(
                    Cors::default() // allowed_origin return access-control-allow-origin: * by default
                        .send_wildcard()
                        .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                        .allowed_headers(vec![
                            header::CONTENT_TYPE,
                            header::AUTHORIZATION,
                            header::ACCEPT,
                        ])
                        .max_age(3600),
                )
                .app_data(web::Data::new(pool.clone()))
                .wrap(actix_web::middleware::Logger::default())
                .configure(crate::config::app::config_services),
        )
        .await;

        let login_dto = LoginDTO {
            username_or_email: "superadmin".to_string(),
            password: "azerty".to_string(),
        };

        let resp = test::TestRequest::post()
            .uri("/admin/login")
            .set_json(&login_dto)
            .send_request(&app)
            .await;

        assert_eq!(resp.status(), StatusCode::FOUND);
    }

    #[test]
    async fn test_login_post_with_invalid_credentials() {
        let docker = clients::Cli::default();
        let postgres = docker.run(Postgres::default());
        let pool = crate::config::db::init_db_pool(
            format!(
                "postgres://postgres:postgres@127.0.0.1:{}/postgres",
                postgres.get_host_port_ipv4(5432)
            )
            .as_str(),
        );
        crate::config::db::run_migration(&mut pool.get().unwrap());

        let app = test::init_service(
            App::new()
                .wrap(
                    Cors::default() // allowed_origin return access-control-allow-origin: * by default
                        .send_wildcard()
                        .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                        .allowed_headers(vec![
                            header::CONTENT_TYPE,
                            header::AUTHORIZATION,
                            header::ACCEPT,
                        ])
                        .max_age(3600),
                )
                .app_data(Data::new(pool.clone()))
                .wrap(actix_web::middleware::Logger::default())
                .configure(crate::config::app::config_services),
        )
        .await;

        let login_dto = LoginDTO {
            username_or_email: "toto".to_string(),
            password: "toto12".to_string(),
        };

        let resp = test::TestRequest::post()
            .uri("/admin/login")
            .set_json(&login_dto)
            .send_request(&app)
            .await;

        assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
    }

    #[test]
    async fn test_logout_with_valid_token() {
        let docker = clients::Cli::default();
        let postgres = docker.run(Postgres::default());
        let pool = crate::config::db::init_db_pool(
            format!(
                "postgres://postgres:postgres@127.0.0.1:{}/postgres",
                postgres.get_host_port_ipv4(5432)
            )
            .as_str(),
        );
        crate::config::db::run_migration(&mut pool.get().unwrap());

        let app = test::init_service(
            App::new()
                .wrap(
                    Cors::default() // allowed_origin return access-control-allow-origin: * by default
                        .send_wildcard()
                        .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                        .allowed_headers(vec![
                            header::CONTENT_TYPE,
                            header::AUTHORIZATION,
                            header::ACCEPT,
                        ])
                        .max_age(3600),
                )
                .app_data(web::Data::new(pool.clone()))
                .wrap(actix_web::middleware::Logger::default())
                .configure(crate::config::app::config_services),
        )
        .await;

        let login_dto = LoginDTO {
            username_or_email: "superadmin".to_string(),
            password: "azerty".to_string(),
        };

        let resp = test::TestRequest::post()
            .uri("/admin/login")
            .set_json(&login_dto)
            .send_request(&app)
            .await;

        assert_eq!(resp.status(), StatusCode::FOUND);

        if let Some(set_cookie_header) = resp.headers().get(SET_COOKIE) {
            if let Ok(cookie_str) = set_cookie_header.to_str() {
                if let Ok(cookie) = Cookie::parse(cookie_str) {
                    if let Some(token) = cookie.value().split(';').next() {
                        let req = test::TestRequest::post()
                            .uri("/admin/logout")
                            .append_header((COOKIE, format!("token={};", token)))
                            .send_request(&app)
                            .await;

                        assert_eq!(req.status(), StatusCode::FOUND);
                    }
                }
            }
        } else {
            println!("No Set-Cookie header found");
        }
    }

    #[test]
    async fn test_logout_without_token() {
        let docker = clients::Cli::default();
        let postgres = docker.run(Postgres::default());
        let pool = crate::config::db::init_db_pool(
            format!(
                "postgres://postgres:postgres@127.0.0.1:{}/postgres",
                postgres.get_host_port_ipv4(5432)
            )
            .as_str(),
        );
        crate::config::db::run_migration(&mut pool.get().unwrap());

        let app = test::init_service(
            App::new()
                .wrap(
                    Cors::default() // allowed_origin return access-control-allow-origin: * by default
                        .send_wildcard()
                        .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                        .allowed_headers(vec![
                            header::CONTENT_TYPE,
                            header::AUTHORIZATION,
                            header::ACCEPT,
                        ])
                        .max_age(3600),
                )
                .app_data(web::Data::new(pool.clone()))
                .wrap(actix_web::middleware::Logger::default())
                .configure(crate::config::app::config_services),
        )
        .await;

        let login_dto = LoginDTO {
            username_or_email: "superadmin".to_string(),
            password: "azerty".to_string(),
        };

        let resp = test::TestRequest::post()
            .uri("/admin/login")
            .set_json(&login_dto)
            .send_request(&app)
            .await;

        assert_eq!(resp.status(), StatusCode::FOUND);

        let req = test::TestRequest::post()
            .uri("/admin/logout")
            .send_request(&app)
            .await;

        assert_eq!(req.status(), StatusCode::BAD_REQUEST);
    }

    #[test]
    async fn test_homepage_back_office_with_valid_token() {
        let docker = clients::Cli::default();
        let postgres = docker.run(Postgres::default());
        let pool = crate::config::db::init_db_pool(
            format!(
                "postgres://postgres:postgres@127.0.0.1:{}/postgres",
                postgres.get_host_port_ipv4(5432)
            )
            .as_str(),
        );
        crate::config::db::run_migration(&mut pool.get().unwrap());

        let app = test::init_service(
            App::new()
                .wrap(
                    Cors::default() // allowed_origin return access-control-allow-origin: * by default
                        .send_wildcard()
                        .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                        .allowed_headers(vec![
                            header::CONTENT_TYPE,
                            header::AUTHORIZATION,
                            header::ACCEPT,
                        ])
                        .max_age(3600),
                )
                .app_data(web::Data::new(pool.clone()))
                .wrap(actix_web::middleware::Logger::default())
                .configure(crate::config::app::config_services),
        )
        .await;

        let login_dto = LoginDTO {
            username_or_email: "superadmin".to_string(),
            password: "azerty".to_string(),
        };

        let resp = test::TestRequest::post()
            .uri("/admin/login")
            .set_json(&login_dto)
            .send_request(&app)
            .await;

        assert_eq!(resp.status(), StatusCode::FOUND);

        if let Some(set_cookie_header) = resp.headers().get(SET_COOKIE) {
            if let Ok(cookie_str) = set_cookie_header.to_str() {
                if let Ok(cookie) = Cookie::parse(cookie_str) {
                    if let Some(token) = cookie.value().split(';').next() {
                        let req = test::TestRequest::get()
                            .uri("/admin/")
                            .append_header((COOKIE, format!("token={};", token)))
                            .send_request(&app)
                            .await;

                        assert_eq!(req.status(), StatusCode::OK);
                    }
                }
            }
        } else {
            println!("No Set-Cookie header found");
        }
    }

    #[test]
    async fn test_homepage_back_office_without_token() {
        let docker = clients::Cli::default();
        let postgres = docker.run(Postgres::default());
        let pool = crate::config::db::init_db_pool(
            format!(
                "postgres://postgres:postgres@127.0.0.1:{}/postgres",
                postgres.get_host_port_ipv4(5432)
            )
            .as_str(),
        );
        crate::config::db::run_migration(&mut pool.get().unwrap());

        let app = test::init_service(
            App::new()
                .wrap(
                    Cors::default() // allowed_origin return access-control-allow-origin: * by default
                        .send_wildcard()
                        .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                        .allowed_headers(vec![
                            header::CONTENT_TYPE,
                            header::AUTHORIZATION,
                            header::ACCEPT,
                        ])
                        .max_age(3600),
                )
                .app_data(web::Data::new(pool.clone()))
                .wrap(actix_web::middleware::Logger::default())
                .configure(crate::config::app::config_services),
        )
        .await;

        let req = test::TestRequest::get()
            .uri("/admin/")
            .send_request(&app)
            .await;

        assert_eq!(req.status(), StatusCode::BAD_REQUEST);
    }
}
