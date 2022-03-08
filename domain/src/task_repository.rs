use crate::{Repository, Task, TaskId};

pub trait TaskRepository:  Repository<AR=Task> {}