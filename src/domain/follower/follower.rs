use chrono::{DateTime, Utc};

#[derive(Clone, Debug)]
pub struct Follower {
    pub id: i32,
    pub user_id: String,
    pub actor: String,
    pub object: String,
    pub inbox: String,
    pub created_at: DateTime<Utc>,
}

impl Follower {
    pub fn new(user_id: &str, actor: &str, object: &str, inbox: &str) -> Self {
        Follower {
            id: 0,
            user_id: user_id.to_string(),
            actor: actor.to_string(),
            object: object.to_string(),
            inbox: inbox.to_string(),
            created_at: Utc::now(),
        }
    }
}
