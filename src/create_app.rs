use std::sync::Arc;
use actix_web::{App, Error, middleware, web};
use actix_web::body::MessageBody;
use actix_web::dev::{ServiceFactory, ServiceRequest, ServiceResponse};
use crate::api::controllers::*;
use crate::container::Container;

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
                let container = Container::new().await;
                Ok::<Arc<Container>, ()>(Arc::new(container))
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
                .route("/webfinger", web::get().to(well_known_handlers::web_finger))
                .route("/nodeinfo", web::get().to(well_known_handlers::node_info_links))
        )
        .service(
            web::scope("/nodeinfo/2.1").route("", web::get().to(well_known_handlers::node_info))
        )
        .service(
            web::scope("/users")
                .route("", web::get().to(user_handlers::list_users_handler))
                .route("", web::post().to(user_handlers::create_user_handler))
                .route("/{user_id}", web::get().to(user_handlers::get_user_handler))
                .route("/{user_id}", web::delete().to(user_handlers::delete_user_handler))
        )
}
