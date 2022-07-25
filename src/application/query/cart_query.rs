use bson::oid;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CartQuery {
    customer_id: oid::ObjectId,
}

impl CartQuery {
    pub fn customer_id(&self) -> &oid::ObjectId {
        &self.customer_id
    }
}
