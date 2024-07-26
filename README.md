# Project Platform CV
A simple app using Actix-web, Diesel and Docker.

## Require
- [Rust Stable](https://rustup.rs)
- [Postgres](https://www.postgresql.org/)

Or using [Docker](https://www.docker.com/)

## How to run
### Manual
- Install postgresql and sqlite backend libraries, more details here
  - Install `libpq` and `libsqlite3` depends on your distribution.
- Enter into project directory
- Change values in `.env` if needed
- Build : `cargo build`
- Enjoy! 😄

### Docker
If you want to use docker for the database, follow these steps : 

- Enter into project directory
- Run `docker-compose up -d` for local environment
  or `docker-compose -f docker-compose.prod.yml up` for production environment
- The database is mounted and accessible
- Enjoy! 😄

### Diesel
[DIESEL](https://diesel.rs/)
- #### Linux/MacOS
`curl --proto '=https' --tlsv1.2 -LsSf https://github.com/diesel-rs/diesel/releases/download/v2.2.1/diesel_cli-installer.sh | sh`<br/><br/>
OR
- #### Windows
`powershell -c "irm https://github.com/diesel-rs/diesel/releases/download/v2.2.1/diesel_cli-installer.ps1 | iex"`<br/><br/>
OR
- #### Cargo
`cargo install diesel_cli`

- #### Execute migrations
  - Enter into project directory
  - Run : `diesel migration run`
  - Database schemas are created
  - Enjoy! 😄

OR
- #### Execute migrations with cargo run
In main.rs l.47 : `config::db::run_migration(conn);`
- Run : `cargo run`
- `✅ Connected to database and table created !`
- Enjoy! 😄

### Test
- Enter into project directory
- Run : `cargo test -- --nocapture`
- Enjoy! 😄
- 
### Support and Contributions
If you find this project useful and would like to support its development, consider making a contribution or sending a tip to 0xe79B2cc4c07dB560f8e1eE63ed407DD2DCFdE80e
