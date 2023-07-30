use std::sync::Arc;
use actix_web::{HttpResponse, Responder};
use actix_web::web::{Data, Query};
use serde::Deserialize;
use serde_json::json;
use crate::app::container::Container;
use crate::usecase::activity_pub::WebFingerParams;

pub async fn host_meta(
    container: Data<Arc<Container>>
) -> impl Responder {
    let body = (&container.activity_pub_usecase).host_meta().await;
    HttpResponse::Ok()
        .content_type("application/xml")
        .body(body)
}

pub async fn web_finger(
    container: Data<Arc<Container>>,
    query: Query<WebFingerQuery>,
) -> impl Responder {
    let params = WebFingerParams { resource: query.resource.clone() };
    match (&container.activity_pub_usecase).web_finger(&params).await {
        Ok(body) => HttpResponse::Ok()
            .content_type("application/jrd+json; charset=utf-8")
            .body(json!(body).to_string()),
        Err(_) => HttpResponse::NotFound().into(),
    }
}

pub async fn node_info_links(
    container: Data<Arc<Container>>,
) -> impl Responder {
    let body = (&container.activity_pub_usecase).node_info_links().await;
    HttpResponse::Ok().json(body)
}

pub async fn node_info(
    container: Data<Arc<Container>>,
) -> impl Responder {
    let body = (&container.activity_pub_usecase).node_info().await;
    HttpResponse::Ok().json(body)
}

#[derive(Deserialize)]
pub struct WebFingerQuery {
    resource: String,
}
