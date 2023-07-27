use std::sync::Arc;
use actix_web::web::{Data, Json, Path};
use crate::api::dto::user::{CreateUserDTO, ListUsersDTO, UserDTO};
use crate::api::extractors::admin_claim::AdminClaim;
use crate::container::Container;
use crate::infrastructure::error::ApiError;

pub async fn create_user_handler(
    _admin_claim: AdminClaim,
    container: Data<Arc<Container>>, post_data: Json<CreateUserDTO>,
) -> Result<Json<UserDTO>, ApiError> {
    let user_service = &container.user_service;
    let user = user_service.create(post_data.into_inner().into()).await?;
    Ok(Json(user.into()))
}

pub async fn list_users_handler(
    _admin_claim: AdminClaim,
    container: Data<Arc<Container>>,
) -> Result<Json<ListUsersDTO>, ApiError> {
    let user_service = &container.user_service;
    let users = user_service.list().await?;
    Ok(Json(ListUsersDTO::from(users)))
}

pub async fn get_user_handler(
    _admin_claim: AdminClaim,
    container: Data<Arc<Container>>, params: Path<String>,
) -> Result<Json<UserDTO>, ApiError> {
    let user_service = &container.user_service;
    let user = user_service.get(params.into_inner()).await?;
    Ok(Json(user.into()))
}

pub async fn delete_user_handler(
    _admin_claim: AdminClaim,
    container: Data<Arc<Container>>, params: Path<String>,
) -> Result<String, ApiError> {
    let user_service = &container.user_service;
    user_service.delete(params.into_inner()).await?;
    Ok("ok".to_string())
}
