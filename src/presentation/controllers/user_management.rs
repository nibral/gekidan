use std::sync::Arc;
use actix_web::web::{Data, Json, Path};
use serde::{Deserialize, Serialize};
use crate::app::container::Container;
use crate::domain::user::user::User;
use crate::presentation::errors::api::ApiError;
use crate::presentation::extractors::admin_claim::AdminClaim;
use crate::usecase::user_management::CreateUserParams;

pub async fn create_user(
    _: AdminClaim,
    container: Data<Arc<Container>>,
    post_data: Json<CreateUserRequest>,
) -> Result<Json<UserResponse>, ApiError> {
    let usecase = &container.user_management_usecase;
    let user = usecase.create(&post_data.into_inner().into()).await?;
    Ok(Json(user.into()))
}

pub async fn list_users(
    _: AdminClaim,
    container: Data<Arc<Container>>,
) -> Result<Json<UserListResponse>, ApiError> {
    let usecase = &container.user_management_usecase;
    let users = usecase.list().await?;
    Ok(Json(UserListResponse::from(users)))
}

pub async fn get_user(
    _: AdminClaim,
    container: Data<Arc<Container>>,
    params: Path<String>,
) -> Result<Json<UserResponse>, ApiError> {
    let usecase = &container.user_management_usecase;
    let user = usecase.get(&params.into_inner()).await?;
    Ok(Json(user.into()))
}

pub async fn delete_user(
    _: AdminClaim,
    container: Data<Arc<Container>>,
    params: Path<String>,
) -> Result<String, ApiError> {
    let usecase = &container.user_management_usecase;
    usecase.delete(&params.into_inner()).await?;
    Ok("ok".to_string())
}

#[derive(Serialize, Deserialize)]
pub struct UserResponse {
    pub id: String,
    pub username: String,
    pub display_name: String,
}

impl From<User> for UserResponse {
    fn from(value: User) -> Self {
        UserResponse {
            id: value.id,
            username: value.username,
            display_name: value.display_name,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct UserListResponse {
    pub users: Vec<UserResponse>,
}

impl From<Vec<User>> for UserListResponse {
    fn from(value: Vec<User>) -> Self {
        UserListResponse {
            users: value.iter().map(|u| u.clone().into()).collect(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct CreateUserRequest {
    pub username: String,
    pub display_name: String,
}

impl Into<CreateUserParams> for CreateUserRequest {
    fn into(self) -> CreateUserParams {
        CreateUserParams {
            username: self.username,
            display_name: self.display_name,
        }
    }
}
