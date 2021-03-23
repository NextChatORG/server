//! NextChat Communication friend request event module.

use async_trait::async_trait;
use nextchat_database::Client;

use crate::{CommunicationMessage, Connection, StorageType};

use super::PacketEvent;

pub struct FriendRequestEvent;

#[async_trait]
impl PacketEvent for FriendRequestEvent {
    async fn run(
        _connection: &Connection,
        message: &CommunicationMessage,
        _client: &Client,
        _storage: &StorageType,
    ) {
        println!("Friend Request Event: {}", message.to_string())
    }
}
