use async_trait::async_trait;

use crate::infrastructure::message_bus::{handler::MessageHandler, message::Message};

#[derive(Clone)]
pub struct AddedToCartEventHandler {}

impl AddedToCartEventHandler {
    pub fn new() -> AddedToCartEventHandler {
        AddedToCartEventHandler {}
    }
}

#[async_trait]
impl MessageHandler for AddedToCartEventHandler {
    fn message_kind(&self) -> String {
        "addedToCart".to_string()
    }

    async fn handle(&self, _message: &Message) -> () {
        todo!()
    }
}
