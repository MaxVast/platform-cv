//TESTS USER
#[cfg(test)]
mod tests {
    use crate::models::user::{LoginDTO, RoleType, UserDTO};
    use actix_cors::Cors;
    use actix_http::header::{COOKIE, SET_COOKIE};
    use actix_web::cookie::Cookie;
    use actix_web::web::Data;
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

    #[test]
    async fn test_get_signup_user_with_valid_token() {
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

                        let resp = test::TestRequest::get()
                            .uri("/admin/signup")
                            .append_header((COOKIE, format!("token={};", token)))
                            .send_request(&app)
                            .await;

                        assert_eq!(resp.status(), StatusCode::OK);
                    }
                }
            }
        } else {
            println!("No Set-Cookie header found");
        }
    }

    #[test]
    async fn test_post_signup_user_with_valid_token() {
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

                        let resp_get = test::TestRequest::get()
                            .uri("/admin/signup")
                            .append_header((COOKIE, format!("token={};", token)))
                            .send_request(&app)
                            .await;

                        assert_eq!(resp_get.status(), StatusCode::OK);

                        let user_dto = UserDTO {
                            username: "testUsername".to_string(),
                            password: Option::from("testPassword".to_string()),
                            company_id: Option::from(None),
                            email: "testUsername@mail.com".to_string(),
                            role: RoleType::User,
                            login_session: None,
                        };

                        let resp_post_user = test::TestRequest::post()
                            .uri("/admin/signup")
                            .append_header((COOKIE, format!("token={};", token)))
                            .set_json(&user_dto)
                            .send_request(&app)
                            .await;

                        assert_eq!(resp_post_user.status(), StatusCode::CREATED);
                    }
                }
            }
        } else {
            println!("No Set-Cookie header found");
        }
    }

    #[test]
    async fn test_post_signup_user_already_exists() {
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

                        let resp_get = test::TestRequest::get()
                            .uri("/admin/signup")
                            .append_header((COOKIE, format!("token={};", token)))
                            .send_request(&app)
                            .await;

                        assert_eq!(resp_get.status(), StatusCode::OK);

                        let user_dto = UserDTO {
                            username: "testUsername".to_string(),
                            password: Option::from("testPassword".to_string()),
                            company_id: Option::from(None),
                            email: "testUsername@mail.com".to_string(),
                            role: RoleType::User,
                            login_session: None,
                        };

                        let resp_post_user = test::TestRequest::post()
                            .uri("/admin/signup")
                            .append_header((COOKIE, format!("token={};", token)))
                            .set_json(&user_dto)
                            .send_request(&app)
                            .await;

                        assert_eq!(resp_post_user.status(), StatusCode::CREATED);

                        let user_dto2 = UserDTO {
                            username: "testUsername".to_string(),
                            password: Option::from("testPassword".to_string()),
                            company_id: Option::from(None),
                            email: "testUsername@mail.com".to_string(),
                            role: RoleType::User,
                            login_session: None,
                        };

                        let resp_post_user2 = test::TestRequest::post()
                            .uri("/admin/signup")
                            .append_header((COOKIE, format!("token={};", token)))
                            .set_json(&user_dto2)
                            .send_request(&app)
                            .await;

                        assert_eq!(resp_post_user2.status(), StatusCode::BAD_REQUEST);
                    }
                }
            }
        } else {
            println!("No Set-Cookie header found");
        }
    }

    #[test]
    async fn test_get_signup_user_without_token() {
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

        let resp_get = test::TestRequest::get()
            .uri("/admin/signup")
            .send_request(&app)
            .await;

        assert_eq!(resp_get.status(), StatusCode::UNAUTHORIZED);
    }

    #[test]
    async fn test_post_signup_user_without_role_superadmin() {
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
                        let user_dto = UserDTO {
                            username: "testUsername".to_string(),
                            password: Option::from("testPassword".to_string()),
                            company_id: Option::from(None),
                            email: "testUsername@mail.com".to_string(),
                            role: RoleType::User,
                            login_session: None,
                        };

                        let resp_post_user = test::TestRequest::post()
                            .uri("/admin/signup")
                            .append_header((COOKIE, format!("token={};", token)))
                            .set_json(&user_dto)
                            .send_request(&app)
                            .await;

                        assert_eq!(resp_post_user.status(), StatusCode::CREATED);

                        let login_dto = LoginDTO {
                            username_or_email: "testUsername".to_string(),
                            password: "testPassword".to_string(),
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
                                        let user_dto2 = UserDTO {
                                            username: "testUsername".to_string(),
                                            password: Option::from("testPassword".to_string()),
                                            company_id: Option::from(None),
                                            email: "testUsername@mail.com".to_string(),
                                            role: RoleType::User,
                                            login_session: None,
                                        };

                                        let resp_post_user = test::TestRequest::post()
                                            .uri("/admin/signup")
                                            .append_header((COOKIE, format!("token={};", token)))
                                            .set_json(&user_dto2)
                                            .send_request(&app)
                                            .await;

                                        assert_eq!(
                                            resp_post_user.status(),
                                            StatusCode::UNAUTHORIZED
                                        );
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            println!("No Set-Cookie header found");
        }
    }
}
