use std::sync::Arc;
use crate::domain::repositories::user::UserRepository;
use crate::domain::services::user::UserService;
use crate::infrastructure::databases::sqlite3::db_conn;
use crate::infrastructure::repositories::user::UserSeaORMRepository;
use crate::services::user::UserServiceImpl;

pub struct Container {
    pub user_service: Arc<dyn UserService>,
}

impl Container {
    pub async fn new() -> Self {
        let user_repository: Arc<dyn UserRepository> = Arc::new(
            UserSeaORMRepository::new(db_conn().await)
        );
        let user_service = Arc::new(
            UserServiceImpl { user_repository }
        );
        Container { user_service }
    }
}
