use std::any::TypeId;

use crate::services::message_bus::{handler::MessageHandler, message::Message};

use super::created_cart_event::CreatedCartEvent;

pub struct CreatedCartEventHandler {}

impl CreatedCartEventHandler {
    pub fn new() -> CreatedCartEventHandler {
        CreatedCartEventHandler {}
    }
}

impl MessageHandler for CreatedCartEventHandler {
    fn message_type(&self) -> TypeId {
        TypeId::of::<CreatedCartEvent>()
    }

    fn handle(&self, message: &dyn Message) -> () {
        println!("Handling CreatedCartEvent!");
    }
}
