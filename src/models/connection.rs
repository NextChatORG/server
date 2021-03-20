//! NextChat Server connection model module.
//!
//! This module contains the socket connection structure.

use tokio::sync::mpsc::{self, error::SendError};
use uuid::Uuid;
use warp::ws::Message;

type Socket = mpsc::UnboundedSender<Result<Message, warp::Error>>;

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
    ) -> Result<(), SendError<Result<Message, warp::Error>>> {
        self.socket.send(Ok(Message::text(message)))
    }
}
