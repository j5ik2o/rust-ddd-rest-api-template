use crate::Aggregate;

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub struct TaskId(pub u64);

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub struct TaskName(pub String);

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub struct Task {
  id: TaskId,
  name: TaskName,
}

impl Aggregate for Task {
  type ID = TaskId;

  fn id(&self) -> &Self::ID {
    &self.id
  }
}

impl Task {
  pub fn new(id: TaskId, name: TaskName) -> Self {
    Self { id, name }
  }
}
