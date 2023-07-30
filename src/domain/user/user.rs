use base64::{Engine as _, engine::general_purpose};
use chrono::Utc;
use openssl::hash::MessageDigest;
use openssl::pkey::{PKey, Private, Public};
use openssl::rsa::{Padding, Rsa};
use openssl::sign::Signer;
use ulid::Ulid;

#[derive(Clone)]
pub struct User {
    pub id: String,
    pub username: String,
    pub display_name: String,
    pub created_at: String,
    pub updated_at: String,
    pub key_pair: UserRsaKey,
}

#[derive(Clone)]
pub struct UserRsaKey {
    pub private_key: PKey<Private>,
    pub public_key: PKey<Public>,
}

impl User {
    pub fn new(username: &str, display_name: &str) -> Self {
        let id = Ulid::new().to_string();
        let now = Utc::now();

        // generate rsa key pair
        let rsa_key = Rsa::generate(2048).unwrap();
        let key_pair = UserRsaKey {
            private_key: PKey::private_key_from_pem(&rsa_key.private_key_to_pem().unwrap()).unwrap(),
            public_key: PKey::public_key_from_pem(&rsa_key.public_key_to_pem().unwrap()).unwrap(),
        };

        User {
            id,
            username: username.to_string(),
            display_name: display_name.to_string(),
            created_at: now.clone().to_rfc3339(),
            updated_at: now.clone().to_rfc3339(),
            key_pair,
        }
    }

    pub fn sign(&self, data: &[u8]) -> String {
        let mut signer = Signer::new(MessageDigest::sha256(), &self.key_pair.private_key).unwrap();
        signer.set_rsa_padding(Padding::PKCS1).unwrap();
        let mut len = signer.len().unwrap();
        let mut buf = vec![0; len];
        len = signer.sign_oneshot(&mut buf, data).unwrap();
        buf.truncate(len);
        general_purpose::STANDARD.encode(buf)
    }
}

#[cfg(test)]
mod test {
    use openssl::hash::MessageDigest;
    use openssl::sign::{Signer, Verifier};
    use crate::domain::user::user::User;

    #[test]
    fn test_new_user_instance() {
        let user = User::new("john", "John Doe");

        // check attributes
        assert_ne!(user.id, "");
        assert_eq!(user.username, "john");
        assert_eq!(user.display_name, "John Doe");
        assert_ne!(user.created_at, "");
        assert_ne!(user.updated_at, "");

        // check key pair
        let data = b"hello, world!";
        let mut signer = Signer::new(MessageDigest::sha256(), &user.key_pair.private_key).unwrap();
        let _ = signer.update(data);
        let signature = signer.sign_to_vec().unwrap();
        let mut verifier = Verifier::new(MessageDigest::sha256(), &user.key_pair.public_key).unwrap();
        let _ = verifier.update(data);
        assert!(verifier.verify(&signature).unwrap());
    }
}
