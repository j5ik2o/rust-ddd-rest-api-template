use std::collections::HashMap;
use std::rc::Rc;

use anyhow::Result;

use rust_ca_domain::{Aggregate, Task, TaskId, TaskRepository};

pub struct TaskRepositoryInMemory {
  pub aggregates: HashMap<TaskId, Rc<dyn Task<ID = TaskId>>>,
}

unsafe impl Send for TaskRepositoryInMemory {}

impl TaskRepositoryInMemory {
  pub fn new() -> Self {
    Self {
      aggregates: HashMap::new(),
    }
  }
}

impl TaskRepository for TaskRepositoryInMemory {
  fn resolve_by_id(&self, id: &TaskId) -> Result<Option<&Rc<dyn Task<ID = TaskId>>>> {
    Ok(self.aggregates.get(id).clone())
  }

  fn store(&mut self, aggregate: Rc<dyn Task<ID = TaskId>>) -> Result<()> {
    self.aggregates.insert(aggregate.id().clone() as TaskId, aggregate);
    Ok(())
  }
}
