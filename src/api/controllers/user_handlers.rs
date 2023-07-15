use std::sync::Arc;
use actix_web::{HttpResponse, Responder};
use actix_web::web::{Data, Json, Path};
use serde_json::json;
use crate::api::dto::user::{CreateUserDTO, ListUsersDTO, UserDTO};
use crate::domain::services::user::UserService;

pub async fn create_user_handler(
    user_service: Data<Arc<dyn UserService>>, post_data: Json<CreateUserDTO>,
) -> impl Responder {
    match user_service.create(post_data.into_inner().into()).await {
        Ok(u) => {
            let dto: UserDTO = u.into();
            HttpResponse::Ok().json(json!(dto))
        }
        Err(e) => HttpResponse::InternalServerError().body(e),
    }
}

pub async fn list_users_handler(
    user_service: Data<Arc<dyn UserService>>
) -> impl Responder {
    match user_service.list().await {
        Ok(l) => {
            let users: Vec<UserDTO> = l.iter().map(|u| u.clone().into()).collect();
            HttpResponse::Ok().json(json!(ListUsersDTO{users}))
        }
        Err(e) => HttpResponse::InternalServerError().body(e),
    }
}

pub async fn get_user_handler(
    user_service: Data<Arc<dyn UserService>>, params: Path<String>,
) -> impl Responder {
    match user_service.get(params.into_inner()).await {
        Ok(u) => {
            let dto: UserDTO = u.into();
            HttpResponse::Ok().json(json!(dto))
        }
        Err(e) => HttpResponse::InternalServerError().body(e),
    }
}

pub async fn delete_user_handler(
    user_service: Data<Arc<dyn UserService>>, params: Path<String>,
) -> impl Responder {
    match user_service.delete(params.into_inner()).await {
        Ok(_) => HttpResponse::Ok().body("ok"),
        Err(e) => HttpResponse::InternalServerError().body(e),
    }
}
