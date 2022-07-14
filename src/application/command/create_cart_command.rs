use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct CreateCartCommand {
    customer_id: Uuid,
}

impl CreateCartCommand {
    pub fn new(customer_id: Uuid) -> CreateCartCommand {
        CreateCartCommand { customer_id }
    }
    pub fn customer_id(&self) -> &Uuid {
        &self.customer_id
    }
}
