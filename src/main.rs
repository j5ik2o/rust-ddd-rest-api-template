use crate::domain::TaskRepositoryInMemory;
use crate::use_case::CreateTaskInteractor;
use actix_web::{web, App, HttpServer};
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

mod domain;
mod infrastructure;
mod interface;
mod use_case;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  let repository = Arc::new(Mutex::new(TaskRepositoryInMemory::new()));
  let create_task_iteractor = CreateTaskInteractor::new(repository);
  let data = web::Data::new(create_task_iteractor);

  HttpServer::new(move || App::new().app_data(data.clone()).service(interface::create_task))
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
