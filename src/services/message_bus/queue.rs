use std::{sync::Arc, thread};

use crate::services::message_bus::{event::EventData, message::MessageData};
use crate::services::persistence::event_store::EventStore;

use super::message::Message;

pub struct MessageQueue {
    queue: Vec<Message>,
    event_store: Arc<EventStore>,
}

impl MessageQueue {
    pub fn new(event_store: Arc<EventStore>) -> MessageQueue {
        MessageQueue {
            event_store,
            queue: Vec::new(),
        }
    }

    pub fn pop_queue(&mut self) -> Option<Message> {
        self.queue.pop()
    }

    pub fn raise_event<T: 'static + MessageData + EventData>(&mut self, event: Message) {
        let store = self.event_store.clone();
        let data = (*(event.data()))
            .as_any()
            .downcast_ref::<T>()
            .unwrap()
            .clone();

        thread::spawn(move || async move { store.write_event::<T>(&data).await });

        self.send(event);
    }

    pub fn send(&mut self, message: Message) {
        println!("Sending message: {:?}", message.code());
        self.queue.push(message);
    }
}
