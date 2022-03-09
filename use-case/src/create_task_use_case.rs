use std::rc::Rc;
use std::sync::{Arc, Mutex};

use anyhow::*;
use chrono::{Date, DateTime, Duration, Local, Utc};

use crate::TaskUseCaseError;
use rust_ca_domain::{PostponeableUndoneTask, Task, TaskId, TaskName, TaskRepository, UndoneTask};
use rust_ca_infrastructure::TaskRepositoryInMemory;

use crate::TaskUseCaseError::RepositoryError;

#[derive(Debug, Clone)]
pub struct CreateTaskUseCaseCommand {
  id: TaskId,
  name: TaskName,
  due_date: DateTime<Utc>,
}

unsafe impl Send for CreateTaskUseCaseCommand {}

impl CreateTaskUseCaseCommand {
  pub fn new(id: TaskId, name: TaskName, due_date: DateTime<Utc>) -> Self {
    Self { id, name, due_date }
  }
}

#[derive(Debug, Clone)]
pub struct CreateTaskUseCaseResult {
  pub id: TaskId,
}

impl CreateTaskUseCaseResult {
  pub fn new(id: TaskId) -> Self {
    Self { id }
  }
}

pub trait CreateTaskUseCase {
  fn execute(&self, request: CreateTaskUseCaseCommand) -> Result<CreateTaskUseCaseResult>;
}

pub struct CreateTaskInteractor {
  task_repository: Arc<Mutex<dyn TaskRepository>>,
}

impl CreateTaskInteractor {
  pub fn new(task_repository: Arc<Mutex<dyn TaskRepository>>) -> Self {
    Self { task_repository }
  }
}

impl CreateTaskUseCase for CreateTaskInteractor {
  fn execute(&self, request: CreateTaskUseCaseCommand) -> Result<CreateTaskUseCaseResult> {
    let mut lock = self.task_repository.lock().unwrap();

    let task_rc = Rc::new(PostponeableUndoneTask::new(
      request.id.clone(),
      request.name,
      request.due_date,
    ));

    lock
      .store(task_rc)
      .map_err(|_| anyhow::Error::new(TaskUseCaseError::RepositoryError))
      .map(|_| CreateTaskUseCaseResult::new(request.id.clone()))
  }
}
