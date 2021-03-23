use std::env;

use nextchat_security::{encrypt_password, verify_password};

#[test]
fn test_encryption() {
    env::set_var("APP_SECRET_KEY", "NextChatORG");

    let password = "1234";

    let hash = encrypt_password(password);
    assert!(hash.is_ok());

    let verification = verify_password(password, &hash.unwrap());
    assert!(verification.is_ok());
    assert!(verification.unwrap());
}

#[test]
#[should_panic(expected = "Security Error -> Cannot get the `APP_SECRET_KEY` variable.")]
fn test_app_secret_key_panic() {
    if env::var("APP_SECRET_KEY").is_ok() {
        env::remove_var("APP_SECRET_KEY");
    }

    assert!(env::var("APP_SECRET_KEY").is_err());
    encrypt_password("1234").unwrap_or(String::new());
}
