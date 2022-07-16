use async_trait::async_trait;

use std::any::TypeId;

use crate::application::event::created_cart_event::CreatedCartEvent;
use crate::services::message_bus::{
    bus::MessageBus,
    {handler::MessageHandler, message::Message},
};

use super::create_cart_command::CreateCartCommand;

pub struct CreateCartCommandHandler<'a> {
    // message_bus: &'a MessageBus,
}

impl<'a> CreateCartCommandHandler<'a> {
    pub fn new(message_bus: &'a MessageBus) -> CreateCartCommandHandler<'a> {
        CreateCartCommandHandler { 
            //message_bus 
        }
    }
}

#[async_trait]
impl<'a> MessageHandler for CreateCartCommandHandler<'a> {
    fn message_type(&self) -> TypeId {
        TypeId::of::<CreateCartCommand>()
    }

    async fn handle(&self, message: &dyn Message) -> () {
        let event = CreatedCartEvent::new(message.customer_id().clone());

        // self.message_bus.raise_event(Box::new(event)).await;
    }
}
