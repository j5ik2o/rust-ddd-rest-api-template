use anyhow::Result;
use std::rc::Rc;

use crate::{Task, TaskId};

pub trait TaskRepository: Send {
  fn resolve_by_id(&self, id: &TaskId) -> Result<Option<&Rc<dyn Task>>>;
  fn store(&mut self, aggregate: Rc<dyn Task>) -> Result<()>;
}
