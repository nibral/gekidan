use std::collections::HashMap;
use std::sync::Mutex;
use once_cell::sync::Lazy;
use crate::domain::error::CommonErrorCode::{DBError, NoteDoesNotExists, UnexpectedError, UserDoesNotExists, UsernameAlreadyExists};

#[derive(Debug)]
pub struct CommonError {
    code: CommonErrorCode,
    message: String,
}

impl CommonError {
    pub fn new(code: CommonErrorCode) -> CommonError {
        let message = COMMON_ERROR_MESSAGES.lock().unwrap()
            .get(&code).unwrap()
            .to_string();
        CommonError {
            code,
            message,
        }
    }

    pub fn get_code(&self) -> CommonErrorCode {
        self.code.clone()
    }

    pub fn get_message(&self) -> String {
        self.message.clone()
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum CommonErrorCode {
    UserDoesNotExists,
    UsernameAlreadyExists,
    NoteDoesNotExists,
    DBError,
    UnexpectedError,
}

static COMMON_ERROR_MESSAGES: Lazy<Mutex<HashMap<CommonErrorCode, String>>> = Lazy::new(|| {
    let mut m = HashMap::new();

    m.insert(UserDoesNotExists, "User does not exists".to_string());
    m.insert(UsernameAlreadyExists, "Username already exists".to_string());
    m.insert(NoteDoesNotExists, "Note does not exists".to_string());
    m.insert(DBError, "DB error".to_string());
    m.insert(UnexpectedError, "Unexpected error".to_string());

    Mutex::new(m)
});

#[cfg(test)]
mod tests {
    use crate::domain::error::{CommonError, CommonErrorCode};

    #[test]
    fn get_message_from_code() {
        let err = CommonError::new(CommonErrorCode::UnexpectedError);
        assert_eq!(err.get_message(), "Unexpected error");
    }
}
