use actix_web::web;
use crate::handlers::health_check::health_check;

pub fn health_check_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/health")
            .route("", web::get().to(health_check))
    );
}