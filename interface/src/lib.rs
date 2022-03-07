extern crate rust_ca_domain;
extern crate rust_ca_use_case;

mod task_controller;

pub use task_controller::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
