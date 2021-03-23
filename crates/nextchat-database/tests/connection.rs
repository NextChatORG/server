use std::env;

use nextchat_database::get_client;

#[tokio::test]
#[should_panic(
    expected = "Database Error -> Cannot read `DATABASE_URL` from environment variables."
)]
async fn test_database_url_panic() {
    if env::var("DATABASE_URL").is_ok() {
        env::remove_var("DATABASE_URL");
    }

    assert!(env::var("DATABASE_URL").is_err());

    let client = get_client().await;
    assert!(client.is_err());
}
