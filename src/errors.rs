use actix_web::{HttpResponse, ResponseError};
use serde::Serialize;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::error::Error;

/// Custom application error type
#[derive(Debug)]
pub enum AppError {
    BadRequest(String),
    Unauthorized(String),
    NotFound(String),
    InternalError(String),
}

/// Struct for serializing error responses
#[derive(Serialize)]
struct ErrorResponse {
    error: String,
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
        let error_message = self.to_string();
        let error_response = ErrorResponse {
            error: error_message,
        };

        match self {
            AppError::BadRequest(_) => HttpResponse::BadRequest().json(error_response),
            AppError::Unauthorized(_) => HttpResponse::Unauthorized().json(error_response),
            AppError::NotFound(_) => HttpResponse::NotFound().json(error_response),
            AppError::InternalError(_) => HttpResponse::InternalServerError().json(error_response),
        }
    }
}