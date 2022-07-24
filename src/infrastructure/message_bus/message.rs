use std::{
    any::{Any, TypeId},
    fmt::Debug,
};

pub trait MessageData: Debug + Send + Sync {
    fn as_any(&self) -> &dyn Any;
}

#[derive(Debug)]
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

    pub fn data(&self) -> &dyn MessageData {
        &*self.data
    }
}
