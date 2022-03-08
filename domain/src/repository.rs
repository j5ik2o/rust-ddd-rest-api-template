use anyhow::Result;
use std::rc::Rc;

pub trait Aggregate {
  type ID;
  fn id(&self) -> &Self::ID;
}

// pub trait Repository: Send {
//   type ID;
//   type AR;
//   fn resolve_by_id(&self, id: &Self::ID) -> Result<Option<&Rc<Self::AR>>>;
//   fn store(&mut self, aggregate: Rc<Self::AR>) -> Result<()>;
// }
