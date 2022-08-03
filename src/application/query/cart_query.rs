use bson::oid;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CartQuery {
    cart_id: oid::ObjectId,
}

impl CartQuery {
    pub fn cart_id(&self) -> &oid::ObjectId {
        &self.cart_id
    }
}
