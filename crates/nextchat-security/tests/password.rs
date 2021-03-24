#[test]
fn test_encryption() {
    std::env::set_var("APP_SECRET_KEY", "NextChatORG");

    let password = "1234";

    let hash = nextchat_security::encrypt_password(password);
    assert!(hash.is_ok());

    let verification = nextchat_security::verify_password(password, &hash.unwrap());
    assert!(verification.is_ok());
    assert!(verification.unwrap());
}

#[cfg(feature = "panic-tests")]
#[test]
#[should_panic(expected = "Security Error -> Cannot get the `APP_SECRET_KEY` variable.")]
fn test_app_secret_key_panic() {
    if std::env::var("APP_SECRET_KEY").is_ok() {
        std::env::remove_var("APP_SECRET_KEY");
    }

    assert!(std::env::var("APP_SECRET_KEY").is_err());
    nextchat_security::encrypt_password("1234").unwrap_or(String::new());
}
