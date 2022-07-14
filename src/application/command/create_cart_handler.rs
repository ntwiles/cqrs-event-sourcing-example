use crate::application::event::{created_cart_event::CreatedCartEventData, Event, EventType};
use crate::services::persistence::event_store::EventStore;

use super::create_cart_command::CreateCartCommand;

pub struct CreateCartCommandHandler {
    event_store: EventStore,
}

impl CreateCartCommandHandler {
    pub fn new() -> CreateCartCommandHandler {
        CreateCartCommandHandler {
            event_store: EventStore::new(),
        }
    }

    pub async fn handle(&self, command: CreateCartCommand) -> () {
        let data = CreatedCartEventData::new(command.customer_id().clone());

        let event = Event::new(EventType::CreatedCart(data));

        self.event_store.write_event(event).await;

        // let added_user = self.user_repository.add(user);
        // added_user.register_user();
    }
}
