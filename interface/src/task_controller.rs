use actix_web::{post, web, HttpResponse, Responder, Result};
use chrono::prelude::*;
use chrono::serde::ts_seconds::deserialize as from_ts;
use chrono::{Date, DateTime, Duration, Local, Utc};
use serde::{Deserialize, Serialize};

use rust_ca_domain::{TaskId, TaskName};
use rust_ca_use_case::{
  CreateTaskInteractor, CreateTaskUseCase, CreateTaskUseCaseCommand, PostponeTaskInteractor, PostponeTaskUseCase,
  PostponeTaskUseCaseCommand,
};

#[derive(Serialize)]
struct ErrorResponse {
  message: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateTaskParams {
  id: u64,
  name: String,
  #[serde(deserialize_with = "from_ts")]
  due_date: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize)]
pub struct CreateTaskResponse {
  id: u64,
}

impl CreateTaskResponse {
  pub fn new(id: u64) -> Self {
    Self { id }
  }
}

#[post("/tasks")]
pub async fn create_task(
  interactor: web::Data<CreateTaskInteractor>,
  params: web::Json<CreateTaskParams>,
) -> impl Responder {
  let id = TaskId(params.id); // TODO Validation
  let name = TaskName(params.name.to_owned()); // TODO Validation
  let command = CreateTaskUseCaseCommand::new(id.clone(), name, params.due_date);
  match interactor.execute(command) {
    Ok(result) => HttpResponse::Ok().json(CreateTaskResponse::new(result.id.0)),
    Err(err) => {
      let response = ErrorResponse {
        message: format!("Failed to create task: {}", err),
      };
      HttpResponse::InternalServerError().json(response)
    }
  }
}

#[derive(Debug, Clone, Deserialize)]
pub struct PostponeTaskRequest {
  id: u64,
  days: u32,
}

#[derive(Debug, Clone, Serialize)]
pub struct PostponeTaskResponse {
  id: u64,
}

impl PostponeTaskResponse {
  pub fn new(id: u64) -> Self {
    Self { id }
  }
}

#[post("/tasks/{task_id}/postpone")]
pub async fn postpone_task(
  interactor: web::Data<PostponeTaskInteractor>,
  task_id: web::Path<String>,
) -> impl Responder {
  let id = TaskId(task_id.parse().unwrap());
  let command = PostponeTaskUseCaseCommand::new(id.clone());
  match interactor.execute(command) {
    Ok(result) => HttpResponse::Ok().json(CreateTaskResponse::new(result.id.0)),
    Err(err) => {
      let response = ErrorResponse {
        message: format!("Failed to postpone task: {}", err),
      };
      HttpResponse::InternalServerError().json(response)
    }
  }
}
