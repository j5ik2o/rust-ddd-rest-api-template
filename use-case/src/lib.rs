#![feature(trait_upcasting)]

use thiserror::Error;

pub use create_task_use_case::*;
pub use postpone_task_use_case::*;

mod create_task_use_case;
mod postpone_task_use_case;

#[derive(Error, Debug)]
pub enum TaskUseCaseError {
  #[error("An error has occurred in the repository.")]
  RepositoryError,
  #[error("The task is not in the state it is supposed to be in.")]
  StateError,
}
