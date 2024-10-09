use actix_web::{web, Responder};
use sqlx::PgPool;
use crate::errors::AppError;
use crate::utils::response::StandardResponse;

pub async fn health_check(pool: web::Data<PgPool>) -> Result<impl Responder, AppError> {
    match pool.acquire().await {
        Ok(_) => {
            let response = StandardResponse::<()>::success_no_data("OK".into());
            Ok(web::Json(response))
        },
        Err(_) => Err(AppError::InternalError("DB failed".into()))
    }
}