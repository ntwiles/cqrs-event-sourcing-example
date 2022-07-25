pub mod event;
pub mod handler;
pub mod message;
pub mod queue;
pub mod registry;

use futures::lock::Mutex;
use tokio;

use std::sync::Arc;

use queue::MessageQueue;
use registry::HandlerRegistry;

pub fn start_message_loop(queue: Arc<Mutex<MessageQueue>>, registry: HandlerRegistry) {
    tokio::spawn(async move {
        loop {
            let message = queue.lock().await.pop_queue();

            if let Some(message) = message {
                let handlers = registry.get_handlers(message.kind().to_string());

                for handler in handlers {
                    handler.handle(&message).await;
                }
            }
        }
    });
}
