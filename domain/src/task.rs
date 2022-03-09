use std::rc::Rc;

use chrono::{Date, DateTime, Local, Utc};
use downcast_rs::{impl_downcast, Downcast};

use crate::Aggregate;

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub enum TaskStatus {
  Undone,
  Done,
}

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub struct TaskId(pub u64);

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub struct TaskName(pub String);

pub trait Task: Aggregate<ID = TaskId> + Downcast {
  fn name(&self) -> &TaskName;
  fn status(&self) -> &TaskStatus;
}

impl_downcast!(Task);

pub trait UndoneTask: Task {
  fn due_date(&self) -> &DateTime<Utc>;
  fn done(&self) -> DoneTask;
}

// --- PostponeableUndoneTask

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub struct PostponeableUndoneTask {
  id: TaskId,
  name: TaskName,
  status: TaskStatus,
  due_date: DateTime<Utc>,
  postpone_count: u32,
}

impl Aggregate for PostponeableUndoneTask {
  type ID = TaskId;

  fn id(&self) -> &Self::ID {
    &self.id
  }
}

impl Task for PostponeableUndoneTask {
  fn name(&self) -> &TaskName {
    &self.name
  }

  fn status(&self) -> &TaskStatus {
    &self.status
  }
}

impl PostponeableUndoneTask {
  const POSTPONE_MAX_COUNT: u32 = 3;

  pub fn new(id: TaskId, name: TaskName, due_date: DateTime<Utc>) -> Self {
    Self {
      id,
      name,
      status: TaskStatus::Undone,
      due_date,
      postpone_count: 0,
    }
  }

  pub fn postpone(&self) -> Rc<dyn UndoneTask> {
    if self.postpone_count < Self::POSTPONE_MAX_COUNT {
      let mut new_task = self.clone();
      new_task.due_date = new_task.due_date + chrono::Duration::days(1);
      new_task.postpone_count += 1;
      Rc::new(new_task)
    } else {
      Rc::new(UndoneTaskWithDeadline::new(
        self.id.clone(),
        self.name.clone(),
        self.due_date.clone(),
      ))
    }
  }
}

impl UndoneTask for PostponeableUndoneTask {
  fn due_date(&self) -> &DateTime<Utc> {
    &self.due_date
  }

  fn done(&self) -> DoneTask {
    DoneTask::new(self.id.clone(), self.name.clone(), self.due_date.clone(), Utc::now())
  }
}

// --- UndoneTaskWithDeadline

pub struct UndoneTaskWithDeadline {
  id: TaskId,
  name: TaskName,
  status: TaskStatus,
  due_date: DateTime<Utc>,
}

impl UndoneTaskWithDeadline {
  pub fn new(id: TaskId, name: TaskName, due_date: DateTime<Utc>) -> Self {
    Self {
      id,
      name,
      status: TaskStatus::Undone,
      due_date,
    }
  }
}

impl Aggregate for UndoneTaskWithDeadline {
  type ID = TaskId;

  fn id(&self) -> &Self::ID {
    &self.id
  }
}

impl Task for UndoneTaskWithDeadline {
  fn name(&self) -> &TaskName {
    &self.name
  }

  fn status(&self) -> &TaskStatus {
    &self.status
  }
}

impl UndoneTask for UndoneTaskWithDeadline {
  fn due_date(&self) -> &DateTime<Utc> {
    &self.due_date
  }

  fn done(&self) -> DoneTask {
    DoneTask::new(self.id.clone(), self.name.clone(), self.due_date.clone(), Utc::now())
  }
}

// --- DoneTask

#[derive(Debug, Clone)]
pub struct DoneTask {
  id: TaskId,
  name: TaskName,
  status: TaskStatus,
  due_date: DateTime<Utc>,
  done_date: DateTime<Utc>,
}

impl DoneTask {
  pub fn new(id: TaskId, name: TaskName, due_date: DateTime<Utc>, done_date: DateTime<Utc>) -> Self {
    Self {
      id,
      name,
      status: TaskStatus::Done,
      due_date,
      done_date,
    }
  }

  pub fn done_date(&self) -> &DateTime<Utc> {
    &self.done_date
  }
}

impl Aggregate for DoneTask {
  type ID = TaskId;

  fn id(&self) -> &Self::ID {
    &self.id
  }
}

impl Task for DoneTask {
  fn name(&self) -> &TaskName {
    &self.name
  }

  fn status(&self) -> &TaskStatus {
    &self.status
  }
}
