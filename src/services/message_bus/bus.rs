use crate::services::persistence::event_store::EventStore;

use super::{handler::MessageHandler, message::Message};

pub struct MessageBus {
    handlers: Vec<Box<dyn MessageHandler>>,
    // event_store: &'a EventStore,
}

impl MessageBus {
    pub fn new(_event_store: &EventStore) -> MessageBus {
        MessageBus {
            // event_store,
            handlers: Vec::new(),
        }
    }

    pub fn register_handler(&mut self, handler: Box<dyn MessageHandler>) {
        self.handlers.push(handler);
    }

    pub async fn raise_event(&self, event: Box<dyn Message>) {
        // self.event_store.write_event(*event).await;

        self.send(event).await
    }

    pub async fn send(&self, message: Box<dyn Message>) {
        for handler in &self.handlers {
            if handler.message_type() == message.message_type() {
                handler.handle(&*message).await;
            }
        }
    }
}
