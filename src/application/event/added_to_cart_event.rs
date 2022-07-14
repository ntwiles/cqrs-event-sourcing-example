use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct AddedToCartEventData {
    customer_id: Uuid,
    cart_id: Uuid,
    offering_id: Uuid,
    quantity: u8,
}

impl AddedToCartEventData {
    pub fn new(
        cart_id: Uuid,
        customer_id: Uuid,
        offering_id: Uuid,
        quantity: u8,
    ) -> AddedToCartEventData {
        AddedToCartEventData {
            cart_id,
            customer_id,
            offering_id,
            quantity,
        }
    }
}
