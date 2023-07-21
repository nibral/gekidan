use std::sync::Arc;
use actix_web::{App, Error, middleware, web};
use actix_web::body::MessageBody;
use actix_web::dev::{ServiceFactory, ServiceRequest, ServiceResponse};
use crate::api::controllers::*;
use crate::domain::repositories::user::UserRepository;
use crate::domain::services::app_config::AppConfigService;
use crate::domain::services::user::UserService;
use crate::infrastructure::databases::sqlite3::db_conn;
use crate::infrastructure::repositories::user::UserSeaORMRepository;
use crate::infrastructure::services::app_config::AppConfigServiceImpl;
use crate::services::user::UserServiceImpl;

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

        // DI
        .data_factory(|| {
            async {
                let app_config_service = Arc::new(
                    AppConfigServiceImpl::new().await
                );
                Ok::<Arc<dyn AppConfigService>, ()>(app_config_service)
            }
        })
        .data_factory(|| {
            async {
                let user_repository: Arc<dyn UserRepository> = Arc::new(
                    UserSeaORMRepository::new(db_conn().await)
                );
                let user_service = Arc::new(
                    UserServiceImpl { user_repository }
                );
                Ok::<Arc<dyn UserService>, ()>(user_service)
            }
        })

        // Logger
        .wrap(middleware::Logger::default())

        // Routes
        .service(
            web::scope("/").route("", web::get().to(root_handlers::echo_ok))
        )
        .service(
            web::scope("/.well-known")
                .route("/host-meta", web::get().to(well_known_handlers::host_meta))
        )
        .service(
            web::scope("/users")
                .route("", web::get().to(user_handlers::list_users_handler))
                .route("", web::post().to(user_handlers::create_user_handler))
                .route("/{user_id}", web::get().to(user_handlers::get_user_handler))
                .route("/{user_id}", web::delete().to(user_handlers::delete_user_handler))
        )
}
