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
