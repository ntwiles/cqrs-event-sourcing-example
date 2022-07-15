use crate::application::event::added_to_cart_event::AddedToCartEvent;
use crate::services::message_bus::bus::MessageBus;

use super::add_to_cart_command::AddToCartCommand;

pub struct AddToCartCommandHandler<'a> {
    message_bus: &'a MessageBus<'a>,
}

impl<'a> AddToCartCommandHandler<'a> {
    pub fn new(message_bus: &'a MessageBus) -> AddToCartCommandHandler<'a> {
        AddToCartCommandHandler { message_bus }
    }

    pub async fn handle(&self, command: AddToCartCommand) -> () {
        let event = AddedToCartEvent::new(
            command.cart_id().clone(),
            command.customer_id().clone(),
            command.offering_id().clone(),
            command.quantity().clone(),
        );

        self.message_bus.raise_event(Box::new(event));
    }
}
