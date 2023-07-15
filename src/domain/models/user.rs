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
