use base64::{Engine as _, engine::general_purpose};
use openssl::hash::MessageDigest;
use openssl::pkey::{PKey, Private, Public};
use openssl::rsa::Padding;
use openssl::sign::Signer;

pub struct UserRsaKey {
    pub private_key: PKey<Private>,
    pub public_key: PKey<Public>,
}

impl UserRsaKey {
    pub fn sign_rsa_sha256_with_private_key(&self, data: &[u8]) -> String {
        let mut signer = Signer::new(MessageDigest::sha256(), &self.private_key).unwrap();
        signer.set_rsa_padding(Padding::PKCS1).unwrap();
        let mut len = signer.len().unwrap();
        let mut buf = vec![0; len];
        len = signer.sign_oneshot(&mut buf, data).unwrap();
        buf.truncate(len);
        general_purpose::STANDARD.encode(buf)
    }
}
