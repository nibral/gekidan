use std::sync::Arc;
use async_trait::async_trait;
use serde::Serialize;
use serde_json::json;
use crate::domain::repositories::user::UserRepository;
use crate::domain::services::activity_pub::ActivityPubService;
use crate::domain::services::app_config::AppConfigService;

const AP_HOST_META_TEMPLATE: &str = &r#"<?xml version="1.0"?>
<XRD xmlns="http://docs.oasis-open.org/ns/xri/xrd-1.0">
<Link rel="lrdd" type="application/xrd+xml" template="https://rust-ap-example.room4.dev/.well-known/webfinger?resource={uri}" />
</XRD>"#;

pub struct ActivityPubServiceImpl {
    pub app_config_service: Arc<dyn AppConfigService>,
    pub user_repository: Arc<dyn UserRepository>,
}

impl ActivityPubServiceImpl {
    pub fn new(
        app_config_service: Arc<dyn AppConfigService>,
        user_repository: Arc<dyn UserRepository>,
    ) -> Self {
        ActivityPubServiceImpl {
            app_config_service,
            user_repository,
        }
    }
}

#[async_trait]
impl ActivityPubService for ActivityPubServiceImpl {
    async fn host_meta(&self) -> String {
        let app_url = &self.app_config_service.get_app_config().app_url;
        AP_HOST_META_TEMPLATE.replace("APP_URL", app_url).to_string()
    }

    async fn web_finger(&self) -> String {
        todo!()
    }

    async fn node_info_links(&self) -> String {
        #[derive(Serialize)]
        struct Links {
            rel: String,
            href: String,
        }

        let app_url = &self.app_config_service.get_app_config().app_url;
        let rel = "http://nodeinfo.diaspora.software/ns/schema/2.1".to_string();
        let href = format!("{}nodeinfo/2.1", app_url);

        json!(Links{rel, href}).to_string()
    }

    async fn node_info(&self) -> String {
        #[derive(Serialize)]
        struct Software {
            name: String,
            version: String,
        }
        #[derive(Serialize)]
        struct Services {
            inbound: Vec<String>,
            outbound: Vec<String>,
        }
        #[derive(Serialize)]
        struct Users {
            total: usize,
        }
        #[derive(Serialize)]
        struct Usage {
            users: Users,
        }
        #[derive(Serialize)]
        struct Metadata {}

        #[derive(Serialize)]
        struct Nodeinfo {
            version: String,
            software: Software,
            protocols: Vec<String>,
            services: Services,
            #[serde(rename(serialize = "openRegistrations"))]
            open_registrations: bool,
            usage: Usage,
            metadata: Metadata,
        }

        let users = (&self.user_repository).list().await.unwrap().len();
        let info = Nodeinfo {
            version: "2.1".to_string(),
            software: Software {
                name: "Gekidan".to_string(),
                version: "0.1".to_string(),
            },
            protocols: vec!["activitypub".to_string()],
            services: Services { inbound: vec![], outbound: vec![] },
            open_registrations: false,
            usage: Usage { users: Users { total: users } },
            metadata: Metadata {},
        };

        json!(info).to_string()
    }
}