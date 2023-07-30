use std::sync::Arc;
use crate::domain::error::CommonError;
use crate::domain::user::user_repository::UserRepository;

pub struct UserService {
    user_repository: Arc<dyn UserRepository>,
}

impl UserService {
    pub fn new(user_repository: Arc<dyn UserRepository>) -> Self {
        UserService {
            user_repository,
        }
    }

    pub async fn is_username_used(&self, username: &str) -> Result<bool, CommonError> {
        let finder = self.user_repository.find(username).await;
        match finder {
            Ok(res) => match res {
                Some(_) => Ok(true),
                None => Ok(false),
            }
            Err(e) => Err(e)
        }
    }
}
