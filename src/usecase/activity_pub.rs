use std::sync::Arc;
use crate::domain::activity_pub::activity_pub::{NodeInfo, NodeInfoLinks, WebFinger};
use crate::domain::activity_pub::activity_pub_service::ActivityPubService;
use crate::domain::app_config::AppConfig;

pub struct ActivityPubUseCase {
    app_url: String,
    activity_pub_service: Arc<ActivityPubService>,
}

impl ActivityPubUseCase {
    pub fn new(
        app_config: Arc<AppConfig>,
        activity_pub_service: Arc<ActivityPubService>,
    ) -> Self {
        ActivityPubUseCase {
            app_url: app_config.app_url.clone(),
            activity_pub_service,
        }
    }

    pub async fn host_meta(&self) -> String {
        self.activity_pub_service.host_meta(&self.app_url).await
    }

    pub async fn web_finger(&self, params: &WebFingerParams) -> Result<WebFinger, ()> {
        self.activity_pub_service.web_finger(&params.resource, &self.app_url).await
    }

    pub async fn node_info_links(&self) -> NodeInfoLinks {
        self.activity_pub_service.node_info_links(&self.app_url).await
    }

    pub async fn node_info(&self) -> NodeInfo {
        self.activity_pub_service.node_info().await
    }
}

pub struct WebFingerParams {
    pub resource: String,
}
