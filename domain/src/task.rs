use crate::Aggregate;
use chrono::{Date, Local};

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub enum TaskStatus {
  Undone,
  Done,
}

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub struct TaskId(pub u64);

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub struct TaskName(pub String);

pub trait Task: Aggregate {
  fn name(&self) -> &TaskName;
  fn status(&self) -> &TaskStatus;
}

pub trait UndoneTask: Task {
  fn due_date(&self) -> &Date<Local>;
  fn done(&self) -> DoneTask;
}

// --- PostponeableUndoneTask

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub struct PostponeableUndoneTask {
  id: TaskId,
  name: TaskName,
  status: TaskStatus,
  due_date: Date<Local>,
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

  pub fn new(id: TaskId, name: TaskName, due_date: Date<Local>) -> Self {
    Self {
      id,
      name,
      status: TaskStatus::Undone,
      due_date,
      postpone_count: 0,
    }
  }

  fn postpone(&self) -> Box<dyn UndoneTask<ID = TaskId>> {
    if self.postpone_count < Self::POSTPONE_MAX_COUNT {
      let mut r = self.clone();
      r.due_date = r.due_date + chrono::Duration::days(1);
      r.postpone_count += 1;
      Box::new(r)
    } else {
      Box::new(UndoneTaskWithDeadline::new(
        self.id.clone(),
        self.name.clone(),
        self.due_date.clone(),
      ))
    }
  }
}

impl UndoneTask for PostponeableUndoneTask {
  fn due_date(&self) -> &Date<Local> {
    &self.due_date
  }

  fn done(&self) -> DoneTask {
    DoneTask::new(
      self.id.clone(),
      self.name.clone(),
      self.due_date.clone(),
      Local::today(),
    )
  }
}

// --- UndoneTaskWithDeadline

pub struct UndoneTaskWithDeadline {
  id: TaskId,
  name: TaskName,
  status: TaskStatus,
  due_date: Date<Local>,
}

impl UndoneTaskWithDeadline {
  pub fn new(id: TaskId, name: TaskName, due_date: Date<Local>) -> Self {
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
  fn due_date(&self) -> &Date<Local> {
    &self.due_date
  }

  fn done(&self) -> DoneTask {
    DoneTask::new(
      self.id.clone(),
      self.name.clone(),
      self.due_date.clone(),
      Local::today(),
    )
  }
}

// --- DoneTask

#[derive(Debug, Clone)]
pub struct DoneTask {
  id: TaskId,
  name: TaskName,
  status: TaskStatus,
  due_date: Date<Local>,
  done_date: Date<Local>,
}

impl DoneTask {
  pub fn new(id: TaskId, name: TaskName, due_date: Date<Local>, done_date: Date<Local>) -> Self {
    Self {
      id,
      name,
      status: TaskStatus::Done,
      due_date,
      done_date,
    }
  }

  pub fn done_date(&self) -> &Date<Local> {
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
