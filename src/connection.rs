use actix::{Actor, ActorContext, AsyncContext, Handler, prelude::Message as ActixMessage, StreamHandler};
use actix_web_actors::ws::{Message as WebSocketMessage, ProtocolError, WebsocketContext};
use std::time::{Duration, Instant};
use uuid::Uuid;

#[derive(ActixMessage)]
#[rtype(result = "()")]
pub struct Message(pub String);

pub struct Connection {
    id: Uuid,
    last_ping: Instant,
}

impl Connection {
    pub fn new(id: Uuid) -> Self {
        Self {
            id,
            last_ping: Instant::now(),
        }
    }

    pub fn get_id(&self) -> Uuid {
        self.id
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
    fn handle(&mut self, message: Result<WebSocketMessage, ProtocolError>, context: &mut Self::Context) {
        let message = match message {
            Err(_) => {
                context.stop();
                return;
            },
            Ok(message) => message,
        };

        match message {
            WebSocketMessage::Ping(message) => {
                self.last_ping = Instant::now();
                context.ping(&message);
            },
            WebSocketMessage::Pong(_) => {
                self.last_ping = Instant::now();
            },
            WebSocketMessage::Text(text) => {
                let text = text.trim();
                println!("User ID: {}", self.get_id());
                println!("Text from sockets: {}", text);
            },
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
        self.start_ping(context);
    }
}
