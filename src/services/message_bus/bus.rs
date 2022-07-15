use std::any::TypeId;

use super::{event::Event, handler::MessageHandler, message::Message};

#[derive(Clone)]
pub struct MessageBus<'a> {
    handlers: Vec<&'a dyn MessageHandler>,
}

impl<'a> MessageBus<'a> {
    pub fn new() -> MessageBus<'a> {
        MessageBus {
            handlers: Vec::new(),
        }
    }

    pub fn register_handler(&mut self, handler: &'a dyn MessageHandler) {
        self.handlers.push(handler);
    }

    pub fn raise_event(event: Event) {}

    pub fn send(&self, message: &dyn Message) {
        for handler in &self.handlers {
            if handler.message_type() == message.message_type() {
                handler.handle(message);
            }
        }
    }
}
