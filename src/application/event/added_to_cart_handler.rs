use std::any::TypeId;

use crate::services::message_bus::{handler::MessageHandler, message::Message};

use super::added_to_cart_event::AddedToCartEvent;

pub struct AddedToCartEventHandler {}

impl AddedToCartEventHandler {
    pub fn new() -> AddedToCartEventHandler {
        AddedToCartEventHandler {}
    }
}

impl MessageHandler for AddedToCartEventHandler {
    fn message_type(&self) -> TypeId {
        TypeId::of::<AddedToCartEvent>()
    }

    fn handle(&self, message: &dyn Message) -> () {
        println!("Handling AddedToCartEvent!");
    }
}
