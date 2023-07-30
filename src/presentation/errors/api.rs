use std::fmt::Formatter;
use actix_web::body::BoxBody;
use actix_web::HttpResponse;
use crate::domain::error::{CommonError, CommonErrorCode};

#[derive(Debug)]
pub struct ApiError(CommonError);

impl From<CommonError> for ApiError {
    fn from(e: CommonError) -> Self {
        ApiError(e)
    }
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.get_message())
    }
}

impl actix_web::ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse<BoxBody> {
        match self.0.get_code() {
            CommonErrorCode::UserDoesNotExists => HttpResponse::NotFound().body(self.0.get_message()),
            CommonErrorCode::UsernameAlreadyExists => HttpResponse::BadRequest().body(self.0.get_message()),
            CommonErrorCode::DBError => HttpResponse::InternalServerError().body(""),
            CommonErrorCode::UnexpectedError => HttpResponse::InternalServerError().body(""),
        }
    }
}
