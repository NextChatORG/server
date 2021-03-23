//! NextChat Database library.

pub mod models;

use std::env;

pub use chrono::NaiveDateTime;
pub use sqlx::{postgres::PgRow, query, Error, Row, Type};
pub use uuid::Uuid;

pub type Client = sqlx::PgPool;

/// Generate a database pool connection to PostgreSQL.
///
/// # Example
/// ```rust
/// use std::env;
///
/// use nextchat_database::{Client, get_client};
///
/// #[tokio::main]
/// async fn main() {
///     env::set_var("DATABASE_URL", "postgres://postgres:password@localhost/nextchat");
///
///     let _client: Client = match get_client().await {
///         Ok(db) => db,
///         Err(e) => {
///             assert_eq!(format!("{:?}", e), String::from("PoolTimedOut"));
///             return;
///         },
///     };
///
///     assert!(true);
/// }
/// ```
///
/// See [https://docs.rs/sqlx/0.5.1/sqlx/type.PgPool.html](https://docs.rs/sqlx/0.5.1/sqlx/type.PgPool.html)
/// for more information.
pub async fn get_client() -> Result<Client, Error> {
    let database_url: String = match env::var("DATABASE_URL") {
        Ok(url) => url,
        Err(_) => {
            panic!("Database Error -> Cannot read `DATABASE_URL` from environment variables.");
        }
    };

    // Connect to the PostgreSQL database using a pool connection.
    let client: Client = Client::connect(&database_url).await?;

    // Add `uuid-ossp` for `uuid_generate_v4` function.
    sqlx::query("CREATE EXTENSION IF NOT EXISTS \"uuid-ossp\"")
        .execute(&client)
        .await?;

    Ok(client)
}
