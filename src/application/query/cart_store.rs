use bson::oid;

use crate::domain::cart::Cart;
use crate::infrastructure::persistence::events::EventsService;

pub struct CartStore {
    events_service: EventsService,
}

impl CartStore {
    pub fn get(&self, customer_id: oid::ObjectId) -> Cart {
        let result = self.events_service.find_events(customer_id);

        Cart::new()
    }
}
