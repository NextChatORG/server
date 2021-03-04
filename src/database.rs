pub mod models;

use sqlx::{Error, PgPool};
use std::env;

async fn check_if_table_exists(client: &PgPool, table_name: &str) -> Result<bool, Error> {
    let result: (bool,) =
        sqlx::query_as("SELECT EXISTS (SELECT FROM pg_tables WHERE tablename = $1)")
            .bind(table_name)
            .fetch_one(client)
            .await?;

    Ok(result.0)
}

pub async fn get_database_connection() -> Result<PgPool, Error> {
    let database_url: String =
        env::var("DATABASE_URL").expect("Cannot read `DATABASE_URL` from environment.");
    let client: PgPool = PgPool::connect(&database_url).await?;

    // Add `uuid-ossp` for `uuid_generate_v4` function.
    sqlx::query("CREATE EXTENSION IF NOT EXISTS \"uuid-ossp\"")
        .execute(&client)
        .await?;

    let users_table = check_if_table_exists(&client, "users").await?;
    if !users_table {
        sqlx::query(include_str!("./database/sqls/users_table.sql"))
            .execute(&client)
            .await?;
        println!("Users table created.");
    }

    Ok(client)
}
