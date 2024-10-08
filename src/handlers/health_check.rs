use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;
use crate::errors::AppError;

#[derive(serde::Serialize)]
struct HealthCheckResponse {
    status: String,
}

pub async fn health_check(pool: web::Data<PgPool>) -> Result<impl Responder, AppError> {
    match pool.acquire().await {
        Ok(_) => Ok(HttpResponse::Ok().json(HealthCheckResponse {
            status: "Ok".to_string()
        })),
        Err(_) => Err(AppError::InternalError("DB failed".to_string()))
    }
}