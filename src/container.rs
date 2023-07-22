use std::sync::Arc;
use crate::domain::repositories::user::UserRepository;
use crate::domain::services::app_config::AppConfigService;
use crate::domain::services::user::UserService;
use crate::infrastructure::databases::sqlite3::db_conn;
use crate::infrastructure::repositories::user::UserSeaORMRepository;
use crate::infrastructure::services::app_config::AppConfigServiceImpl;
use crate::services::user::UserServiceImpl;

pub struct Container {
    pub app_config_service: Arc<dyn AppConfigService>,
    pub user_service: Arc<dyn UserService>,
}

impl Container {
    pub async fn new() -> Self {
        let app_config_service = Arc::new(
            AppConfigServiceImpl::new().await
        );

        let user_repository: Arc<dyn UserRepository> = Arc::new(
            UserSeaORMRepository::new(db_conn().await)
        );
        let user_service = Arc::new(
            UserServiceImpl { user_repository }
        );

        Container {
            app_config_service,
            user_service,
        }
    }
}
