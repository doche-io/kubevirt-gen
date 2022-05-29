use crate::routes::virtual_machine;
use crate::routes::service;
use actix_web::{post, web, HttpResponse, Responder};
use serde_yaml;

#[post("/virtualmachine")]
pub async fn vm(data: web::Json<virtual_machine::VirtualMachineRequest>) -> impl Responder {
  match serde_yaml::to_string(&virtual_machine::Vm::new(data.0)) {
    Ok(v) => {
      let mut res = v
        .replacen("\\n", "\n                ", 4)
        .replace("userData: ", "userData: |\n                ")
        .replace("\"#cloud", "#cloud");
      res.pop();
      res.pop();
      res = res
        .trim_end()
        .to_string()
        .replace(
          "ssh_authorized_keys: ",
          "ssh_authorized_keys: \n                  ",
        )
        .replacen("[\\\"", "- ", 9999)
        .replacen("\\\"]", "\n                  ", 9999)
        .replacen("\\\",", "\n                  -", 9999)
        .replacen("\\\"", "", 9999)
        .trim_end()
        .to_string();
      HttpResponse::Ok().body(res)
    }
    Err(e) => {
      HttpResponse::BadRequest().body(format!("{{\"status\": \"Error\", \"reason\": \"{}\"}}", e))
    }
  }
}

#[post("/service")]
pub async fn ser(data: web::Json<service::ServiceRequest>) -> impl Responder {
  match serde_yaml::to_string(&service::Service::new(data.0)) {
    Ok(v) => {
      HttpResponse::Ok().body(v)
    },
    Err(e) => {
      HttpResponse::BadRequest().body(format!("{{\"status\": \"Error\", \"reason\": \"{}\"}}", e))
    }
  }
}