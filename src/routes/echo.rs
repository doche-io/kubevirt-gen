use actix_web::{get, Responder};

#[get("/echo")]
pub async fn echo() -> impl Responder {
  ""
}