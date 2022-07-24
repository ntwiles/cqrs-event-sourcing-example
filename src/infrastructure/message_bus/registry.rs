use std::any::TypeId;

use crate::infrastructure::persistence::events::EventsService;

use super::handler::MessageHandler;

pub struct HandlerRegistry {
    handlers: Vec<Box<dyn MessageHandler>>,
}

impl HandlerRegistry {
    pub fn new(_event_store: &EventsService) -> HandlerRegistry {
        HandlerRegistry {
            handlers: Vec::new(),
        }
    }

    pub fn add(&mut self, handler: Box<dyn MessageHandler>) {
        self.handlers.push(handler);
    }

    pub fn get_handlers(
        &self,
        message_type: TypeId,
    ) -> impl Iterator<Item = &Box<dyn MessageHandler>> + '_ {
        self.handlers
            .iter()
            .filter(move |h| h.message_type() == message_type)
    }
}
