//! NextChat Security library.
//!
//! This module contains the encrypt function for the user password using
//! Argon2id algorithm and the app secret key. Also, contains the function
//! to verify the user password using the posible password and the
//! encrypted password hash.

use std::env;

use argonautica::{Hasher, Verifier};

/// Get the `APP_SECRET_KEY` variable from the environment.
fn get_secret_key() -> String {
    match env::var("APP_SECRET_KEY") {
        Ok(secret_key) => secret_key,
        Err(_) => {
            panic!("Security Error -> Cannot get the `APP_SECRET_KEY` variable.");
        }
    }
}

/// Encrypt an user password using Argon2id algorithm and the app secret key.
///
/// # Example
/// ```rust
/// use std::env;
///
/// use nextchat_security::encrypt_password;
///
/// fn main() {
///     env::set_var("APP_SECRET_KEY", "NextChatORG");
///
///     let password: &str = "1234";
///     let hash = encrypt_password(password);
///     assert!(hash.is_ok());
/// }
/// ```
pub fn encrypt_password(password: &str) -> Result<String, String> {
    let mut hasher = Hasher::default();
    match hasher
        .with_password(password)
        .with_secret_key(get_secret_key())
        .hash()
    {
        Ok(password_hash) => Ok(password_hash),
        Err(_) => Err(String::from(
            "Cannot encrypt the password using Argon2id algorithm.",
        )),
    }
}

/// Verify the password with the user password hash using Argon2id algorithm and the
/// app secret key.
///
/// # Example
/// ```rust
/// use std::env;
///
/// use nextchat_security::{encrypt_password, verify_password};
///
/// fn main() {
///     env::set_var("APP_SECRET_KEY", "NextChatORG");
///
///     let password: &str = "1234";
///     let hash: String = encrypt_password(password).unwrap();
///
///     let verification = verify_password(password, &hash);
///     assert!(verification.is_ok());
///     assert!(verification.unwrap());
/// }
/// ```
pub fn verify_password(password: &str, password_hash: &str) -> Result<bool, String> {
    let mut verifier = Verifier::default();
    match verifier
        .with_hash(password_hash)
        .with_password(password)
        .with_secret_key(get_secret_key())
        .verify()
    {
        Ok(result) => Ok(result),
        Err(_) => Err(String::from(
            "Cannot verify the password hash using Argon2id algorithm.",
        )),
    }
}
