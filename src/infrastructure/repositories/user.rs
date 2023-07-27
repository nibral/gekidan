use async_trait::async_trait;
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, DbConn};
use sea_orm::prelude::*;
use crate::domain::error::{CommonError, CommonErrorCode};
use crate::domain::models::user::User;
use crate::domain::models::user_rsa_key::UserRsaKey;
use crate::domain::repositories::user::UserRepository;
use crate::domain::services::rsa_key::RsaKeyService;
use crate::infrastructure::entities::{user, user_rsa_key};

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
        let finder = user::Entity::find()
            .filter(user::Column::Username.eq(new_user.username.clone()))
            .one(&self.db_conn)
            .await;
        match finder {
            Ok(r) => match r {
                None => {}
                Some(_) => {
                    return Err(CommonError::new(CommonErrorCode::UsernameAlreadyUsed));
                }
            }
            Err(e) => {
                log::error!("Unexpected DB error in user search: {}", e.to_string());
                return Err(CommonError::new(CommonErrorCode::UnexpectedDBError));
            }
        }

        let user = user::ActiveModel {
            id: Set(new_user.id.clone()),
            username: Set(new_user.username.clone()),
            display_name: Set(new_user.display_name.clone()),
            created_at: Set(new_user.created_at.clone()),
            updated_at: Set(new_user.updated_at.clone()),
        };

        // add user
        let result: User = match user.insert(&self.db_conn).await {
            Ok(u) => u.into(),
            Err(e) => {
                log::error!("Unexpected DB error in user creation: {}", e.to_string());
                return Err(CommonError::new(CommonErrorCode::UnexpectedDBError));
            }
        };

        // generate and store user's rsa key
        let (private_pem, public_pem) = RsaKeyService::generate_key_pair();
        let user_key_pair = user_rsa_key::ActiveModel {
            user_id: Set(result.id.clone()),
            private_key: Set(private_pem),
            public_key: Set(public_pem),
        };
        match user_key_pair.insert(&self.db_conn).await {
            Ok(_) => {}
            Err(e) => {
                log::error!("Unexpected DB error in store rsa key pair: {}", e.to_string());
                return Err(CommonError::new(CommonErrorCode::UnexpectedDBError));
            }
        }

        Ok(result)
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

    async fn get(&self, user_id: String) -> Result<User, CommonError> {
        let result = user::Entity::find_by_id(&user_id).one(&self.db_conn)
            .await;

        match result {
            Ok(user) => match user {
                Some(u) => Ok(u.into()),
                _ => Err(CommonError::new(CommonErrorCode::UserDoesNotExists)),
            },
            Err(e) => {
                log::error!("Unexpected DB Error: {}", e.to_string());
                Err(CommonError::new(CommonErrorCode::UnexpectedDBError))
            }
        }
    }

    async fn delete(&self, user_id: String) -> Result<(), CommonError> {
        match user_rsa_key::Entity::delete_by_id(&user_id).exec(&self.db_conn).await {
            Ok(_) => {}
            Err(e) => {
                log::error!("Unexpected DB error in delete user key pair: {}", e.to_string());
                return Err(CommonError::new(CommonErrorCode::UnexpectedDBError));
            }
        }

        user::Entity::delete_by_id(&user_id).exec(&self.db_conn)
            .await
            .map(|_| ())
            .map_err(|e| {
                log::error!("Unexpected DB error in user deletion: {}", e.to_string());
                CommonError::new(CommonErrorCode::UnexpectedDBError)
            })
    }

    async fn find_by_username_with_rsa_key(&self, username: String) -> Result<(User, UserRsaKey), CommonError> {
        let result = user::Entity::find()
            .find_also_related(user_rsa_key::Entity)
            .filter(user::Column::Username.eq(username))
            .one(&self.db_conn)
            .await;

        let (user, key_pair) = match result {
            Ok(u) => match u {
                Some(pair) => pair,
                _ => return Err(CommonError::new(CommonErrorCode::UserDoesNotExists))
            }
            Err(e) => {
                log::error!("Unexpected DB Error: {}", e.to_string());
                return Err(CommonError::new(CommonErrorCode::UnexpectedDBError));
            }
        };

        match key_pair {
            Some(k) => Ok((user.into(), k.into())),
            _ => {
                log::error!("Unexpected DB state: user rsa key pair not found");
                Err(CommonError::new(CommonErrorCode::UserDoesNotExists))
            }
        }
    }
}
