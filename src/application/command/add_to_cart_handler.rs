use async_trait::async_trait;

use std::{
    any::TypeId,
    sync::{Arc, Mutex},
};

use crate::application::event::added_to_cart_event::AddedToCartEvent;
use crate::services::message_bus::{
    handler::MessageHandler, message::Message, queue::MessageQueue,
};

use super::add_to_cart_command::AddToCartCommand;

pub struct AddToCartCommandHandler {
    message_queue: Arc<Mutex<MessageQueue>>,
}

impl AddToCartCommandHandler {
    pub fn new(message_queue: Arc<Mutex<MessageQueue>>) -> AddToCartCommandHandler {
        AddToCartCommandHandler { message_queue }
    }
}

#[async_trait]
impl MessageHandler for AddToCartCommandHandler {
    fn message_type(&self) -> TypeId {
        TypeId::of::<AddToCartCommand>()
    }

    fn handle(&self, message: &Message) -> () {
        let command = message
            .data()
            .as_any()
            .downcast_ref::<AddToCartCommand>()
            .unwrap();

        let event = AddedToCartEvent::new(
            command.cart_id().clone(),
            command.customer_id().clone(),
            command.offering_id().clone(),
            command.quantity().clone(),
        );

        let message = Message::new_event(event);

        self.message_queue
            .lock()
            .unwrap()
            .raise_event::<AddedToCartEvent>(message);
    }
}
