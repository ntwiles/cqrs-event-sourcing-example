use crate::services::persistence::event_store::EventStore;

use super::message::Message;

pub struct MessageQueue {
    queue: Vec<Box<dyn Message>>,
    // event_store: &'a EventStore,
}

impl MessageQueue {
    pub fn new(_event_store: &EventStore) -> MessageQueue {
        MessageQueue {
            // event_store,
            queue: Vec::new(),
        }
    }

    pub fn pop_queue(&mut self) -> Option<Box<dyn Message>> {
        self.queue.pop()
    }

    pub fn raise_event(&mut self, event: Box<dyn Message>) {
        // self.event_store.write_event(*event).await;

        self.send(event);
    }

    pub fn send(&mut self, message: Box<dyn Message>) {
        println!("Sending message: {:?}", message.message_type());
        self.queue.push(message);
    }
}
