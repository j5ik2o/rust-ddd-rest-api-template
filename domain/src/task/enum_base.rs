use chrono::{DateTime, Utc};

use crate::{TaskId, TaskName, TaskStatus};

pub enum Task {
  Undone(UndoneTask),
  Done(DoneTask),
}

impl Task {
  pub fn id(&self) -> &TaskId {
    match self {
      Task::Undone(undone_task) => &undone_task.id(),
      Task::Done(task) => &task.id,
    }
  }

  pub fn name(&self) -> &TaskName {
    match self {
      Task::Undone(undone_task) => undone_task.name(),
      Task::Done(task) => &task.name,
    }
  }

  pub fn status(&self) -> &TaskStatus {
    match self {
      Task::Undone(undone_task) => undone_task.status(),
      Task::Done(task) => &task.status,
    }
  }

  pub fn as_undone_task(&self) -> Option<&UndoneTask> {
    match self {
      Task::Undone(undone_task) => Some(&undone_task),
      _ => None,
    }
  }
}

pub enum UndoneTask {
  PostponeableUndone(PostponeableUndoneTask),
  UndoneWithDeadline(UndoneTaskWithDeadline),
}

impl UndoneTask {
  pub fn id(&self) -> &TaskId {
    match self {
      UndoneTask::PostponeableUndone(task) => &task.id,
      UndoneTask::UndoneWithDeadline(task) => &task.id,
    }
  }

  pub fn name(&self) -> &TaskName {
    match self {
      UndoneTask::PostponeableUndone(task) => &task.name,
      UndoneTask::UndoneWithDeadline(task) => &task.name,
    }
  }

  pub fn status(&self) -> &TaskStatus {
    match self {
      UndoneTask::PostponeableUndone(task) => &task.status,
      UndoneTask::UndoneWithDeadline(task) => &task.status,
    }
  }

  pub fn due_date(&self) -> &DateTime<Utc> {
    match self {
      UndoneTask::PostponeableUndone(task) => &task.due_date,
      UndoneTask::UndoneWithDeadline(task) => &task.due_date,
    }
  }

  pub fn done(&self) -> DoneTask {
    match self {
      UndoneTask::PostponeableUndone(task) => {
        DoneTask::new(task.id.clone(), task.name.clone(), task.due_date.clone(), Utc::now())
      }
      UndoneTask::UndoneWithDeadline(task) => {
        DoneTask::new(task.id.clone(), task.name.clone(), task.due_date.clone(), Utc::now())
      }
    }
  }
}

// --- PostponeableUndoneTask

#[derive(Debug, Clone)]
pub struct PostponeableUndoneTask {
  id: TaskId,
  name: TaskName,
  status: TaskStatus,
  due_date: DateTime<Utc>,
  postpone_count: u32,
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

  pub fn postpone(&self) -> UndoneTask {
    if self.postpone_count < Self::POSTPONE_MAX_COUNT {
      let mut new_task = self.clone();
      new_task.due_date = new_task.due_date + chrono::Duration::days(1);
      new_task.postpone_count += 1;
      UndoneTask::PostponeableUndone(new_task)
    } else {
      UndoneTask::UndoneWithDeadline(UndoneTaskWithDeadline::new(
        self.id.clone(),
        self.name.clone(),
        self.due_date.clone(),
      ))
    }
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

// --- DoneTask

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
