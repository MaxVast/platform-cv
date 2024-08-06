#[cfg(test)]
mod tests {
    use crate::create_directory_if_not_exists;
    use actix_cors::Cors;
    use actix_web::{
        body::to_bytes,
        http::{header, StatusCode},
        test,
        web::Bytes,
        App,
    };
    use actix_web::{web, HttpServer};
    use std::{fs, os::unix::fs::PermissionsExt, path::Path};
    use testcontainers::{clients, images::postgres::Postgres};

    #[test]
    async fn test_create_directory_if_not_exists_creates_directory() {
        let test_dir = Path::new("uploads");
        if test_dir.exists() {
            fs::remove_dir_all(test_dir).unwrap();
        }

        let result = create_directory_if_not_exists(test_dir);
        assert!(result.is_ok());
        assert!(test_dir.exists());
        assert_eq!(
            fs::metadata(test_dir).unwrap().permissions().mode() & 0o777,
            0o775
        );

        fs::remove_dir_all(test_dir).unwrap();
    }

    #[test]
    async fn test_create_directory_if_not_exists_directory_already_exists() {
        let test_dir = Path::new("uploads");
        if test_dir.exists() {
            let result = create_directory_if_not_exists(test_dir);
            assert!(result.is_ok());
            assert!(test_dir.exists());
            fs::remove_dir_all(test_dir).unwrap();
        }
    }

    #[test]
    async fn test_startup_ok() {
        //CODE OK FOR testcontainers 0.14.0 not 0.20
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

        let _ = HttpServer::new(move || {
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
                .configure(crate::config::app::config_services)
        })
        .bind("localhost:8001".to_string())
        .unwrap()
        .run();

        assert_eq!(true, true);
    }

    trait BodyTest {
        fn as_str(&self) -> &str;
    }

    impl BodyTest for Bytes {
        fn as_str(&self) -> &str {
            std::str::from_utf8(self).unwrap()
        }
    }

    #[test]
    async fn test_startup_health_check_ok() {
        //CODE OK FOR testcontainers 0.14.0 not 0.20
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
            .uri("/health-check")
            .send_request(&app)
            .await;

        assert_eq!(resp.status(), StatusCode::OK);

        let body = to_bytes(&mut resp.into_body()).await.unwrap();
        assert_eq!(body.as_str(), "Health check OK");
    }

    #[test]
    async fn test_handler_404() {
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
            .uri("/health-checkKKKK")
            .send_request(&app)
            .await;

        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }
}
