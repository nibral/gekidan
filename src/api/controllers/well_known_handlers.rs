use std::sync::Arc;
use actix_web::{HttpResponse, Responder};
use actix_web::web::Data;
use crate::container::Container;

pub async fn host_meta(
    container: Data<Arc<Container>>
) -> impl Responder {
    let app_url = &container.app_config_service.get_app_config().app_url;
    let body = r#"<?xml version="1.0"?>
<XRD xmlns="http://docs.oasis-open.org/ns/xri/xrd-1.0">
    <Link rel="lrdd" type="application/xrd+xml" template="{APP_URL}.well-known/webfinger?resource={uri}" />
</XRD>"#.replace("APP_URL", app_url);

    HttpResponse::Ok()
        .content_type("application/xml")
        .body(body)
}
