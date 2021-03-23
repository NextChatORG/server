use nextchat_database::get_client;

#[tokio::test]
#[should_panic(
    expected = "Database Error -> Cannot read `DATABASE_URL` from environment variables."
)]
async fn test_database_url_panic() {
    let client = get_client().await;
    assert!(client.is_err());
}
