use actix_web::{HttpResponse, Responder};

#[post("/tasks")]
pub async fn create_task() -> impl Responder {
   HttpResponse::Ok().body("Hello World!")
}