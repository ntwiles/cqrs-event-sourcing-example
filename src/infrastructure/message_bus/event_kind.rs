use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum EventKind {
    UserCartCreated,
    CartItemAdded,
    CartItemRemoved,
}
