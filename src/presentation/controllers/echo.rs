use actix_web::{HttpResponse, Responder};

pub async fn echo_ok() -> impl Responder {
    HttpResponse::Ok().body("ok")
}
