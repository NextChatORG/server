pub mod models;

use chrono::NaiveDateTime;
use sqlx::{Error, PgPool};
use std::{env, time::SystemTime};

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

    let tables = [
        ("users", include_str!("./database/sqls/users.sql")),
        ("friends", include_str!("./database/sqls/friends.sql")),
    ];

    for (table, sql) in tables.iter() {
        if !check_if_table_exists(&client, table).await? {
            sqlx::query(sql).execute(&client).await?;
            println!("{} table created.", table);
        }
    }

    Ok(client)
}

pub fn get_now_time() -> NaiveDateTime {
    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("Cannot get timestamp.");
    NaiveDateTime::from_timestamp(now.as_secs() as i64, 0)
}
