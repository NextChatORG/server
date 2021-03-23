//! NextChat Communication library.

mod connection;
mod incoming;
mod outgoing;
mod storage;

pub use connection::Connection;
pub use incoming::run_event;
pub use storage::{Storage, StorageType};

pub struct CommunicationMessage {
    name: String,
    arguments: Vec<String>,
}

impl CommunicationMessage {
    /// Parse a string like `/{name} {argument1} {argument2}` to a CommunicationMessage struct.
    pub fn from_string(message: String) -> anyhow::Result<CommunicationMessage> {
        // Check if the messages does not start with a slash (`/`).
        if !message.starts_with("/") {
            return Err(anyhow::Error::msg(
                "The message format is incorrect. `/{name} {argument1} {argument2}`",
            ));
        }

        // Split the message by whitespaces.
        let arguments: Vec<String> = message.split(' ').map(|e| String::from(e)).collect();

        Ok(Self {
            name: String::from(&arguments[0][1..]),
            arguments: arguments[1..].iter().map(|e| e.clone()).collect(),
        })
    }

    /// Get the event name.
    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    /// Get the event arguments.
    pub fn get_arguments(&self) -> Vec<String> {
        self.arguments.clone()
    }

    /// Convert the event to a string.
    pub fn to_string(&self) -> String {
        format!("/{} {}", self.name, self.arguments.join(" "))
    }
}
