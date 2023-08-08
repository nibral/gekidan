use async_trait::async_trait;
use crate::domain::error::CommonError;
use crate::domain::follower::follower::Follower;

#[async_trait]
pub trait FollowerRepository: Sync + Send {
    async fn add(&self, new_follower: &Follower) -> Result<(), CommonError>;
    async fn list(&self, user_id: &String) -> Result<Vec<Follower>, CommonError>;
    async fn delete(&self, follower_id: i32) -> Result<(), CommonError>;
}
