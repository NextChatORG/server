use argonautica::{Hasher, Verifier};
use std::env;

pub fn get_secret_key() -> String {
    env::var("APP_SECRET_KEY").expect("Cannot get the `APP_SECRET_KEY` variable.")
}

pub fn encrypt_password(password: String) -> String {
    let mut hasher = Hasher::default();
    hasher
        .with_password(password)
        .with_secret_key(get_secret_key())
        .hash()
        .expect("Cannot encrypt the password.")
}

pub fn verify_password(password: String, password_hash: String) -> bool {
    let mut verifier = Verifier::default();
    verifier
        .with_hash(password_hash)
        .with_password(password)
        .with_secret_key(get_secret_key())
        .verify()
        .expect("Cannot verify the password.")
}
