use std::sync::Arc;
use actix_web::{App, Error, middleware, web};
use actix_web::body::MessageBody;
use actix_web::dev::{ServiceFactory, ServiceRequest, ServiceResponse};
use crate::app::container::Container;
use crate::presentation::controllers::*;

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

        // logger
        .wrap(middleware::Logger::default())

        // routes
        .service(
            web::scope("/").route("", web::get().to(echo::echo_ok))
        )
        .service(
            web::scope("/.well-known")
                .route("/host-meta", web::get().to(activity_pub::host_meta))
                .route("/webfinger", web::get().to(activity_pub::web_finger))
                .route("/nodeinfo", web::get().to(activity_pub::node_info_links))
        )
        .service(
            web::scope("/nodeinfo/2.1").route("", web::get().to(activity_pub::node_info))
        )
        .service(
            web::scope("/admin")
                .service(
                    web::scope("/users")
                        .route("", web::post().to(user_management::create_user))
                        .route("", web::get().to(user_management::list_users))
                        .route("/{user_id}", web::get().to(user_management::get_user))
                        .route("/{user_id}", web::put().to(user_management::update_user))
                        .route("/{user_id}", web::delete().to(user_management::delete_user))
                )
        )
}
