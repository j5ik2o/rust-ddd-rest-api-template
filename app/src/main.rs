extern crate rust_ca_infrastructure;
extern crate rust_ca_use_case;

use std::sync::{Arc, Mutex};

use actix_web::{App, HttpServer, web};

use rust_ca_infrastructure::TaskRepositoryInMemory;
use rust_ca_use_case::CreateTaskInteractor;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let repository = Arc::new(Mutex::new(TaskRepositoryInMemory::new()));
    let create_task_iteractor = CreateTaskInteractor::new(repository);
    let data = web::Data::new(create_task_iteractor);

    HttpServer::new(move || App::new().app_data(data.clone()).service(rust_ca_interface::create_task))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
