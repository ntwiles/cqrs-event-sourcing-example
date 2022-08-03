use async_trait::async_trait;

use crate::infrastructure::message_bus::{
    event_kind::EventKind,
    handler::MessageHandler,
    message::{Message, MessageKind},
};

#[derive(Clone)]
pub struct AddedToCartEventHandler {}

impl AddedToCartEventHandler {
    pub fn new() -> AddedToCartEventHandler {
        AddedToCartEventHandler {}
    }
}

#[async_trait]
impl MessageHandler for AddedToCartEventHandler {
    fn message_kind(&self) -> MessageKind {
        MessageKind::Event(EventKind::AddedToCart)
    }

    async fn handle(&self, _message: &Message) -> () {
        todo!()
    }
}
