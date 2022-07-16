use std::any::{Any, TypeId};

pub trait MessageData: Send + Sync {
    fn as_any(&self) -> &dyn Any;
}

pub struct Message {
    code: TypeId,
    data: Box<dyn MessageData>,
}

impl Message {
    pub fn new<T: 'static + MessageData>(data: T) -> Message {
        Message {
            code: TypeId::of::<T>(),
            data: Box::new(data),
        }
    }
    pub fn code(&self) -> TypeId {
        self.code
    }

    pub fn data(&self) -> &Box<dyn MessageData> {
        &self.data
    }
}
