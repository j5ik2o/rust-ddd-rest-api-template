use actix_web::{post, Responder, Result, web};
use chrono::{Date, DateTime, Duration, Local, Utc};
use chrono::prelude::*;
use chrono::serde::ts_seconds::deserialize as from_ts;
use serde::{Deserialize, Serialize};

use rust_ca_domain::{TaskId, TaskName};
use rust_ca_use_case::{CreateTaskInteractor, CreateTaskUseCase, CreateTaskUseCaseCommand};

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
) -> Result<impl Responder> {
  let id = TaskId(params.id); // TODO Validation
  let name = TaskName(params.name.to_owned()); // TODO Validation
  let command = CreateTaskUseCaseCommand::new(id.clone(), name, params.due_date);
  let result = &interactor.execute(command).unwrap(); // TODO Error Handling
  let response = CreateTaskResponse::new(id.0);
  Ok(web::Json(response))
}

#[derive(Debug, Clone, Deserialize)]
pub struct PostponeTaskRequest {
  id: u64,
  days: u32,
}