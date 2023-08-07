use std::sync::Arc;
use actix_web::{HttpResponse, Responder, ResponseError};
use actix_web::web::{Data, Json, Path, Query};
use serde::Deserialize;
use serde_json::json;
use crate::app::container::Container;
use crate::domain::activity_pub::activity_pub::{ActivityNoteBox, InboxActivity};
use crate::presentation::errors::api::ApiError;
use crate::usecase::activity_pub::{WebFingerParams};

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

pub async fn actor_by_username(
    container: Data<Arc<Container>>,
    params: Path<String>,
) -> impl Responder {
    match (&container.activity_pub_usecase).actor_by_username(&params.into_inner()).await {
        Ok(p) => HttpResponse::Ok()
            .content_type("application/activity+json; charset=utf-8")
            .body(json!(p).to_string()),
        Err(e) => ApiError::from(e).error_response(),
    }
}

pub async fn actor_by_user_id(
    container: Data<Arc<Container>>,
    params: Path<String>,
) -> impl Responder {
    match (&container.activity_pub_usecase).redirect_to_username(&params.into_inner()).await {
        Ok(l) => HttpResponse::Found()
            .insert_header(("Location", l))
            .body(""),
        Err(e) => ApiError::from(e).error_response(),
    }
}

pub async fn post_inbox(
    container: Data<Arc<Container>>,
    params: Path<String>,
    post_data: Json<InboxActivity>,
) -> impl Responder {
    let activity = post_data.into_inner();
    match (&container.activity_pub_usecase).process_inbox_activity(&params.into_inner(), &activity).await {
        Ok(_) => HttpResponse::Ok().body("ok"),
        Err(e) => ApiError::from(e).error_response(),
    }
}

pub async fn get_outbox() -> impl Responder {
    HttpResponse::Ok()
        .content_type("application/activity+json; charset=utf-8")
        .body(json!(ActivityNoteBox {
            context: "https://www.w3.org/ns/activitystreams".to_string(),
            summary: "outbox".to_string(),
            r#type: "OrderedCollection".to_string(),
            total_items: 0,
            ordered_items: vec![],
        }).to_string())
}

#[derive(Deserialize)]
pub struct WebFingerQuery {
    resource: String,
}
