use std::fmt::Debug;

#[derive(Debug)]
pub struct Message {
    kind: String,
    data: bson::Bson,
}

impl Message {
    pub fn new(kind: String, data: bson::Bson) -> Message {
        Message { kind, data }
    }

    pub fn kind(&self) -> &str {
        &self.kind
    }

    pub fn data(&self) -> &bson::Bson {
        &self.data
    }
}
