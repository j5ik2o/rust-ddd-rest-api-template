pub mod enum_base;
pub mod trait_base;

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub enum TaskStatus {
  Undone,
  Done,
}

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub struct TaskId(pub u64);

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub struct TaskName(pub String);
