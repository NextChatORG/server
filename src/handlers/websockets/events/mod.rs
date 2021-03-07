use super::StorageType;
use crate::database::{get_now_time, models::UserModel};
use futures::{FutureExt, StreamExt};
use sqlx::PgPool;
use tokio::sync::mpsc;
use tokio_stream::wrappers::UnboundedReceiverStream;
use uuid::Uuid;
use warp::ws::{Message, WebSocket};

pub async fn on_user_connected(ws: WebSocket, user_id: Uuid, client: PgPool, storage: StorageType) {
    let storage2 = &storage;
    let mut storage = storage.write().await;

    // Split the socket into a sender and receive of messages.
    let (user_ws_tx, mut user_ws_rx) = ws.split();
    let (tx, rx) = mpsc::unbounded_channel();
    let rx = UnboundedReceiverStream::new(rx);

    tokio::task::spawn(rx.forward(user_ws_tx).map(|result| {
        if let Err(e) = result {
            eprintln!("WebSocket send error: {:?}", e);
        }
    }));

    // Add the user sender handler to the users connections list.
    storage.connections.insert(user_id, tx);

    // Set the user model with the user from its id.
    storage.me = UserModel::from_id(&client, &user_id, false)
        .await
        .expect("Cannot get the user by its id.");

    // Set the online status to true.
    storage.me.set_online(true);

    // Set the last online to the now timestamp.
    storage.me.set_last_online(get_now_time());

    // Update the online status and last online of the user at the database.
    sqlx::query("UPDATE users SET online = true, last_online = $2 WHERE id = $1")
        .bind(user_id)
        .bind(storage.me.get_last_online())
        .execute(&client)
        .await
        .expect("Cannot update the user.");

    println!("User connected: {}", user_id);

    // Read messages.
    while let Some(result) = user_ws_rx.next().await {
        // Get the message struct.
        let message = match result {
            Ok(msg) => msg,
            Err(e) => {
                eprintln!("WebSocket error (User ID: {}): {:?}", user_id, e);
                return;
            }
        };

        // Call on message event.
        on_message(&user_id, message, &client).await;
    }

    // Call on user disconnected event.
    on_user_disconnected(&user_id, &client, storage2).await;
}

async fn on_message(user_id: &Uuid, message: Message, _client: &PgPool) {
    let message = if let Ok(s) = message.to_str() {
        s
    } else {
        return;
    };

    // TODO: Messages events, format: /{name} {arguments}
    println!("User ID: {} - Message: {}", user_id, message);
}

async fn on_user_disconnected(user_id: &Uuid, client: &PgPool, storage: &StorageType) {
    let mut storage = storage.write().await;

    // Set the online status to false.
    storage.me.set_online(false);

    // Set the last online to the now timestamp.
    storage.me.set_last_online(get_now_time());

    // Update the online status and last online of the user at the database.
    sqlx::query("UPDATE users SET online = false, last_online = $2 WHERE id = $1")
        .bind(user_id)
        .bind(storage.me.get_last_online())
        .execute(client)
        .await
        .expect("Cannot update the user.");

    // Remove the current user from the users connections list.
    storage.connections.remove(user_id);

    println!("User disconnected: {}", user_id);
}
