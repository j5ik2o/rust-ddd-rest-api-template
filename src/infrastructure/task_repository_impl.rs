use anyhow::Result;

use crate::domain::{Aggregate, Repository, Task, TaskId, TaskRepositoryInMemory};

impl Repository for TaskRepositoryInMemory {
  type AID = TaskId;
  type AR = Task;

  fn resolve_by_id(&self, id: &Self::AID) -> Result<Option<&Self::AR>> {
    Ok(self.aggregates.get(id))
  }

  fn store(&mut self, aggregate: Task) -> Result<()> {
    self.aggregates.insert(aggregate.id().clone(), aggregate);
    Ok(())
  }
}
