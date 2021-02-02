mod database;
mod routes;

use actix_web::{App, HttpServer};
use dotenv::dotenv;
use sqlx::PgPool;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Read the .env file.
    dotenv().ok();

    // Get the database pool connection.
    let client: PgPool = database::get_database_connection().await.expect("Cannot get the database connection.");

    // Get the server host from the environment.
    let host: String = env::var("API_HOST").unwrap_or(String::from("127.0.0.1"));

    // Get `API_PORT` from the environment variables.
    let port: u16 = env::var("API_PORT").unwrap_or(String::from("5000")).parse().unwrap_or(5000);

    // HTTP Server.
    HttpServer::new(move || {
        App::new()
            .data(client.clone())
            .service(routes::users::scope())
    })
    .bind((host, port))?
    .run()
    .await
}
