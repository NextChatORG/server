//! NextChat Communication inconming module.

mod friend_request;

use nextchat_database::Client;

use super::{CommunicationMessage, Connection, StorageType};

pub trait PacketEvent {
    fn run(
        connection: &Connection,
        message: &CommunicationMessage,
        client: &Client,
        storage: &StorageType,
    );
}

pub fn run_event(
    connection: &Connection,
    message: &CommunicationMessage,
    client: &Client,
    storage: &StorageType,
) {
    match message.get_name().as_str() {
        "friend_request" => {
            friend_request::FriendRequestEvent::run(connection, message, client, storage)
        }
        _ => {
            println!("Unknown event: {}", message.to_string());
        }
    }
}
