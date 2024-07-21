mod config;
mod constants;

use actix_cors::Cors;
use actix_web::{App, HttpServer, web, http::header};
use std::{
    env, io,
    fs,
    os::unix::fs::PermissionsExt,
    path::Path
};

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
        Ok(value) => value, // Use the value from the environment if available
        Err(_) => app_port.clone(), // Use app_port if FORWARDED_PORT is not set
    };
    let app_url = format!("{}:{}", &app_host, &app_port);
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL not found.");

    let cors_allow_origin = match env::var("CORS_ALLOW_ORIGIN") {
        Ok(value) => value, // Use the value from the environment if available
        Err(_) => format!("http://127.0.0.1:{}", &forwarded_port),
    };

    println!("{}", constants::SERVER_STARTED);

    let pool = config::db::init_db_pool(&db_url);
    let conn = &mut pool.get().expect("Failed to get a connection from the pool");
    config::db::run_migration(conn);

    println!("✅ Connected to database and table created !");

    HttpServer::new(move || {

        // Split the string by '|' delimiter
        let allowed_origins: Vec<&str> = (&cors_allow_origin).split('|').collect();
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
            .wrap(actix_web::middleware::Logger::new("%a %{User-Agent}i %{Host}i"))
            //.wrap(crate::middleware::auth_middleware::Authentication) // Comment this line if you want to integrate with yew-address-book-frontend
            //.wrap_fn(|req, srv| srv.call(req).map(|res| res))
            //.configure(config::app::config_services)
    })
        .bind(&app_url)?
        .run()
        .await
}
