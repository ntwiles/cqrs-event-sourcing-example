use bson::oid;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct RemoveFromCartCommand {
    customer_id: oid::ObjectId,
    offering_id: oid::ObjectId,
}

impl RemoveFromCartCommand {
    pub fn customer_id(&self) -> &oid::ObjectId {
        &self.customer_id
    }

    pub fn offering_id(&self) -> &oid::ObjectId {
        &self.offering_id
    }
}
