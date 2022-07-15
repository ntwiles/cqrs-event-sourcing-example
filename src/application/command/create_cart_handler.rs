use crate::application::event::created_cart_event::CreatedCartEvent;
use crate::services::message_bus::bus::MessageBus;

use super::create_cart_command::CreateCartCommand;

pub struct CreateCartCommandHandler<'a> {
    message_bus: &'a MessageBus<'a>,
}

impl<'a> CreateCartCommandHandler<'a> {
    pub fn new(message_bus: &'a MessageBus) -> CreateCartCommandHandler<'a> {
        CreateCartCommandHandler { message_bus }
    }

    pub async fn handle(&self, command: CreateCartCommand) -> () {
        let event = CreatedCartEvent::new(command.customer_id().clone());

        self.message_bus.raise_event(Box::new(event));
    }
}
