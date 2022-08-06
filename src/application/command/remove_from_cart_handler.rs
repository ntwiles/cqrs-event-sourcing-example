use async_trait::async_trait;
use futures::lock::Mutex;

use std::sync::Arc;

use crate::{
    application::event::added_to_cart_event::AddedToCartEvent,
    infrastructure::message_bus::{handler::MessageHandler, message::Message, queue::MessageQueue},
};

use super::add_to_cart_command::AddToCartCommand;

pub struct RemoveFromCartCommandHandler {
    message_queue: Arc<Mutex<MessageQueue>>,
}

impl RemoveFromCartCommandHandler {
    pub fn new(message_queue: Arc<Mutex<MessageQueue>>) -> RemoveFromCartCommandHandler {
        RemoveFromCartCommandHandler { message_queue }
    }
}

#[async_trait]
impl MessageHandler for RemoveFromCartCommandHandler {
    fn message_kind(&self) -> String {
        "removeFromCart".to_string()
    }

    async fn handle(&self, message: &Message) -> () {
        let command: RemoveFromCartCommand = bson::from_bson(message.data().clone()).unwrap();

        let event =
            AddedToCartEvent::new(command.offering_id().clone(), command.quantity().clone());

        let data = bson::to_bson(&event).unwrap();

        self.message_queue
            .lock()
            .await
            .raise_event(*command.customer_id(), "addedToCart".to_string(), data)
            .await
    }
}
