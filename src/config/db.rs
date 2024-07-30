use crate::models::user::{RoleType, User, UserDTO};
#[allow(unused_imports)]
use diesel::{
    pg::PgConnection,
    r2d2::{self, ConnectionManager},
    sql_query,
};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

pub type Connection = PgConnection;

pub type Pool = r2d2::Pool<ConnectionManager<Connection>>;

pub fn init_db_pool(url: &str) -> Pool {
    use log::info;

    info!("Migrating and configuring database...");
    let manager = ConnectionManager::<Connection>::new(url);
    Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
}

pub fn run_migration(conn: &mut PgConnection) {
    conn.run_pending_migrations(MIGRATIONS)
        .expect("Failed to run migrations");

    let superadmin_exists = User::get_superadmin_user(conn);

    if !superadmin_exists {
        User::insert(
            UserDTO {
                username: "superadmin".to_string(),
                entreprise_id: None,
                email: "mvast@syneidolab.com".to_string(),
                password: None,
                role: RoleType::SuperAdmin,
            },
            conn,
        )
        .expect("Error");
    }
}
