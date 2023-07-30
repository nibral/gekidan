use std::sync::Arc;
use crate::domain::error::{CommonError, CommonErrorCode};
use crate::domain::user::user::User;
use crate::domain::user::user_repository::UserRepository;
use crate::domain::user::user_service::UserService;

pub struct UserManagementUseCase {
    user_repository: Arc<dyn UserRepository>,
    user_service: Arc<UserService>,
}

impl UserManagementUseCase {
    pub fn new(
        user_repository: Arc<dyn UserRepository>,
        user_service: Arc<UserService>,
    ) -> Self {
        UserManagementUseCase {
            user_repository,
            user_service,
        }
    }

    pub async fn create(&self, params: &CreateUserParams) -> Result<User, CommonError> {
        // username check
        let check = match self.user_service.is_username_used(&params.username).await {
            Ok(r) => r,
            Err(e) => return Err(e),
        };
        if check {
            return Err(CommonError::new(CommonErrorCode::UsernameAlreadyExists));
        }

        let new_user = User::new(&params.username, &params.display_name);
        self.user_repository
            .add(&new_user)
            .await
            .map(|_| new_user)
    }

    pub async fn list(&self) -> Result<Vec<User>, CommonError> {
        self.user_repository
            .list()
            .await
    }

    pub async fn get(&self, user_id: &str) -> Result<User, CommonError> {
        self.user_repository
            .get(user_id)
            .await
    }

    pub async fn delete(&self, user_id: &str) -> Result<(), CommonError> {
        self.user_repository
            .delete(user_id)
            .await
    }
}

pub struct CreateUserParams {
    pub username: String,
    pub display_name: String,
}
