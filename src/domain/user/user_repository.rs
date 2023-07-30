use async_trait::async_trait;
use crate::domain::error::CommonError;
use crate::domain::user::user::User;

#[async_trait]
pub trait UserRepository: Sync + Send {
    async fn add(&self, new_user: &User) -> Result<(), CommonError>;
    async fn list(&self) -> Result<Vec<User>, CommonError>;
    async fn get(&self, user_id: &str) -> Result<User, CommonError>;
    async fn delete(&self, user_id: &str) -> Result<(), CommonError>;
    async fn find(&self, username: &str) -> Result<Option<User>, CommonError>;
}
