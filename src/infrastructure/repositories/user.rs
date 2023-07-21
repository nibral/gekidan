use async_trait::async_trait;
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, DbConn};
use sea_orm::prelude::*;
use crate::domain::error::{CommonError, CommonErrorCode};
use crate::domain::models::user::User;
use crate::domain::repositories::user::UserRepository;
use crate::infrastructure::entities::user;

pub struct UserSeaORMRepository {
    pub db_conn: DbConn,
}

impl UserSeaORMRepository {
    pub fn new(db_conn: DbConn) -> Self {
        UserSeaORMRepository { db_conn }
    }
}

#[async_trait]
impl UserRepository for UserSeaORMRepository {
    async fn create(&self, new_user: &User) -> Result<User, CommonError> {
        let user = user::ActiveModel {
            id: Set(new_user.id.clone()),
            username: Set(new_user.username.clone()),
            display_name: Set(new_user.display_name.clone()),
            created_at: Set(new_user.created_at.clone()),
            updated_at: Set(new_user.updated_at.clone()),
        };

        user.insert(&self.db_conn)
            .await
            .map(|u| u.into())
            .map_err(|e| {
                log::error!("Unexpected DB Error: {}", e.to_string());
                CommonError::new(CommonErrorCode::UnexpectedDBError)
            })
    }

    async fn list(&self) -> Result<Vec<User>, CommonError> {
        user::Entity::find().all(&self.db_conn)
            .await
            .map(|l| l.iter()
                .map(|u| -> User { u.clone().into() })
                .collect()
            )
            .map_err(|e| {
                log::error!("Unexpected DB Error: {}", e.to_string());
                CommonError::new(CommonErrorCode::UnexpectedDBError)
            })
    }

    async fn get(&self, user_id: String) -> Result<Option<User>, CommonError> {
        let result = user::Entity::find_by_id(&user_id).one(&self.db_conn)
            .await;

        match result {
            Ok(user) => match user {
                Some(u) => Ok(Some(u.into())),
                _ => Ok(None),
            },
            Err(e) => {
                log::error!("Unexpected DB Error: {}", e.to_string());
                Err(CommonError::new(CommonErrorCode::UnexpectedDBError))
            }
        }
    }

    async fn delete(&self, user_id: String) -> Result<(), CommonError> {
        user::Entity::delete_by_id(&user_id).exec(&self.db_conn)
            .await
            .map(|_| ())
            .map_err(|e| {
                log::error!("Unexpected DB Error: {}", e.to_string());
                CommonError::new(CommonErrorCode::UnexpectedDBError)
            })
    }
}
