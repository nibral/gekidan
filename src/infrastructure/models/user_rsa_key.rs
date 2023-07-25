use openssl::pkey::PKey;
use crate::domain::models::user_rsa_key::UserRsaKey;
use crate::infrastructure::entities::*;

impl Into<UserRsaKey> for user_rsa_key::Model {
    fn into(self) -> UserRsaKey {
        UserRsaKey {
            private_key: PKey::private_key_from_pem(self.private_key.as_bytes()).unwrap(),
            public_key: PKey::public_key_from_pem(self.public_key.as_bytes()).unwrap(),
        }
    }
}
