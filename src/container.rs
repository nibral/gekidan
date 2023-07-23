use std::sync::Arc;
use crate::domain::repositories::user::UserRepository;
use crate::domain::services::activity_pub::ActivityPubService;
use crate::domain::services::app_config::AppConfigService;
use crate::domain::services::user::UserService;
use crate::infrastructure::databases::sqlite3::db_conn;
use crate::infrastructure::repositories::user::UserSeaORMRepository;
use crate::services::activity_pub::ActivityPubServiceImpl;
use crate::services::app_config::AppConfigServiceImpl;
use crate::services::user::UserServiceImpl;

pub struct Container {
    pub activity_pub_service: Arc<dyn ActivityPubService>,
    pub user_service: Arc<dyn UserService>,
}

impl Container {
    pub async fn new() -> Self {
        let app_config_service: Arc<dyn AppConfigService> = Arc::new(
            AppConfigServiceImpl::new().await
        );

        let activity_pub_service: Arc<dyn ActivityPubService> = Arc::new(
            ActivityPubServiceImpl {
                app_config_service,
            }
        );

        let user_repository: Arc<dyn UserRepository> = Arc::new(
            UserSeaORMRepository::new(db_conn().await)
        );
        let user_service: Arc<dyn UserService> = Arc::new(
            UserServiceImpl { user_repository }
        );

        Container {
            activity_pub_service,
            user_service,
        }
    }
}
