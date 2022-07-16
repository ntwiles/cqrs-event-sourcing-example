use crate::services::persistence::event_store::EventStore;

use super::message::Message;

pub struct MessageQueue {
    queue: Vec<Message>,
    // event_store: &'a EventStore,
}

impl MessageQueue {
    pub fn new(_event_store: &EventStore) -> MessageQueue {
        MessageQueue {
            // event_store,
            queue: Vec::new(),
        }
    }

    pub fn pop_queue(&mut self) -> Option<Message> {
        self.queue.pop()
    }

    pub fn raise_event(&mut self, event: Message) {
        // self.event_store.write_event(*event).await;

        self.send(event);
    }

    pub fn send(&mut self, message: Message) {
        println!("Sending message: {:?}", message.code());
        self.queue.push(message);
    }
}
