use std::sync::Arc;
use crate::domain::activity_pub::activity_pub::{InboxActivity, NodeInfo, NodeInfoLinks, Person, WebFinger};
use crate::domain::activity_pub::activity_pub_service::ActivityPubService;
use crate::domain::app_config::AppConfig;
use crate::domain::error::{CommonError, CommonErrorCode};
use crate::domain::follower::follower::Follower;
use crate::domain::follower::follower_repository::FollowerRepository;
use crate::domain::user::user_repository::UserRepository;

pub struct ActivityPubUseCase {
    app_url: String,
    activity_pub_service: Arc<ActivityPubService>,
    user_repository: Arc<dyn UserRepository>,
    follower_repository: Arc<dyn FollowerRepository>,
}

impl ActivityPubUseCase {
    pub fn new(
        app_config: Arc<AppConfig>,
        activity_pub_service: Arc<ActivityPubService>,
        user_repository: Arc<dyn UserRepository>,
        follower_repository: Arc<dyn FollowerRepository>,
    ) -> Self {
        ActivityPubUseCase {
            app_url: app_config.app_url.clone(),
            activity_pub_service,
            user_repository,
            follower_repository,
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
        let user = self.user_repository.get(user_id).await?;

        match &*activity.r#type {
            "Follow" => {
                let follower = Follower::new(
                    &user.id,
                    &activity.actor,
                    &activity.object.object,
                    &format!("{}/inbox", activity.actor)
                );
                self.follower_repository.add(&follower).await?;
                self.activity_pub_service.send_follow_accept(&user, &activity, &self.app_url).await
            }
            "Undo" => {
                let followers = self.follower_repository.list(&user.id).await?;
                for f in followers.iter() {
                    if f.actor == activity.object.actor {
                        self.follower_repository.delete(f.id).await?;
                    }
                }
                Ok(())
            }
            _ => Err(CommonError::new(CommonErrorCode::UnexpectedError))
        }
    }
}

pub struct WebFingerParams {
    pub resource: String,
}
