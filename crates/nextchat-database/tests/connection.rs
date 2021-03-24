#[cfg(feature = "panic-tests")]
#[tokio::test]
#[should_panic(
    expected = "Database Error -> Cannot read `DATABASE_URL` from environment variables."
)]
async fn test_database_url_panic() {
    if std::env::var("DATABASE_URL").is_ok() {
        std::env::remove_var("DATABASE_URL");
    }

    assert!(std::env::var("DATABASE_URL").is_err());

    let client = nextchat_database::get_client().await;
    assert!(client.is_err());
}
