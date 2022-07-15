use crate::application::event::added_to_cart_event::AddedToCartEvent;
use crate::services::{
    message_bus::event::{Event, EventType},
    persistence::event_store::EventStore,
};

use super::add_to_cart_command::AddToCartCommand;

pub struct AddToCartCommandHandler {
    event_store: EventStore,
}

impl AddToCartCommandHandler {
    pub fn new() -> AddToCartCommandHandler {
        AddToCartCommandHandler {
            event_store: EventStore::new(),
        }
    }

    pub async fn handle(&self, command: AddToCartCommand) -> () {
        let data = AddedToCartEvent::new(
            command.cart_id().clone(),
            command.customer_id().clone(),
            command.offering_id().clone(),
            command.quantity().clone(),
        );

        let event = Event::new(EventType::AddedToCart(data));

        self.event_store.write_event(event).await;
    }
}
