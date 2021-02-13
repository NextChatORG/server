use std::env;

pub fn get_secret_key() -> String {
    env::var("APP_SECRET_KEY").expect("Cannot get the `APP_SECRET_KEY` variable.")
}

pub fn encrypt_password(password: String) -> String {
    let config = argon2::Config::default();
    argon2::hash_encoded(password.as_bytes(), get_secret_key().as_bytes(), &config)
        .expect("Cannot encrypt the password.")
}

pub fn verify_password(password: String, password_hash: String) -> bool {
    argon2::verify_encoded(&password_hash, password.as_bytes())
        .expect("Cannot verify the password.")
}
