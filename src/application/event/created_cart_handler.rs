use async_trait::async_trait;

use crate::infrastructure::message_bus::{
    event_kind::EventKind,
    handler::MessageHandler,
    message::{Message, MessageKind},
};

#[derive(Clone)]
pub struct CreatedCartEventHandler {}

impl CreatedCartEventHandler {
    pub fn new() -> CreatedCartEventHandler {
        CreatedCartEventHandler {}
    }
}

#[async_trait]
impl MessageHandler for CreatedCartEventHandler {
    fn message_kind(&self) -> MessageKind {
        MessageKind::Event(EventKind::CreatedCart)
    }

    async fn handle(&self, _message: &Message) -> () {
        println!("Handling CreatedCartEvent!");
    }
}
