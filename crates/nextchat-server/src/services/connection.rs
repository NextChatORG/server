//! NextChat Server connection service module.
//!
//! This module contains the `on_message`, `on_close_connection` and `on_new_connection` events
//! of a WebSocket connection.

use futures::{FutureExt, StreamExt};
use nextchat_communication::{CommunicationMessage, Connection, StorageType};
use nextchat_database::{Client, Uuid};
use tokio::sync::mpsc;
use tokio_stream::wrappers::UnboundedReceiverStream;
use warp::ws::{Message, WebSocket};

/// This functions handles all received messages and run the correspondent event.
async fn on_new_message(
    message: Message,
    connection: &Connection,
    client: &Client,
    storage: &StorageType,
) {
    let message = match message.to_str() {
        Ok(message) => message,
        Err(_) => {
            return;
        }
    };

    // Get the communication message.
    let cmessage = CommunicationMessage::from_string(String::from(message))
        .expect("Cannot parse the message string to a communication message.");

    // Run a event.
    nextchat_communication::run_event(connection, &cmessage, client, storage);
}

/// Remove a connection from the storage.
async fn on_close_connection(user_id: &Uuid, _client: &Client, storage: &StorageType) {
    let mut storage = storage.write().await;

    storage.remove_connection(user_id);

    println!("User disconnected: {}", user_id);
}

/// This functions handle the socket messages and add the new connection
/// to the storage.
pub async fn on_new_connection(
    socket: WebSocket,
    user_id: Uuid,
    client: Client,
    storage: StorageType,
) {
    let mut storage_mut = storage.write().await;

    // Split the socket into a sender and receive of messages.
    let (user_socket_tx, mut user_socket_rx) = socket.split();
    let (tx, rx) = mpsc::unbounded_channel();
    let rx = UnboundedReceiverStream::new(rx);

    tokio::task::spawn(rx.forward(user_socket_tx).map(|result| {
        if let Err(e) = result {
            eprintln!("WebSocket send error: {:?}", e);
        }
    }));

    let connection = Connection::new(&user_id, &tx);

    storage_mut.add_connection(&user_id, &connection);

    println!("User connected: {}", user_id);

    // Handle the socket messages.
    while let Some(result) = user_socket_rx.next().await {
        // Get the Message struct.
        let message = match result {
            Ok(message) => message,
            Err(_) => {
                return;
            }
        };

        on_new_message(message, &connection, &client, &storage).await;
    }

    on_close_connection(&user_id, &client, &storage).await;
}
