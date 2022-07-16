use std::any::TypeId;

pub trait Message: Send + Sync {
    fn message_type(&self) -> TypeId;
}
