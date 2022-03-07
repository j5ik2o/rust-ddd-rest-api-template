use crate::domain::task::{Task, TaskId};
use anyhow::Result;
use std::collections::HashMap;

pub trait Repository: Send {
  type AggregateId;
  type Aggregate;
  fn resolve_by_id(&self, id: &Self::AggregateId) -> Result<Option<&Self::Aggregate>>;
  fn store(&mut self, aggregate: Self::Aggregate) -> Result<()>;
}

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
