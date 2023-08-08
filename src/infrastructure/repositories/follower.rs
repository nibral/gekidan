use async_trait::async_trait;
use sea_orm::{ActiveModelTrait, DbConn};
use sea_orm::prelude::*;
use crate::domain::error::{CommonError, CommonErrorCode};
use crate::domain::follower::follower::Follower;
use crate::domain::follower::follower_repository::FollowerRepository;
use crate::infrastructure::databases::entities::follower;

pub struct FollowerSeaORMRepository {
    db_conn: DbConn,
}

impl FollowerSeaORMRepository {
    pub fn new(db_conn: DbConn) -> Self {
        FollowerSeaORMRepository {
            db_conn
        }
    }
}

#[async_trait]
impl FollowerRepository for FollowerSeaORMRepository {
    async fn add(&self, new_follower: &Follower) -> Result<(), CommonError> {
        match follower::ActiveModel::from(new_follower).insert(&self.db_conn).await {
            Ok(_) => Ok(()),
            Err(e) => {
                log::error!("Failed to insert follower: {}", e.to_string());
                return Err(CommonError::new(CommonErrorCode::DBError));
            }
        }
    }

    async fn list(&self, user_id: &String) -> Result<Vec<Follower>, CommonError> {
        let result = follower::Entity::find()
            .filter(follower::Column::UserId.eq(user_id))
            .all(&self.db_conn)
            .await;
        match result {
            Ok(l) => Ok(l.iter().map(|f| -> Follower { f.clone().into() }).collect()),
            Err(e) => {
                log::error!("Failed to list follower: {}", e.to_string());
                Err(CommonError::new(CommonErrorCode::DBError))
            }
        }
    }

    async fn delete(&self, follower_id: i32) -> Result<(), CommonError> {
        match follower::Entity::delete_by_id(follower_id).exec(&self.db_conn).await {
            Ok(_) => Ok(()),
            Err(e) => {
                log::error!("Failed to delete follower: {}", e.to_string());
                Err(CommonError::new(CommonErrorCode::DBError))
            }
        }
    }
}
