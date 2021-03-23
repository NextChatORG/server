//! NextChat Communication inconming module.

mod friend_request;

use async_trait::async_trait;
use nextchat_database::Client;

use super::{CommunicationMessage, Connection, StorageType};

#[async_trait]
pub trait PacketEvent {
    async fn run(
        connection: &Connection,
        message: &CommunicationMessage,
        client: &Client,
        storage: &StorageType,
    );
}

pub async fn run_event(
    connection: &Connection,
    message: &CommunicationMessage,
    client: &Client,
    storage: &StorageType,
) {
    match message.get_name().as_str() {
        "friend_request" => {
            friend_request::FriendRequestEvent::run(connection, message, client, storage).await
        }
        _ => {
            println!("Unknown event: {}", message.to_string());
        }
    }
}
