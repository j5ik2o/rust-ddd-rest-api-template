use std::rc::Rc;
use std::sync::{Arc, Mutex};

use anyhow::*;
use chrono::Local;

use crate::TaskUseCaseError;
use rust_ca_domain::trait_base::PostponeableUndoneTask;
use rust_ca_domain::{TaskId, TaskName, TaskRepository};
use rust_ca_infrastructure::TaskRepositoryInMemory;

use crate::TaskUseCaseError::RepositoryError;

#[derive(Debug, Clone)]
pub struct PostponeTaskUseCaseCommand {
  id: TaskId,
}

unsafe impl Send for PostponeTaskUseCaseCommand {}

impl PostponeTaskUseCaseCommand {
  pub fn new(id: TaskId) -> Self {
    Self { id }
  }
}

#[derive(Debug, Clone)]
pub struct PostponeTaskUseCaseResult {
  pub id: TaskId,
}

impl PostponeTaskUseCaseResult {
  pub fn new(id: TaskId) -> Self {
    Self { id }
  }
}

pub trait PostponeTaskUseCase {
  fn execute(&self, request: PostponeTaskUseCaseCommand) -> Result<PostponeTaskUseCaseResult>;
}

pub struct PostponeTaskInteractor {
  task_repository: Arc<Mutex<dyn TaskRepository>>,
}

impl PostponeTaskInteractor {
  pub fn new(task_repository: Arc<Mutex<dyn TaskRepository>>) -> Self {
    Self { task_repository }
  }
}

impl PostponeTaskUseCase for PostponeTaskInteractor {
  fn execute(&self, request: PostponeTaskUseCaseCommand) -> Result<PostponeTaskUseCaseResult> {
    let mut lock = self.task_repository.lock().unwrap();
    let task_rc = lock.resolve_by_id(&request.id).unwrap().unwrap().clone();

    match task_rc.downcast_ref::<PostponeableUndoneTask>() {
      Some(task) => lock
        .store(task.postpone())
        .map_err(|_| anyhow::Error::new(TaskUseCaseError::RepositoryError))
        .map(|_| PostponeTaskUseCaseResult::new(request.id.clone())),
      None => Err(TaskUseCaseError::StateError)?,
    }
  }
}
