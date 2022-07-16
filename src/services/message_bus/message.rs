use std::any::{Any, TypeId};

pub trait MessageData {
    fn as_any(&self) -> &dyn Any;
}

// TODO: Maybe this should be a struct instead of a trait since we've offloaded data.
pub trait Message: Send + Sync {
    fn code(&self) -> TypeId;
    fn data(&self) -> &dyn MessageData;
}
