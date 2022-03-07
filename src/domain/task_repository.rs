use std::collections::HashMap;
use crate::domain::{Task, TaskId};

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
