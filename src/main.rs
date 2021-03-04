mod database;
mod handlers;
mod security;

use dotenv::dotenv;
use sqlx::PgPool;
use std::env;

#[tokio::main]
async fn main() {
    // Read the .env file.
    dotenv().ok();

    // Get the database pool connection.
    let database_connection: PgPool = match database::get_database_connection().await {
        Ok(connection) => {
            println!("Database -> Connected!");
            connection
        }
        Err(e) => {
            eprintln!("failed to connect to the database - error: {:?}", e);
            return;
        }
    };

    // Get the server host from the environment.
    let host: Vec<u8> = env::var("API_HOST")
        .unwrap_or_else(|_| String::from("127.0.0.1"))
        .split('.')
        .map(|x| x.parse::<u8>().unwrap_or_else(|_| 0))
        .collect();

    // Get `API_PORT` from the environment variables.
    let port: u16 = env::var("API_PORT")
        .unwrap_or_else(|_| String::from("5000"))
        .parse()
        .unwrap_or_else(|_| 5000);

    warp::serve(handlers::routes(&database_connection))
        .run(([host[0], host[1], host[2], host[3]], port))
        .await;
}
