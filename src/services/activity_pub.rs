use std::sync::Arc;
use serde::Serialize;
use serde_json::json;
use crate::domain::services::activity_pub::ActivityPubService;
use crate::domain::services::app_config::AppConfigService;

const AP_HOST_META_TEMPLATE: &str = &r#"<?xml version="1.0"?>
<XRD xmlns="http://docs.oasis-open.org/ns/xri/xrd-1.0">
<Link rel="lrdd" type="application/xrd+xml" template="https://rust-ap-example.room4.dev/.well-known/webfinger?resource={uri}" />
</XRD>"#;

pub struct ActivityPubServiceImpl {
    pub app_config_service: Arc<dyn AppConfigService>,
}

impl ActivityPubServiceImpl {
    pub fn new(app_config_service: Arc<dyn AppConfigService>) -> Self {
        ActivityPubServiceImpl {
            app_config_service
        }
    }
}

impl ActivityPubService for ActivityPubServiceImpl {
    fn host_meta(&self) -> String {
        let app_url = &self.app_config_service.get_app_config().app_url;
        AP_HOST_META_TEMPLATE.replace("APP_URL", app_url).to_string()
    }

    fn web_finger(&self) -> String {
        todo!()
    }

    fn node_info_links(&self) -> String {
        #[derive(Serialize)]
        struct Links {
            rel: String,
            href: String,
        }

        let app_url = &self.app_config_service.get_app_config().app_url;
        let rel = "http://nodeinfo.diaspora.software/ns/schema/2.1".to_string();
        let href = format!("{}/nodeinfo/2.1", app_url);

        json!(Links{rel, href}).to_string()
    }

    fn node_info(&self) -> String {
        todo!()
    }
}
