use openssl::rsa::Rsa;

pub struct RsaKeyService {}

impl RsaKeyService {
    pub fn generate_key_pair() -> (String, String) {
        let key_pair = Rsa::generate(2048).unwrap();
        let private_pem = key_pair.private_key_to_pem().unwrap();
        let public_pem = key_pair.public_key_to_pem().unwrap();

        (
            String::from_utf8(private_pem).unwrap(),
            String::from_utf8(public_pem).unwrap()
        )
    }
}
