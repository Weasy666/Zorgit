use argon2::{self, Config};
use data_encoding::HEXLOWER;
use rand::{RngCore, SeedableRng};
use rand_chacha::ChaChaRng;

type EncodedPassword = String;

pub fn password_hash(password: &str) -> EncodedPassword {
    let mut rnd = ChaChaRng::from_entropy();
    let mut salt = [0u8; 32];
    rnd.fill_bytes(&mut salt);
    
    let mut  config = Config::default();
    config.variant = argon2::Variant::Argon2id;
    argon2::hash_encoded(password.as_bytes(), &salt, &config).unwrap()
}

pub fn password_verify(encoded_password: &str, entered_password: &str) -> Result<bool, String> {
    argon2::verify_encoded(encoded_password, entered_password.as_bytes()).map_err(|e| e.to_string())
}

pub fn generate_session_token() -> String {
    let mut rnd = ChaChaRng::from_entropy();
    let mut token = [0u8; 32];
    rnd.fill_bytes(&mut token);

    HEXLOWER.encode(&token)
}