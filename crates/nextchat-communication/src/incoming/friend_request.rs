//! NextChat Communication friend request event module.

use nextchat_database::Client;

use crate::{CommunicationMessage, Connection, StorageType};

use super::PacketEvent;

pub struct FriendRequestEvent;

impl PacketEvent for FriendRequestEvent {
    fn run(
        _connection: &Connection,
        message: &CommunicationMessage,
        _client: &Client,
        _storage: &StorageType,
    ) {
        println!("Friend Request Event: {}", message.to_string())
    }
}
