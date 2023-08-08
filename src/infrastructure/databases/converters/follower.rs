use chrono::{DateTime, Utc};
use sea_orm::ActiveValue::Set;
use crate::domain::follower::follower::Follower;
use crate::infrastructure::databases::entities::follower;

impl From<&Follower> for follower::ActiveModel {
    fn from(new_follower: &Follower) -> Self {
        follower::ActiveModel {
            id: Default::default(),
            user_id: Set(new_follower.user_id.clone()),
            actor: Set(new_follower.actor.clone()),
            object: Set(new_follower.object.clone()),
            inbox: Set(new_follower.inbox.clone()),
            created_at: Set(new_follower.created_at.to_rfc3339()),
        }
    }
}

impl Into<Follower> for follower::Model {
    fn into(self) -> Follower {
        Follower {
            id: self.id,
            user_id: self.user_id,
            actor: self.actor,
            object: self.object,
            inbox: self.inbox,
            created_at: DateTime::parse_from_rfc3339(&self.created_at).unwrap().with_timezone(&Utc),
        }
    }
}
