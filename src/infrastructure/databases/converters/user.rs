use openssl::pkey::PKey;
use sea_orm::ActiveValue::Set;
use crate::domain::user::user::{User, UserRsaKey};
use crate::infrastructure::databases::entities::{user, user_rsa_key};

impl From<&User> for user::ActiveModel {
    fn from(user: &User) -> Self {
        user::ActiveModel {
            id: Set(user.id.clone()),
            username: Set(user.username.clone()),
            display_name: Set(user.display_name.clone()),
            created_at: Set(user.created_at.clone()),
            updated_at: Set(user.updated_at.clone()),
        }
    }
}

impl From<&User> for user_rsa_key::ActiveModel {
    fn from(user: &User) -> Self {
        let private_key_str = String::from_utf8(user.key_pair.private_key.private_key_to_pem_pkcs8().unwrap()).unwrap();
        let public_key_str = String::from_utf8(user.key_pair.public_key.public_key_to_pem().unwrap()).unwrap();

        user_rsa_key::ActiveModel {
            user_id: Set(user.id.clone()),
            private_key: Set(private_key_str),
            public_key: Set(public_key_str),
        }
    }
}

impl Into<UserRsaKey> for user_rsa_key::Model {
    fn into(self) -> UserRsaKey {
        UserRsaKey {
            private_key: PKey::private_key_from_pem(self.private_key.as_bytes()).unwrap(),
            public_key: PKey::public_key_from_pem(self.public_key.as_bytes()).unwrap(),
        }
    }
}

pub fn restore(user: &user::Model, key_pair: &user_rsa_key::Model) -> User {
    User {
        id: user.id.clone(),
        username: user.username.clone(),
        display_name: user.display_name.clone(),
        created_at: user.created_at.clone(),
        updated_at: user.updated_at.clone(),
        key_pair: (*key_pair).clone().into(),
    }
}
