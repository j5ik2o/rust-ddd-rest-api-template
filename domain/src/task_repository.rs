use std::rc::Rc;

use crate::trait_base::Task;
use crate::TaskId;
use anyhow::Result;

pub trait TaskRepository: Send {
  fn resolve_by_id(&self, id: &TaskId) -> Result<Option<&Rc<dyn Task<ID = TaskId>>>>;
  fn store(&mut self, aggregate: Rc<dyn Task<ID = TaskId>>) -> Result<()>;
}
