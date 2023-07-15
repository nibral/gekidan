use async_trait::async_trait;
use chrono::Utc;
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, DbConn};
use sea_orm::prelude::*;
use ulid::Ulid;
use crate::domain::models::user::{CreateUser, User};
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
    async fn create(&self, new_user: &CreateUser) -> Result<User, String> {
        let new_user = new_user.clone();

        let id = Ulid::new().to_string();
        let now = Utc::now();
        let user = user::ActiveModel {
            id: Set(id),
            username: Set(new_user.username),
            display_name: Set(new_user.display_name),
            created_at: Set(now.clone().to_rfc3339()),
            updated_at: Set(now.clone().to_rfc3339()),
        };

        user.insert(&self.db_conn)
            .await
            .map(|u| u.into())
            .map_err(|e| format!("DB Error: {:?}", e))
    }

    async fn list(&self) -> Result<Vec<User>, String> {
        user::Entity::find().all(&self.db_conn)
            .await
            .map(|l| l.iter()
                .map(|u| -> User { u.clone().into() })
                .collect()
            )
            .map_err(|e| format!("DB Error: {:?}", e))
    }

    async fn get(&self, user_id: String) -> Result<User, String> {
        let user = user::Entity::find_by_id(&user_id).one(&self.db_conn)
            .await
            .map_err(|e| format!("DB Error: {:?}", e))?;
        match user {
            Some(user) => Ok(user.into()),
            _ => Err(format!("user id {} does not exist", &user_id))
        }
    }

    async fn delete(&self, user_id: String) -> Result<(), String> {
        user::Entity::delete_by_id(&user_id).exec(&self.db_conn)
            .await
            .map(|_| ())
            .map_err(|e| format!("DB Error: {:?}", e))
    }
}
