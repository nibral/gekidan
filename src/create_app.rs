use std::sync::Arc;
use actix_web::{App, Error, middleware, web};
use actix_web::body::MessageBody;
use actix_web::dev::{ServiceFactory, ServiceRequest, ServiceResponse};
use crate::api::controllers::*;
use crate::container::Container;
use crate::domain::services::user::UserService;

pub fn create_app() -> App<
    impl ServiceFactory<
        ServiceRequest,
        Response=ServiceResponse<impl MessageBody>,
        Config=(),
        InitError=(),
        Error=Error
    >
> {
    App::new()
        .data_factory(|| {
            async {
                let container = Container::new().await;
                let user_service = container.user_service;
                Ok::<Arc<dyn UserService>, ()>(user_service)
            }
        })
        .wrap(middleware::Logger::default())
        .service(
            web::scope("/").route("", web::get().to(root_handlers::echo_ok))
        )
        .service(
            web::scope("/users")
                .route("", web::get().to(user_handlers::list_users_handler))
                .route("", web::post().to(user_handlers::create_user_handler))
                .route("/{user_id}", web::get().to(user_handlers::get_user_handler))
                .route("/{user_id}", web::delete().to(user_handlers::delete_user_handler))
        )
}
