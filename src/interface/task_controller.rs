use actix_web::{post, web, HttpRequest, HttpResponse, Responder, Result};

use crate::use_case::{CreateTaskUseCase, CreateTaskUseCaseCommand};
use crate::CreateTaskInteractor;
use crate::domain::{TaskId, TaskName};

use serde::{ Deserialize, Serialize };

#[derive(Debug, Deserialize)]
pub struct CreateTaskParams {
  id: u64,
  name: String,
}

#[derive(Debug, Serialize)]
pub struct CreateTaskResponse {
  id: u64
}

impl CreateTaskResponse {
  pub fn new(id: u64) -> Self {
    Self {id}
  }
}

#[post("/tasks")]
pub async fn create_task(
  interactor: web::Data<CreateTaskInteractor>,
  params: web::Json<CreateTaskParams>,
) -> Result<impl Responder> {
  let id = TaskId(params.id); // TODO Validation
  let name = TaskName(params.name.to_owned()); // TODO Validation
  let command = CreateTaskUseCaseCommand::new(id.clone(), name);
  let result  = &interactor.execute(command).unwrap(); // TODO Error Handling
  let response = CreateTaskResponse::new(id.0);
  Ok(web::Json(response))
}
