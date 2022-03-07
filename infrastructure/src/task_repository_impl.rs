use std::collections::HashMap;

use anyhow::Result;

use rust_ca_domain::{Aggregate, Repository, Task, TaskId, TaskRepository};

pub struct TaskRepositoryInMemory {
    pub aggregates: HashMap<TaskId, Task>,
}

impl TaskRepositoryInMemory {
    pub fn new() -> Self {
        Self {
            aggregates: HashMap::new(),
        }
    }
}

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

impl TaskRepository for TaskRepositoryInMemory {}