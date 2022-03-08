use anyhow::*;
use chrono::Local;
use mopa::*;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

use rust_ca_domain::{PostponeableUndoneTask, Task, TaskId, TaskName, TaskRepository, UndoneTask};
use rust_ca_infrastructure::TaskRepositoryInMemory;

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

    // mopaを使ったダウンキャスト
    let task_rc = lock.resolve_by_id(&request.id).unwrap().unwrap().clone();
    match task_rc.downcast_ref::<PostponeableUndoneTask>() {
      Some(task) => lock
        .store(task.postpone())
        .map(|_| PostponeTaskUseCaseResult::new(request.id.clone())),
      None => Ok(PostponeTaskUseCaseResult::new(request.id.clone())),
    }
  }
}
