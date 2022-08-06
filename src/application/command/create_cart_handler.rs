use async_trait::async_trait;
use futures::lock::Mutex;

use std::sync::Arc;

use crate::{
    application::event::user_cart_created_event::UserCartCreatedEvent,
    infrastructure::message_bus::{
        command_kind::CommandKind,
        event_kind::EventKind,
        queue::MessageQueue,
        {
            handler::MessageHandler,
            message::{Message, MessageKind},
        },
    },
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
    fn message_kind(&self) -> MessageKind {
        MessageKind::Command(CommandKind::CreateCart)
    }

    async fn handle(&self, message: &Message) {
        let command: CreateCartCommand = bson::from_bson(message.data().clone()).unwrap();
        let event = bson::to_bson(&UserCartCreatedEvent {}).unwrap();

        self.message_queue
            .lock()
            .await
            .raise_event(*command.customer_id(), EventKind::UserCartCreated, event)
            .await;
    }
}
