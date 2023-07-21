use serde::{Deserialize, Serialize};
use crate::domain::models::user::{CreateUser, User};

#[derive(Serialize)]
pub struct UserDTO {
    pub id: String,
    pub username: String,
    pub display_name: String,
}

#[derive(Deserialize)]
pub struct CreateUserDTO {
    pub username: String,
    pub display_name: String,
}

#[derive(Serialize)]
pub struct ListUsersDTO {
    pub users: Vec<UserDTO>,
}

impl Into<UserDTO> for User {
    fn into(self) -> UserDTO {
        UserDTO {
            id: self.id,
            username: self.username,
            display_name: self.display_name,
        }
    }
}

impl Into<CreateUser> for CreateUserDTO {
    fn into(self) -> CreateUser {
        CreateUser {
            username: self.username,
            display_name: self.display_name,
        }
    }
}

impl From<Vec<User>> for ListUsersDTO {
    fn from(value: Vec<User>) -> Self {
        ListUsersDTO {
            users: value.iter().map(|u| u.clone().into()).collect()
        }
    }
}
