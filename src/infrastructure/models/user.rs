use crate::domain::models::user::User;
use crate::infrastructure::entities::*;

impl From<User> for user::Model {
    fn from(value: User) -> Self {
        user::Model {
            id: value.id,
            username: value.username,
            display_name: value.display_name,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}

impl Into<User> for user::Model {
    fn into(self) -> User {
        User {
            id: self.id,
            username: self.username,
            display_name: self.display_name,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}
