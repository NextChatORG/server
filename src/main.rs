//! NextChat main module.
//!
//! This module contains the database connection and server initializations.

use std::env;

use colored::*;
use dotenv::dotenv;

#[tokio::main]
async fn main() {
    // Read the .env file if exists.
    dotenv().ok();

    // Get the postgres pool connection.
    let database_connection = match nextchat_database::get_client().await {
        Ok(connection) => {
            println!("Database {}", "Connected!".green());
            connection
        }
        Err(e) => {
            eprintln!("{} {:?}", "Database Error".red(), e);
            return;
        }
    };

    // Get the server host from the environment variables.
    let host: Vec<u8> = env::var("API_HOST")
        .unwrap_or_else(|_| String::from("127.0.0.1"))
        .split('.')
        .map(|x| x.parse::<u8>().unwrap_or(0))
        .collect();

    // Check if the host doesn't have 4 ports.
    if host.len() != 4 {
        panic!(
            "{} `{:?}` is not a valid server host.",
            "Configuration Error".red(),
            host
        );
    }

    // Get the server port from the environment variables.
    let port: u16 = env::var("API_PORT")
        .unwrap_or_else(|_| String::from("5000"))
        .parse()
        .unwrap_or(5000);

    // Initialize the NextChat Server.
    nextchat_server::run(
        &database_connection,
        [host[0], host[1], host[2], host[3]],
        port,
    )
    .await;
}
