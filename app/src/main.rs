extern crate rust_ca_infrastructure;
extern crate rust_ca_use_case;

use std::sync::{Arc, Mutex};

use actix_web::{web, App, HttpServer};
use rust_ca_domain::TaskRepository;

use rust_ca_infrastructure::TaskRepositoryInMemory;
use rust_ca_use_case::{CreateTaskInteractor, PostponeTaskInteractor};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  let repository: Arc<Mutex<dyn TaskRepository>> = Arc::new(Mutex::new(TaskRepositoryInMemory::new()));
  let create_task_iteractor = CreateTaskInteractor::new(repository.clone());
  let create_task_iteractor_data = web::Data::new(create_task_iteractor);
  let postpone_task_interator = PostponeTaskInteractor::new(repository);
  let postpone_task_iteractor_data = web::Data::new(postpone_task_interator);

  HttpServer::new(move || {
    App::new()
      .app_data(create_task_iteractor_data.clone())
      .app_data(postpone_task_iteractor_data.clone())
      .service(rust_ca_interface::create_task)
      .service(rust_ca_interface::postpone_task)
  })
  .bind(("127.0.0.1", 8080))?
  .run()
  .await
}
