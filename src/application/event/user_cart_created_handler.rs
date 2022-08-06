use async_trait::async_trait;

use crate::infrastructure::message_bus::{
    event_kind::EventKind,
    handler::MessageHandler,
    message::{Message, MessageKind},
};

#[derive(Clone)]
pub struct UserCartCreatedHandler {}

impl UserCartCreatedHandler {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl MessageHandler for UserCartCreatedHandler {
    fn message_kind(&self) -> MessageKind {
        MessageKind::Event(EventKind::UserCartCreated)
    }

    async fn handle(&self, _message: &Message) -> () {
        println!("Handling CreatedCartEvent!");
    }
}
