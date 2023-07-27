use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use actix_web::{Error, FromRequest, HttpRequest};
use actix_web::dev::Payload;
use actix_web::error::ErrorUnauthorized;
use actix_web::web::Data;
use crate::container::Container;

pub struct AdminClaim;

impl FromRequest for AdminClaim {
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output=Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let container = req.app_data::<Data<Arc<Container>>>().unwrap().clone();
        let correct = container.app_config_service.get_app_config().admin_api_key.clone();
        let challenge = match get_api_key(req) {
            Some(v) => v.to_string(),
            None => "".to_string(),
        };

        Box::pin(async move {
            if challenge == correct {
                Ok(AdminClaim {})
            } else {
                Err(ErrorUnauthorized("Unauthorized"))
            }
        })
    }
}

fn get_api_key(req: &HttpRequest) -> Option<&str> {
    req.headers().get("x-api-key")?.to_str().ok()
}
