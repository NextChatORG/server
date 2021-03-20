//! NextChat Server database module.
//!
//! This module contains the function to get the database pool connection to PostgreSQL.

use std::{convert::Infallible, env};

use sqlx::PgPool;
use warp::Filter;

/// Generate a database pool connection to PostgreSQL.
///
/// # Example
/// ```rust
/// let database_connection: PgPool = match get_pool_connection() {
///     Ok(db) => db,
///     Err(e) => {
///         eprintln!("Database connection error: {:?}", e);
///         return;
///     },
/// }
/// ```
///
/// See [https://docs.rs/sqlx/0.5.1/sqlx/type.PgPool.html](https://docs.rs/sqlx/0.5.1/sqlx/type.PgPool.html)
/// for more information.
pub async fn get_pool_connection() -> anyhow::Result<PgPool> {
    let database_url: String = match env::var("DATABASE_URL") {
        Ok(url) => url,
        Err(_) => {
            return Err(anyhow::Error::msg(
                "Cannot read `DATABASE_URL` from environment variables.",
            ));
        }
    };

    // Connect to the PostgreSQL database using a pool connection.
    let client: PgPool = PgPool::connect(&database_url).await?;

    // Add `uuid-ossp` for `uuid_generate_v4` function.
    sqlx::query("CREATE EXTENSION IF NOT EXISTS \"uuid-ossp\"")
        .execute(&client)
        .await?;

    Ok(client)
}

/// This function helps to add a copy of the database connection to a warp path.
///
/// # Example
/// ```rust
/// let database_connection = get_pool_connection().unwrap();
///
/// async fn handler(client: PgPool) { }
///
/// let route = warp::path("testing")
///     .and(with_client(database_connection))
///     .and_then(handler);
/// ```
pub fn with_client(client: PgPool) -> impl Filter<Extract = (PgPool,), Error = Infallible> + Clone {
    warp::any().map(move || client.clone())
}
