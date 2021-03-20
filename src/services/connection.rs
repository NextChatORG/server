//! NextChat Server connection service module.
//!
//! This module contains the `on_message`, `on_close_connection` and `on_new_connection` events
//! of a WebSocket connection.

use futures::{FutureExt, StreamExt};
use sqlx::PgPool;
use tokio::sync::mpsc;
use tokio_stream::wrappers::UnboundedReceiverStream;
use uuid::Uuid;
use warp::ws::{Message, WebSocket};

use crate::{core::storage::StorageType, models::connection::Connection};

/// This functions handles all received messages and run the correspondent event.
async fn on_new_message(
    user_id: &Uuid,
    message: Message,
    _client: &PgPool,
    _storage: &StorageType,
) {
    let message = match message.to_str() {
        Ok(message) => message,
        Err(_) => {
            return;
        }
    };

    // TODO: Messages events handler, format: /{name} {arguments}
    println!("User ID: {} - Message: {}", user_id, message);
}

/// Remove a connection from the storage.
async fn on_close_connection(user_id: &Uuid, _client: &PgPool, storage: &StorageType) {
    let mut storage = storage.write().await;

    storage.remove_connection(user_id);

    println!("User disconnected: {}", user_id);
}

/// This functions handle the socket messages and add the new connection
/// to the storage.
pub async fn on_new_connection(
    socket: WebSocket,
    user_id: Uuid,
    client: PgPool,
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

    storage_mut.add_connection(&user_id, Connection::new(&user_id, &tx));

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

        on_new_message(&user_id, message, &client, &storage).await;
    }

    on_close_connection(&user_id, &client, &storage).await;
}
