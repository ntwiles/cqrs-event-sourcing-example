use async_trait::async_trait;

use crate::infrastructure::message_bus::{
    event_kind::EventKind,
    handler::MessageHandler,
    message::{Message, MessageKind},
};

#[derive(Clone)]
pub struct CartItemAddedHandler {}

impl CartItemAddedHandler {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl MessageHandler for CartItemAddedHandler {
    fn message_kind(&self) -> MessageKind {
        MessageKind::Event(EventKind::CartItemAdded)
    }

    async fn handle(&self, _message: &Message) -> () {
        todo!()
    }
}
