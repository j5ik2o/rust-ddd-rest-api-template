use crate::domain::{Repository, Task, TaskId, TaskRepositoryInMemory};
use anyhow::Result;

impl Repository for TaskRepositoryInMemory {
  type Aggregate = Task;
  type AggregateId = TaskId;

  fn resolve_by_id(&self, id: &Self::AggregateId) -> Result<Option<&Self::Aggregate>> {
    Ok(self.aggregates.get(id))
  }

  fn store(&mut self, aggregate: Self::Aggregate) -> Result<()> {
    todo!()
  }
}
