use actix_web::{web, App, HttpServer};
use actix_web::dev::Server;
use sqlx::PgPool;

use routes::{health_check, subscribe};
use crate::routes;

pub fn run(
    listener: std::net::TcpListener,
    db_pool: PgPool
) -> Result<Server, std::io::Error> {
    let pool = web::Data::new(db_pool);
    let server = HttpServer::new( move || {
            App::new()
                .route("/health_check", web::get().to(health_check))
                .route("/subscribe", web::post().to(subscribe))
                .app_data(pool.clone())
        })
        .listen(listener)?
        .run();

    Ok(server)
}