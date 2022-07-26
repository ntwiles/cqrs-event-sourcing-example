use bson::oid;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateCartCommand {
    customer_id: oid::ObjectId,
}

impl CreateCartCommand {
    // pub fn new(customer_id: oid::ObjectId) -> CreateCartCommand {
    //     CreateCartCommand { customer_id }
    // }

    pub fn customer_id(&self) -> &oid::ObjectId {
        &self.customer_id
    }
}
