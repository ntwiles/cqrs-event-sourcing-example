use async_trait::async_trait;
use uuid::Uuid;

use std::{
    any::TypeId,
    sync::{Arc, Mutex},
};

use crate::application::event::created_cart_event::CreatedCartEvent;
use crate::services::message_bus::{
    queue::MessageQueue,
    {handler::MessageHandler, message::Message},
};

use super::create_cart_command::CreateCartCommand;

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

    fn handle(&self, _message: &dyn Message) {
        println!("Handling CreateCartCommand.");
        // let event = CreatedCartEvent::new(message.customer_id().clone());

        let fake_event = CreatedCartEvent::new(Uuid::new_v4());

        self.message_queue
            .lock()
            .unwrap()
            .raise_event(Box::new(fake_event));
    }
}
