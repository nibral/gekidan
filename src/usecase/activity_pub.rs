use std::sync::Arc;
use crate::domain::activity_pub::activity_pub::{InboxActivity, NodeInfo, NodeInfoLinks, Person, WebFinger};
use crate::domain::activity_pub::activity_pub_service::ActivityPubService;
use crate::domain::app_config::AppConfig;
use crate::domain::error::{CommonError, CommonErrorCode};
use crate::domain::user::user_repository::UserRepository;

pub struct ActivityPubUseCase {
    app_url: String,
    activity_pub_service: Arc<ActivityPubService>,
    user_repository: Arc<dyn UserRepository>,
}

impl ActivityPubUseCase {
    pub fn new(
        app_config: Arc<AppConfig>,
        activity_pub_service: Arc<ActivityPubService>,
        user_repository: Arc<dyn UserRepository>,
    ) -> Self {
        ActivityPubUseCase {
            app_url: app_config.app_url.clone(),
            activity_pub_service,
            user_repository,
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

    pub async fn actor_by_username(&self, username: &String) -> Result<Person, CommonError> {
        self.activity_pub_service.actor(username, &self.app_url).await
    }

    pub async fn redirect_to_username(&self, user_id: &String) -> Result<String, CommonError> {
        self.activity_pub_service.get_redirect_url_to_username(user_id, &self.app_url).await
    }

    pub async fn process_inbox_activity(&self, user_id: &String, activity: &InboxActivity) -> Result<(), CommonError> {
        let user = match self.user_repository.get(user_id).await {
            Ok(u) => u,
            Err(e) => return Err(e),
        };

        match &*activity.r#type {
            "Follow" => {
                self.activity_pub_service.send_follow_accept(&user, &activity, &self.app_url).await
            }
            "Undo" => {
                // followを削除する処理
                Ok(())
            }
            _ => Err(CommonError::new(CommonErrorCode::UnexpectedError))
        }
    }
}

pub struct WebFingerParams {
    pub resource: String,
}
