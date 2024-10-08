use actix_web::{HttpResponse, ResponseError};
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::error::Error;

use crate::utils::response::StandardResponse;

/// Custom application error type
#[derive(Debug)]
pub enum AppError {
    BadRequest(String),
    Unauthorized(String),
    NotFound(String),
    InternalError(String),
}

impl Display for AppError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            AppError::BadRequest(msg) => write!(f, "Bad Request: {}", msg),
            AppError::Unauthorized(msg) => write!(f, "Unauthorized: {}", msg),
            AppError::NotFound(msg) => write!(f, "Not Found: {}", msg),
            AppError::InternalError(msg) => write!(f, "Internal Server Error: {}", msg),
        }
    }
}

impl Error for AppError {}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        let (code, message) = match self {
            AppError::BadRequest(msg) => (400, msg.clone()),
            AppError::Unauthorized(msg) => (401, msg.clone()),
            AppError::NotFound(msg) => (404, msg.clone()),
            AppError::InternalError(msg) => (500, msg.clone()),
        };

        let response = StandardResponse::<()>::new_error(code, message);

        HttpResponse::build(actix_web::http::StatusCode::from_u16(code).unwrap())
            .json(response)
    }
}