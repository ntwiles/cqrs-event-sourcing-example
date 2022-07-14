use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct CreatedCartEventData {
    customer_id: Uuid,
}

impl CreatedCartEventData {
    pub fn new(customer_id: Uuid) -> CreatedCartEventData {
        CreatedCartEventData { customer_id }
    }
}
