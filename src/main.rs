//! NextChat Server main module.
//!
//! This module contains the database connection and server initializations.

mod controllers;
mod core;
mod models;
mod services;

use std::env;

use colored::*;
use dotenv::dotenv;
use sqlx::PgPool;

#[tokio::main]
async fn main() {
    // Read the .env file if exists.
    dotenv().ok();

    // Get the postgres pool connection.
    let database_connection: PgPool = match core::database::get_pool_connection().await {
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

    warp::serve(controllers::routes(&database_connection))
        .run(([host[0], host[1], host[2], host[3]], port))
        .await;
}
