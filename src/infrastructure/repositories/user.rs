use async_trait::async_trait;
use sea_orm::{ActiveModelTrait, DbConn};
use sea_orm::ActiveValue::Set;
use sea_orm::prelude::*;
use crate::domain::error::{CommonError, CommonErrorCode};
use crate::domain::user::user::{User};
use crate::domain::user::user_repository::UserRepository;
use crate::infrastructure::databases::converters::user::restore;
use crate::infrastructure::databases::entities::{user, user_rsa_key};

pub struct UserSeaORMRepository {
    db_conn: DbConn,
}

impl UserSeaORMRepository {
    pub fn new(db_conn: DbConn) -> Self {
        UserSeaORMRepository {
            db_conn
        }
    }
}

#[async_trait]
impl UserRepository for UserSeaORMRepository {
    async fn add(&self, new_user: &User) -> Result<(), CommonError> {
        match user::ActiveModel::from(new_user).insert(&self.db_conn).await {
            Ok(_) => {}
            Err(e) => {
                log::error!("Failed to insert user: {}", e.to_string());
                return Err(CommonError::new(CommonErrorCode::DBError));
            }
        };

        match user_rsa_key::ActiveModel::from(new_user).insert(&self.db_conn).await {
            Ok(_) => {}
            Err(e) => {
                log::error!("Failed to insert user rsa key: {}", e.to_string());
                return Err(CommonError::new(CommonErrorCode::DBError));
            }
        }

        Ok(())
    }

    async fn list(&self) -> Result<Vec<User>, CommonError> {
        let result = user::Entity::find()
            .find_also_related(user_rsa_key::Entity)
            .all(&self.db_conn)
            .await;
        let users = match result {
            Ok(l) => l,
            Err(e) => {
                log::error!("Failed to list users: {}", e);
                return Err(CommonError::new(CommonErrorCode::DBError));
            }
        };
        users.iter()
            .map(|(u, k)| -> Result<User, CommonError> {
                let key_pair = match k {
                    Some(rsa) => rsa,
                    None => {
                        log::error!("User rsa key does not exists");
                        return Err(CommonError::new(CommonErrorCode::UnexpectedError));
                    }
                };
                Ok(restore(u, key_pair))
            })
            .collect()
    }

    async fn get(&self, user_id: &str) -> Result<User, CommonError> {
        let result = user::Entity::find_by_id(user_id)
            .find_also_related(user_rsa_key::Entity)
            .one(&self.db_conn)
            .await;

        let (user, key_pair) = match result {
            Ok(u) => match u {
                Some(p) => p,
                None => return Err(CommonError::new(CommonErrorCode::UserDoesNotExists)),
            }
            Err(e) => {
                log::error!("Failed to get user: {}", e.to_string());
                return Err(CommonError::new(CommonErrorCode::DBError));
            }
        };

        match key_pair {
            Some(k) => Ok(restore(&user, &k)),
            None => {
                log::error!("User rsa key does not exists");
                return Err(CommonError::new(CommonErrorCode::UnexpectedError));
            }
        }
    }

    async fn update(&self, user: &User) -> Result<(), CommonError> {
        let target = match user::Entity::find_by_id(&user.id).one(&self.db_conn).await {
            Ok(r) => match r {
                Some(t) => t,
                None => {
                    log::error!("Specified user does not exists");
                    return Err(CommonError::new(CommonErrorCode::UnexpectedError));
                }
            },
            Err(e) => {
                log::error!("Failed to get user: {}", e.to_string());
                return Err(CommonError::new(CommonErrorCode::DBError));
            }
        };
        let mut target: user::ActiveModel = target.into();

        // set all columns
        target.username = Set((&user.username).clone());
        target.display_name = Set((&user.display_name).clone());
        target.updated_at = Set((&user.updated_at).clone());

        // update
        match target.update(&self.db_conn).await {
            Ok(_) => Ok(()),
            Err(e) => {
                log::error!("Failed to update user: {}", e.to_string());
                return Err(CommonError::new(CommonErrorCode::DBError));
            }
        }
    }

    async fn delete(&self, user_id: &str) -> Result<(), CommonError> {
        match user_rsa_key::Entity::delete_by_id(user_id).exec(&self.db_conn).await {
            Ok(_) => {}
            Err(e) => {
                log::error!("Failed to delete user rsa key: {}", e.to_string());
                return Err(CommonError::new(CommonErrorCode::DBError));
            }
        }

        user::Entity::delete_by_id(user_id).exec(&self.db_conn)
            .await
            .map(|_| ())
            .map_err(|e| {
                log::error!("Failed to delete user: {}", e.to_string());
                CommonError::new(CommonErrorCode::DBError)
            })
    }

    async fn find(&self, username: &str) -> Result<Option<User>, CommonError> {
        let result = user::Entity::find()
            .find_also_related(user_rsa_key::Entity)
            .filter(user::Column::Username.eq(username))
            .one(&self.db_conn)
            .await;

        let (user, key_pair) = match result {
            Ok(u) => match u {
                Some(p) => p,
                None => return Ok(None),
            }
            Err(e) => {
                log::error!("Failed to find user: {}", e.to_string());
                return Err(CommonError::new(CommonErrorCode::DBError));
            }
        };

        match key_pair {
            Some(k) => Ok(Some(restore(&user, &k))),
            None => {
                log::error!("User rsa key does not exists");
                return Err(CommonError::new(CommonErrorCode::UnexpectedError));
            }
        }
    }
}
