use std::sync::Arc;
use actix_web::{HttpResponse, Responder};
use actix_web::web::Data;
use crate::container::Container;

pub async fn host_meta(
    container: Data<Arc<Container>>
) -> impl Responder {
    let body = (&container.activity_pub_service).host_meta().await;
    HttpResponse::Ok()
        .content_type("application/xml")
        .body(body)
}

pub async fn node_info_links(
    container: Data<Arc<Container>>
) -> impl Responder {
    let body = (&container.activity_pub_service).node_info_links().await;
    HttpResponse::Ok()
        .content_type("application/json")
        .body(body)
}

pub async fn node_info(
    container: Data<Arc<Container>>
) -> impl Responder {
    let body = (&container.activity_pub_service).node_info().await;
    HttpResponse::Ok()
        .content_type("application/json")
        .body(body)
}
