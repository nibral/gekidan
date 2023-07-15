use async_trait::async_trait;
use crate::domain::models::user::{CreateUser, User};

#[async_trait]
pub trait UserService: Sync + Send {
    async fn create(&self, new_user: CreateUser) -> Result<User, String>;
    async fn list(&self) -> Result<Vec<User>, String>;
    async fn get(&self, user_id: String) -> Result<User, String>;
    async fn delete(&self, user_id: String) -> Result<(), String>;
}
