use anyhow::Result;

pub trait Aggregate {
  type ID;
  fn id(&self) -> &Self::ID;
}

pub trait Repository: Send {
  type AR: Aggregate;
  fn resolve_by_id(&self, id: &<Self::AR as Aggregate>::ID) -> Result<Option<&Self::AR>>;
  fn store(&mut self, aggregate: Self::AR) -> Result<()>;
}
