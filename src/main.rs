use sqlx::{PgPool};
use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let configuration = get_configuration().expect("Failed to read configuration");
    let pg_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to database!");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    run(std::net::TcpListener::bind(address).unwrap(), pg_pool)?.await
}