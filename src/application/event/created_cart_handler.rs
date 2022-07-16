use async_trait::async_trait;

use std::any::TypeId;

use crate::services::message_bus::{handler::MessageHandler, message::Message};

use super::created_cart_event::CreatedCartEvent;

#[derive(Clone)]
pub struct CreatedCartEventHandler {}

impl CreatedCartEventHandler {
    pub fn new() -> CreatedCartEventHandler {
        CreatedCartEventHandler {}
    }
}

#[async_trait]
impl MessageHandler for CreatedCartEventHandler {
    fn message_type(&self) -> TypeId {
        TypeId::of::<CreatedCartEvent>()
    }

    async fn handle(&self, message: &dyn Message) -> () {
        println!("Handling CreatedCartEvent!");
    }
}
