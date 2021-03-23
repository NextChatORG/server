//! NextChat Communication connection module.
//!
//! This module contains the socket connection structure.

use nextchat_database::Uuid;
use tokio::sync::mpsc::{self, error::SendError};
use warp::{ws::Message, Error};

use crate::outgoing::PacketComposer;

type Socket = mpsc::UnboundedSender<Result<Message, Error>>;

#[derive(Clone)]
pub struct Connection {
    user_id: Uuid,
    socket: Socket,
}

impl Connection {
    /// Create a new socket connection object.
    pub fn new(user_id: &Uuid, socket: &Socket) -> Self {
        Self {
            user_id: user_id.clone(),
            socket: socket.clone(),
        }
    }

    /// Get the user id.
    pub fn get_user_id(&self) -> Uuid {
        self.user_id
    }

    /// Send a text message using the websocket connection.
    pub fn send_text_message(
        &self,
        message: &str,
    ) -> Result<(), SendError<Result<Message, Error>>> {
        self.socket.send(Ok(Message::text(message)))
    }

    /// Send a packet composer.
    pub fn send_packet(
        &self,
        packet: Box<dyn PacketComposer>,
    ) -> Result<(), SendError<Result<Message, Error>>> {
        self.send_text_message(&packet.to_message().to_string())
    }
}
