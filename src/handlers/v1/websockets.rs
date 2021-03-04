use crate::handlers::with_client;
use futures::{FutureExt, StreamExt};
use serde::Deserialize;
use sqlx::PgPool;
use tokio::sync::mpsc;
use tokio_stream::wrappers::UnboundedReceiverStream;
use uuid::Uuid;
use warp::{
    ws::{Message, WebSocket, Ws},
    Filter, Rejection, Reply,
};

async fn on_user_disconnected(user_id: &Uuid, _client: &PgPool) {
    println!("User disconnected: {}", user_id);
}

async fn on_user_message(user_id: &Uuid, message: Message, _client: &PgPool) {
    let message = if let Ok(s) = message.to_str() {
        s
    } else {
        return;
    };
    println!("User ID: {} - Message: {}", user_id, message);
}

async fn on_user_connected(ws: WebSocket, user_id: Uuid, client: PgPool) {
    println!("User connected: {}", user_id);

    // Split the socket into a sender and receive of messages.
    let (user_ws_tx, mut user_ws_rx) = ws.split();
    let (_tx, rx) = mpsc::unbounded_channel();
    let rx = UnboundedReceiverStream::new(rx);

    tokio::task::spawn(rx.forward(user_ws_tx).map(|result| {
        if let Err(e) = result {
            eprintln!("WebSocket send error: {:?}", e);
        }
    }));

    while let Some(result) = user_ws_rx.next().await {
        let message = match result {
            Ok(msg) => msg,
            Err(e) => {
                eprintln!("WebSocket error (User ID: {}): {:?}", user_id, e);
                return;
            }
        };

        on_user_message(&user_id, message, &client).await;
    }

    on_user_disconnected(&user_id, &client).await;
}

pub fn routes(client: &PgPool) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    #[derive(Deserialize)]
    struct WebSocketQuery {
        pub user_id: Uuid,
    }

    warp::ws()
        .and(super::v1_path_prefix())
        .and(warp::path("ws"))
        .and(warp::query::<WebSocketQuery>())
        .and(with_client(client.clone()))
        .map(|ws: Ws, query: WebSocketQuery, client: PgPool| {
            ws.on_upgrade(move |socket| on_user_connected(socket, query.user_id, client))
        })
}
