//! NextChat Server security module.
//!
//! This module contains the encrypt function for the user password using
//! Argon2 algorithm and the app secret key. Also, contains the function
//! to verify the user password using the posible password and the
//! encrypted password hash.

use std::env;

use argonautica::{Hasher, Verifier};
use colored::*;

/// Get the `APP_SECRET_KEY` variable from the environment.
fn get_secret_key() -> String {
    match env::var("APP_SECRET_KEY") {
        Ok(secret_key) => secret_key,
        Err(_) => {
            panic!(
                "{} Cannot get the `APP_SECRET_KEY` variable.",
                "Security Error".red()
            );
        }
    }
}

/// Encrypt an user password using Argon2 algorithm and the app secret key.
///
/// # Example
/// ```rust
/// let password: &str = "1234";
/// let hash: String = match encrypt_password(password);
/// ```
pub fn encrypt_password(password: &str) -> anyhow::Result<String> {
    let mut hasher = Hasher::default();
    match hasher
        .with_password(password)
        .with_secret_key(get_secret_key())
        .hash()
    {
        Ok(password_hash) => Ok(password_hash),
        Err(_) => Err(anyhow::Error::msg(
            "Cannot encrypt the password using Argon2 algorithm.",
        )),
    }
}

/// Verify the password with the user password hash using Argon2 algorithm and the
/// app secret key.
///
/// # Example
/// ```rust
/// let password: &str = "1234";
/// let check: bool = verify_password(password, "argon2_hash");
/// ```
pub fn verify_password(password: &str, password_hash: &str) -> anyhow::Result<bool> {
    let mut verifier = Verifier::default();
    match verifier
        .with_hash(password_hash)
        .with_password(password)
        .with_secret_key(get_secret_key())
        .verify()
    {
        Ok(result) => Ok(result),
        Err(_) => Err(anyhow::Error::msg(
            "Cannot verify the password hash using Argon2 algorithm.",
        )),
    }
}
