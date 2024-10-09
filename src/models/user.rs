use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct User {
  pub id: i32,
  pub email: String,
  pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserRegisterDto {
  pub email: String,
  pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserLoginDto {
  pub email: String,
  pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserResponse {
  pub id: i32,
  pub email: String,
}