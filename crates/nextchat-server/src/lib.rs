//! NextChat Server library.

use nextchat_database::Client;

mod controllers;
mod response;
mod services;

pub use controllers::{with_client, with_storage};
pub use response::{Error, Response};

pub async fn run(client: &Client, host: [u8; 4], port: u16) {
    warp::serve(controllers::routes(client))
        .run((host, port))
        .await;
}
