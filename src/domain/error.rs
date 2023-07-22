use std::collections::HashMap;
use std::sync::Mutex;
use once_cell::sync::Lazy;
use crate::domain::error::CommonErrorCode::*;

#[derive(Debug)]
pub struct CommonError {
    code: CommonErrorCode,
    message: String,
}

impl CommonError {
    pub fn new(code: CommonErrorCode) -> CommonError {
        let message = COMMON_ERROR_MESSAGES.lock().unwrap().get(&code).unwrap().to_string();
        CommonError {
            code,
            message,
        }
    }

    pub fn get_code(&self) -> CommonErrorCode {
        self.code.clone()
    }

    pub fn get_message(&self) -> String {
        self.message.to_string()
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum CommonErrorCode {
    UserDoesNotExists,
    UnexpectedDBError,
}

static COMMON_ERROR_MESSAGES: Lazy<Mutex<HashMap<CommonErrorCode, String>>> = Lazy::new(|| {
    let mut m = HashMap::new();

    m.insert(UserDoesNotExists, "User does not exists".to_string());
    m.insert(UnexpectedDBError, "Unexpected DB Error".to_string());

    Mutex::new(m)
});

#[cfg(test)]
mod tests {
    use crate::domain::error::{CommonError, CommonErrorCode};

    #[test]
    fn get_error_message_from_code() {
        let err = CommonError::new(CommonErrorCode::UnexpectedDBError);
        assert_eq!(err.get_message(), "Unexpected DB Error");
    }
}
