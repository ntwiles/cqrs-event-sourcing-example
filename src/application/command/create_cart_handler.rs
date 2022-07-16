use async_trait::async_trait;

use std::{
    any::TypeId,
    sync::{Arc, Mutex},
};

use crate::application::event::created_cart_event::CreatedCartEvent;
use crate::services::message_bus::{
    queue::MessageQueue,
    {handler::MessageHandler, message::Message},
};

use super::create_cart_command::{CreateCartCommand, CreateCartCommandData};

pub struct CreateCartCommandHandler {
    message_queue: Arc<Mutex<MessageQueue>>,
}

impl CreateCartCommandHandler {
    pub fn new(message_queue: Arc<Mutex<MessageQueue>>) -> CreateCartCommandHandler {
        CreateCartCommandHandler { message_queue }
    }
}

#[async_trait]
impl MessageHandler for CreateCartCommandHandler {
    fn message_type(&self) -> TypeId {
        TypeId::of::<CreateCartCommand>()
    }

    fn handle(&self, message: &dyn Message) {
        let command: &CreateCartCommandData = message
            .data()
            .as_any()
            .downcast_ref::<CreateCartCommandData>()
            .unwrap();

        let event = CreatedCartEvent::new(command.customer_id().clone());

        self.message_queue
            .lock()
            .unwrap()
            .raise_event(Box::new(event));
    }
}
