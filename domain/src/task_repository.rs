use crate::{Repository, Task, TaskId};

pub trait TaskRepository:  Repository<AID=TaskId, AR=Task> {}