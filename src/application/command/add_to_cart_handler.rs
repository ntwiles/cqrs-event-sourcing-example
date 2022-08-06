use async_trait::async_trait;
use futures::lock::Mutex;

use std::sync::Arc;

use crate::{
    application::event::cart_item_added_event::CartItemAddedEvent,
    infrastructure::message_bus::{
        command_kind::CommandKind,
        event_kind::EventKind,
        handler::MessageHandler,
        message::{Message, MessageKind},
        queue::MessageQueue,
    },
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
    fn message_kind(&self) -> MessageKind {
        MessageKind::Command(CommandKind::AddToCart)
    }

    async fn handle(&self, message: &Message) -> () {
        let command: AddToCartCommand = bson::from_bson(message.data().clone()).unwrap();

        let event =
            CartItemAddedEvent::new(command.offering_id().clone(), command.quantity().clone());

        let data = bson::to_bson(&event).unwrap();

        self.message_queue
            .lock()
            .await
            .raise_event(*command.customer_id(), EventKind::CartItemAdded, data)
            .await
    }
}
