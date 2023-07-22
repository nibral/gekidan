use chrono::Utc;
use ulid::Ulid;

#[derive(Clone)]
pub struct User {
    pub id: String,
    pub username: String,
    pub display_name: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Clone)]
pub struct CreateUser {
    pub username: String,
    pub display_name: String,
}

impl From<CreateUser> for User {
    fn from(new_user: CreateUser) -> Self {
        let id = Ulid::new().to_string();
        let now = Utc::now();
        User {
            id,
            username: new_user.username,
            display_name: new_user.display_name,
            created_at: now.clone().to_rfc3339(),
            updated_at: now.clone().to_rfc3339(),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::domain::models::user::{CreateUser, User};

    #[test]
    fn generate_user_instance_from_creation_info() {
        let creation_info = CreateUser {
            username: "John Doe".to_string(),
            display_name: "john".to_string(),
        };
        let user = User::from(creation_info);

        assert_ne!(user.id, "");
        assert_eq!(user.username, "John Doe");
        assert_eq!(user.display_name, "john");
        assert_ne!(user.created_at, "");
        assert_ne!(user.updated_at, "");
        assert_eq!(user.created_at, user.updated_at);
    }
}
