use sqlx::{Connection, Executor, PgConnection, PgPool};

#[derive(serde::Deserialize)]
pub struct Settings{
    pub database: DatabaseSettings,
    pub application_port: u16
}

#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: String,
    pub host: String,
    pub database_name: String,
    pub port: u16
}

impl DatabaseSettings {
    pub fn connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.database_name
        )
    }
    pub fn connection_string_without_db(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}",
            self.username, self.password, self.host, self.port
        )
    }
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let settings = config::Config::builder()
        .add_source(config::File::with_name("configuration"))
        .build()?;
    settings.try_deserialize()
}

pub async fn configure_database(config: &DatabaseSettings) -> PgPool {
    let mut connection = PgConnection::connect(&config.connection_string_without_db())
        .await
        .expect("Failed to connect to database!");
    connection
       .execute(format!(r#"CREATE DATABASE "{}""#, config.database_name).as_str())
       .await
       .expect("Failed to create database");
    let db_pool = PgPool::connect(&config.connection_string())
        .await
        .expect("Failed to connect to Postgres!");
    sqlx::migrate!("./migrations")
        .run(&db_pool)
        .await
        .expect("Failed to create database!");
    db_pool
}