use std::any::TypeId;

pub struct Command {
    message_type: TypeId,
}

impl Message for Command {
    fn message_type(&self) -> TypeId {
        self.message_type
    }
}

pub trait Message {
    fn message_type(&self) -> TypeId;
}
