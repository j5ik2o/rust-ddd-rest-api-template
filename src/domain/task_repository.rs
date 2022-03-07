use crate::domain::{Task, TaskId};
use std::collections::HashMap;

pub struct TaskRepositoryInMemory {
  pub(crate) aggregates: HashMap<TaskId, Task>,
}

impl TaskRepositoryInMemory {
  pub fn new() -> Self {
    Self {
      aggregates: HashMap::new(),
    }
  }
}
