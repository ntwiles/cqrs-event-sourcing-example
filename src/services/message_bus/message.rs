use std::any::TypeId;

pub trait Message {
    fn message_type(&self) -> TypeId;
}
