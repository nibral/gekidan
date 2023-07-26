use std::sync::Arc;
use actix_web::{HttpResponse, Responder};
use actix_web::web::{Data, Query};
use serde::Deserialize;
use crate::container::Container;

pub async fn host_meta(
    container: Data<Arc<Container>>
) -> impl Responder {
    let body = (&container.activity_pub_service).host_meta().await;
    HttpResponse::Ok()
        .content_type("application/xml")
        .body(body)
}

#[derive(Deserialize)]
pub struct WebFingerQuery {
    resource: String,
}

pub async fn web_finger(
    container: Data<Arc<Container>>, query: Query<WebFingerQuery>,
) -> impl Responder {
    let resource = query.resource.clone();
    match (&container.activity_pub_service).web_finger(resource).await {
        Ok(body) => HttpResponse::Ok()
            .content_type("application/jrd+json; charset=utf-8")
            .body(body),
        Err(_) => HttpResponse::NotFound().into(),
    }
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
