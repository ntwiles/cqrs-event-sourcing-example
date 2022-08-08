use async_trait::async_trait;
use futures::lock::Mutex;

use std::sync::Arc;

use crate::{
    application::{event::cart_item_added_event::CartItemAddedEvent, query::product::ProductStore},
    infrastructure::message_bus::{
        command_kind::CommandKind,
        event_kind::EventKind,
        handler::MessageHandler,
        message::{Message, MessageKind},
        queue::MessageQueue,
    },
};

use super::cart_add_item_command::CartAddItemCommand;

pub struct CartAddItemHandler {
    message_queue: Arc<Mutex<MessageQueue>>,
    product_store: Arc<ProductStore>,
}

impl CartAddItemHandler {
    pub fn new(message_queue: Arc<Mutex<MessageQueue>>, product_store: Arc<ProductStore>) -> Self {
        Self {
            message_queue,
            product_store,
        }
    }
}

#[async_trait]
impl MessageHandler for CartAddItemHandler {
    fn message_kind(&self) -> MessageKind {
        MessageKind::Command(CommandKind::AddToCart)
    }

    async fn handle(&self, message: &Message) -> () {
        let command: CartAddItemCommand = bson::from_bson(message.data().clone()).unwrap();

        let product = self.product_store.get(command.product_id).await.unwrap();

        let event = CartItemAddedEvent::new(product, command.quantity);
        let data = bson::to_bson(&event).unwrap();

        self.message_queue
            .lock()
            .await
            .raise_event(command.cart_id, EventKind::CartItemAdded, data)
            .await
            .unwrap();
    }
}
