mod config;
mod constants;
mod controller;
mod templates;

use actix_cors::Cors;
use actix_web::{http::header, web, App, HttpServer};
use std::{env, fs, io, os::unix::fs::PermissionsExt, path::Path};

fn create_directory_if_not_exists(path: &Path) -> io::Result<()> {
    if !path.exists() {
        fs::create_dir_all(path)?;
        fs::set_permissions(path, fs::Permissions::from_mode(0o775))?;
        println!(
            "✅ Folder path : './{}' created with permission 0775 successfully",
            path.display()
        );
    }
    Ok(())
}

#[actix_rt::main]
async fn main() -> io::Result<()> {
    dotenv::dotenv().expect("Failed to read .env file");
    env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();

    let upload_cv = Path::new(constants::PATH_UPLOAD_CV);
    create_directory_if_not_exists(upload_cv)?;

    let app_host = env::var("APP_HOST").expect("APP_HOST not found.");
    let app_port = env::var("APP_PORT").expect("APP_PORT not found.");
    let forwarded_port = match env::var("FORWARDED_PORT") {
        Ok(value) => value,         // Use the value from the environment if available
        Err(_) => app_port.clone(), // Use app_port if FORWARDED_PORT is not set
    };
    let app_url = format!("{}:{}", &app_host, &app_port);
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL not found.");

    let cors_allow_origin = match env::var("CORS_ALLOW_ORIGIN") {
        Ok(value) => value, // Use the value from the environment if available
        Err(_) => format!("http://127.0.0.1:{}", &forwarded_port),
    };

    let pool = config::db::init_db_pool(&db_url);
    let conn = &mut pool
        .get()
        .expect("Failed to get a connection from the pool");
    config::db::run_migration(conn);

    println!("✅ Connected to database and table created !");
    println!("{}", constants::SERVER_STARTED);

    HttpServer::new(move || {
        // Split the string by '|' delimiter
        let allowed_origins: Vec<&str> = (cors_allow_origin).split('|').collect();
        let mut cors = Cors::default()
            .send_wildcard()
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .allowed_headers(vec![
                header::CONTENT_TYPE,
                header::AUTHORIZATION,
                header::ACCEPT,
            ])
            .max_age(3600);

        // Add each origin to the Cors builder
        for origin in &allowed_origins {
            cors = cors.allowed_origin(origin.trim()); // Trim leading/trailing whitespace
        }

        App::new()
            .wrap(cors)
            .app_data(web::Data::new(pool.clone()))
            .wrap(actix_web::middleware::Logger::default())
            .wrap(actix_web::middleware::Logger::new(
                "%a %{User-Agent}i %{Host}i",
            ))
            .configure(config::app::config_services)
    })
    .bind(&app_url)?
    .workers(2)
    .run()
    .await
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{http::{header, StatusCode}, test, App};
    use std::{fs, path::Path};
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

    #[actix_web::test]
    async fn test_startup_ok() {
        //CODE OK FOR testcontainers 0.14.0 not 0.20
        let docker = clients::Cli::default();
        let postgres = docker.run(Postgres::default());
        let pool = config::db::init_db_pool(
            format!(
                "postgres://postgres:postgres@127.0.0.1:{}/postgres",
                postgres.get_host_port_ipv4(5432)
            )
            .as_str(),
        );
        config::db::run_migration(&mut pool.get().unwrap());

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
                .configure(config::app::config_services)
        })
        .bind("localhost:8001".to_string())
        .unwrap()
        .run();

        assert_eq!(true, true);
    }

    #[actix_web::test]
    async fn test_startup_health_check_ok() {
        //CODE OK FOR testcontainers 0.14.0 not 0.20
        let docker = clients::Cli::default();
        let postgres = docker.run(Postgres::default());
        let pool = config::db::init_db_pool(
            format!(
                "postgres://postgres:postgres@127.0.0.1:{}/postgres",
                postgres.get_host_port_ipv4(5432)
            )
                .as_str(),
        );
        config::db::run_migration(&mut pool.get().unwrap());

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
                .configure(config::app::config_services)
        )
            .await;

        let resp = test::TestRequest::get()
            .uri("/health-check")
            .send_request(&app)
            .await;

        assert_eq!(resp.status(), StatusCode::OK);
    }
}
