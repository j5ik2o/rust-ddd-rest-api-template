use anyhow::Result;

pub trait Aggregate {
    type AggregateId;
    fn id(&self) -> &Self::AggregateId;
}

pub trait Repository: Send {
    type AID;
    type AR: Aggregate;
    fn resolve_by_id(&self, id: &Self::AID) -> Result<Option<&Self::AR>>;
    fn store(&mut self, aggregate: Self::AR) -> Result<()>;
}