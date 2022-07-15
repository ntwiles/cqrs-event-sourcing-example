use crate::services::persistence::event_store::EventStore;

use super::{handler::MessageHandler, message::Message};

#[derive(Clone)]
pub struct MessageBus<'a> {
    handlers: Vec<&'a dyn MessageHandler>,
    event_store: &'a EventStore,
}

impl<'a> MessageBus<'a> {
    pub fn new(event_store: &'a EventStore) -> MessageBus<'a> {
        MessageBus {
            event_store,
            handlers: Vec::new(),
        }
    }

    pub fn register_handler(&mut self, handler: &'a dyn MessageHandler) {
        self.handlers.push(handler);
    }

    pub async fn raise_event(&self, event: Box<dyn Message>) {
        // self.event_store.write_event(*event).await;

        self.send(event)
    }

    pub fn send(&self, message: Box<dyn Message>) {
        for handler in &self.handlers {
            if handler.message_type() == message.message_type() {
                handler.handle(&*message);
            }
        }
    }
}
