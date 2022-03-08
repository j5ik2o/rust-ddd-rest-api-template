extern crate rust_ca_domain;

mod task_repository_impl;

pub use task_repository_impl::*;

#[cfg(test)]
mod tests {
  #[test]
  fn it_works() {
    let result = 2 + 2;
    assert_eq!(result, 4);
  }
}
