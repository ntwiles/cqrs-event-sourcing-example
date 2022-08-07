use bson::oid;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct RemoveFromCartCommand {
    pub customer_id: oid::ObjectId,
    pub product_id: oid::ObjectId,
}
