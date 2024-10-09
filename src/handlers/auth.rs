use actix_web::{web, HttpResponse, Responder};
use crate::models::user::{UserLoginDto, UserRegisterDto};
use crate::utils::response::StandardResponse;

pub async fn register(body: web::Json<UserRegisterDto>) -> impl Responder {
    let email = &body.email;

    let response = StandardResponse::new("User register successfully".to_string(), Some(email.clone()));

    HttpResponse::Created().json(response)
}

pub async fn login(body: web::Json<UserLoginDto>) -> impl Responder {
    let email = &body.email;

    let response = StandardResponse::success("Login successfully".to_string(), Some(email.clone()));

    HttpResponse::Ok().json(response)
}