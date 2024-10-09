use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Task {
  pub id: i32,
  pub user_id: i32,
  pub title: String,
  pub description: String,
  pub completed: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTaskDto {
  pub title: String,
  pub description: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateTaskDto {
  pub title: String,
  pub description: String,
  pub completed: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TaskResponse {
  pub id: i32,
  pub user_id: i32,
  pub title: String,
  pub description: String,
  pub completed: bool,
}