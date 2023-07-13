use actix_web::{App, Error, middleware, web};
use actix_web::body::MessageBody;
use actix_web::dev::{ServiceFactory, ServiceRequest, ServiceResponse};
use crate::api::controllers::root_handlers;

pub fn create_app() -> App<
    impl ServiceFactory<
        ServiceRequest,
        Response=ServiceResponse<impl MessageBody>,
        Config=(),
        InitError=(),
        Error=Error,
    >
> {
    App::new()
        .wrap(middleware::Logger::default())
        .service(
            web::scope("/")
                .route("", web::get().to(root_handlers::echo_ok))
        )
}
