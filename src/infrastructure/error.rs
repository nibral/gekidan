use std::fmt::Formatter;
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
    fn error_response(&self) -> actix_web::HttpResponse {
        match self.0.get_code() {
            CommonErrorCode::UserDoesNotExists => actix_web::HttpResponse::NotFound().body(""),
            CommonErrorCode::UnexpectedDBError => actix_web::HttpResponse::InternalServerError().body(""),
        }
    }
}
