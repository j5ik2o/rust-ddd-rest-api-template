use std::sync::{Arc, Mutex};

use anyhow::*;

use rust_ca_domain::{Task, TaskId, TaskName, TaskRepository};

#[derive(Debug, Clone)]
pub struct CreateTaskUseCaseCommand {
  id: TaskId,
  name: TaskName,
}

unsafe impl Send for CreateTaskUseCaseCommand {}

impl CreateTaskUseCaseCommand {
  pub fn new(id: TaskId, name: TaskName) -> Self {
    Self { id, name }
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
    let id = request.id.clone();
    let name = request.name.clone();
    let task = Task::new(id, name);
    let mut lock = self.task_repository.lock().unwrap();
    lock
      .store(task)
      .map(|_| CreateTaskUseCaseResult::new(request.id.clone()))
  }
}