use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct AddToCartCommand {
    // cart_id: Uuid,
    customer_id: Uuid,
    // offering_id: Uuid,
    // quantity: u8,
}

impl AddToCartCommand {
    // pub fn cart_id(&self) -> &Uuid {
    //     &self.cart_id
    // }

    pub fn customer_id(&self) -> &Uuid {
        &self.customer_id
    }

    // pub fn offering_id(&self) -> &Uuid {
    //     &self.offering_id
    // }

    // pub fn quantity(&self) -> &u8 {
    //     &self.quantity
    // }
}
