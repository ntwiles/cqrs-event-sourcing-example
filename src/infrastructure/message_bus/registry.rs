use crate::infrastructure::persistence::events::EventService;

use super::{handler::MessageHandler, message::MessageKind};

pub struct HandlerRegistry {
    handlers: Vec<Box<dyn MessageHandler>>,
}

impl HandlerRegistry {
    pub fn new(_event_store: &EventService) -> HandlerRegistry {
        HandlerRegistry {
            handlers: Vec::new(),
        }
    }

    pub fn add(&mut self, handler: Box<dyn MessageHandler>) {
        self.handlers.push(handler);
    }

    pub fn get_handlers(
        &self,
        message_kind: MessageKind,
    ) -> impl Iterator<Item = &Box<dyn MessageHandler>> + '_ {
        self.handlers
            .iter()
            .filter(move |h| h.message_kind() == message_kind)
    }
}
