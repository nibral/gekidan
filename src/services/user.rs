use std::sync::Arc;
use async_trait::async_trait;
use crate::domain::error::CommonError;
use crate::domain::models::user::{CreateUser, User};
use crate::domain::repositories::user::UserRepository;
use crate::domain::services::user::UserService;

pub struct UserServiceImpl {
    pub user_repository: Arc<dyn UserRepository>,
}

impl UserServiceImpl {
    pub fn new(user_repository: Arc<dyn UserRepository>) -> Self {
        UserServiceImpl {
            user_repository,
        }
    }
}

#[async_trait]
impl UserService for UserServiceImpl {
    async fn create(&self, new_user: CreateUser) -> Result<User, CommonError> {
        self.user_repository
            .create(&new_user)
            .await
    }

    async fn list(&self) -> Result<Vec<User>, CommonError> {
        self.user_repository
            .list()
            .await
    }

    async fn get(&self, user_id: String) -> Result<User, CommonError> {
        self.user_repository
            .get(user_id)
            .await
    }

    async fn delete(&self, user_id: String) -> Result<(), CommonError> {
        self.user_repository
            .delete(user_id)
            .await
    }
}
