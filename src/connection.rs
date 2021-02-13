use crate::database::User;
use actix::{
    prelude::Message as ActixMessage, Actor, ActorContext, AsyncContext, Handler, StreamHandler,
};
use actix_web_actors::ws::{Message as WebSocketMessage, ProtocolError, WebsocketContext};
use std::time::{Duration, Instant};
use uuid::Uuid;

#[derive(ActixMessage)]
#[rtype(result = "()")]
pub struct Message(pub String);

pub struct Connection {
    user: User,
    last_ping: Instant,
}

impl Connection {
    pub fn new(user: User) -> Self {
        Self {
            user,
            last_ping: Instant::now(),
        }
    }

    pub fn get_id(&self) -> Uuid {
        self.user.get_data().get_id()
    }

    fn start_ping(&self, context: &mut WebsocketContext<Self>) {
        context.run_interval(Duration::from_secs(5), |actor, context| {
            if Instant::now().duration_since(actor.last_ping) > Duration::from_secs(10) {
                context.stop();
                return;
            }

            context.ping(b"");
        });
    }
}

impl StreamHandler<Result<WebSocketMessage, ProtocolError>> for Connection {
    fn handle(
        &mut self,
        message: Result<WebSocketMessage, ProtocolError>,
        context: &mut Self::Context,
    ) {
        let message = match message {
            Err(_) => {
                context.stop();
                return;
            }
            Ok(message) => message,
        };

        match message {
            WebSocketMessage::Ping(message) => {
                self.last_ping = Instant::now();
                context.ping(&message);
            }
            WebSocketMessage::Pong(_) => {
                self.last_ping = Instant::now();
            }
            WebSocketMessage::Text(text) => {
                let text = text.trim();
                println!(
                    "New Message - User ID: {} - Message: {}",
                    self.get_id(),
                    text
                );
            }
            WebSocketMessage::Close(reason) => {
                println!(
                    "Close Connection - User ID: {} - Reason: {:?}",
                    self.get_id(),
                    reason
                );
            }
            _ => unimplemented!("Unimplemented feature."),
        }
    }
}

impl Handler<Message> for Connection {
    type Result = ();

    fn handle(&mut self, message: Message, context: &mut Self::Context) {
        context.text(message.0);
    }
}

impl Actor for Connection {
    type Context = WebsocketContext<Self>;

    fn started(&mut self, context: &mut Self::Context) {
        println!("Username: {}", self.user.get_data().get_username());
        self.start_ping(context);
    }
}
