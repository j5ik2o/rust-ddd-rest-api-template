use std::rc::Rc;

pub trait Aggregate {
  type ID;
  fn id(&self) -> &Self::ID;
}
