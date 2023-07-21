use async_trait::async_trait;
use crate::domain::error::CommonError;
use crate::domain::models::user::User;

#[async_trait]
pub trait UserRepository: Sync + Send {
    async fn create(&self, new_user: &User) -> Result<User, CommonError>;
    async fn list(&self) -> Result<Vec<User>, CommonError>;
    async fn get(&self, user_id: String) -> Result<Option<User>, CommonError>;
    async fn delete(&self, user_id: String) -> Result<(), CommonError>;
}
