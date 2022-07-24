use async_trait::async_trait;
use futures::lock::Mutex;

use std::{any::TypeId, sync::Arc};

use crate::application::event::created_cart_event::CreatedCartEvent;
use crate::infrastructure::message_bus::{
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

    async fn handle(&self, message: &Message) {
        let command = message
            .data()
            .as_any()
            .downcast_ref::<CreateCartCommand>()
            .unwrap();

        let event = CreatedCartEvent::new(command.customer_id().clone());

        self.message_queue.lock().await.raise_event(event).await;
    }
}
