mod config;
mod db;
mod errors;
mod handlers;
mod routes;
mod utils;

use actix_web::{App, HttpServer, middleware::Logger, web};
use dotenv::dotenv;
use env_logger::Env;
use sqlx::PgPool;

use crate::config::Config;
use crate::db::connection::establish_connection;
use crate::routes::health_check;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let config = Config::from_env().expect("Failed to load configuration");

    let db_pool: PgPool = establish_connection(&config.database_url)
        .await
        .expect("Failed to connect DB");

    let config_clone = config.clone();

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(db_pool.clone()))
            .app_data(web::Data::new(config_clone.clone()))
            .configure(health_check::health_check_routes)
    })
    .bind((config.host.as_str(), config.port))?
    .run()
    .await
}
