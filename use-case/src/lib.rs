#![feature(trait_upcasting)]

#[macro_use]
extern crate mopa;

mod create_task_use_case;
mod postpone_task_use_case;

pub use create_task_use_case::*;
