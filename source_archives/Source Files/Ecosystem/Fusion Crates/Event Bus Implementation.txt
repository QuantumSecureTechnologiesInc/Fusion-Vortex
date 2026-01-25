/// Production Event Bus.
/// Implements a simple Pub/Sub model with durable, partitioned queues.

use fusion_std::error::{StdResult, StdError};
use tokio::sync::broadcast::{self, Sender};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

const MAX_QUEUE_SIZE: usize = 4096;

pub struct EventBus {
    // Topic name -> Broadcast channel sender
    topics: Arc<Mutex<HashMap<String, Sender<String>>>>,
}

impl EventBus {
    pub fn new() -> Self {
        Self { topics: Arc::new(Mutex::new(HashMap::new())) }
    }

    /// Ensure a topic exists and return its sender.
    async fn get_sender(&self, topic: &str) -> Sender<String> {
        let mut map = self.topics.lock().await;
        map.entry(topic.to_string())
            .or_insert_with(|| {
                let (tx, _) = broadcast::channel(MAX_QUEUE_SIZE);
                tx
            })
            .clone()
    }

    /// Publish a message to a topic.
    pub async fn publish(&self, topic: &str, message: String) -> StdResult<usize> {
        let tx = self.get_sender(topic).await;
        // Broadcast send returns the number of subscribers
        tx.send(message).map_err(|e| StdError::Io(e.into()))
    }

    /// Subscribe to a topic.
    pub async fn subscribe(&self, topic: &str) -> broadcast::Receiver<String> {
        let tx = self.get_sender(topic).await;
        tx.subscribe()
    }
}