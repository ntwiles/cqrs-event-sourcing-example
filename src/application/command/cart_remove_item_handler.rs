use async_trait::async_trait;
use futures::lock::Mutex;

use std::sync::Arc;

use crate::{
    application::{
        event::cart_item_removed_event::CartItemRemovedEvent, query::product::ProductStore,
    },
    infrastructure::message_bus::{
        command_kind::CommandKind,
        event_kind::EventKind,
        handler::MessageHandler,
        message::{Message, MessageKind},
        queue::MessageQueue,
    },
};

use super::cart_remove_item_command::CartRemoveItemCommand;

pub struct CartRemoveItemHandler {
    message_queue: Arc<Mutex<MessageQueue>>,
    product_store: Arc<ProductStore>,
}

impl CartRemoveItemHandler {
    pub fn new(message_queue: Arc<Mutex<MessageQueue>>, product_store: Arc<ProductStore>) -> Self {
        Self {
            message_queue,
            product_store,
        }
    }
}

#[async_trait]
impl MessageHandler for CartRemoveItemHandler {
    fn message_kind(&self) -> MessageKind {
        MessageKind::Command(CommandKind::AddToCart)
    }

    async fn handle(&self, message: &Message) -> () {
        let command: CartRemoveItemCommand = bson::from_bson(message.data().clone()).unwrap();

        let product = self.product_store.get(command.product_id).await.unwrap();

        let event = CartItemRemovedEvent::new(product);
        let data = bson::to_bson(&event).unwrap();

        self.message_queue
            .lock()
            .await
            .raise_event(command.cart_id, EventKind::CartItemRemoved, data)
            .await
            .unwrap()
    }
}
