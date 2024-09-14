// messenger module: provides a simple interface for messaging functionality

mod ffi;
mod domain;
mod repository;

use std::sync::Arc;
use tokio::sync::Mutex;
use thiserror::Error;

pub use domain::{Topic, SubscriberId, Message, Callback};
use repository::SubscriptionRepository;

#[derive(Error, Debug)]
pub enum MessengerError {
    #[error("Failed to initialize messenger: {0}")]
    InitializationError(String),
    #[error("Failed to send message: {0}")]
    SendError(String),
    #[error("Failed to subscribe: {0}")]
    SubscribeError(String),
    #[error("Failed to unsubscribe: {0}")]
    UnsubscribeError(String),
    #[error("Invalid subscriber ID: {0}")]
    InvalidSubscriberId(String),
}

pub struct Messenger {
    inner: Arc<Mutex<ffi::FFIMessenger>>,
    repository: Arc<SubscriptionRepository>,
}

impl Messenger {
    
    // create a new messenger instance
    pub async fn new(library_path: &str) -> Result<Self, MessengerError> {
        let ffi_messenger = ffi::FFIMessenger::new(library_path)
            .map_err(|e| MessengerError::InitializationError(e.to_string()))?;
        Ok(Self {
            inner: Arc::new(Mutex::new(ffi_messenger)),
            repository: Arc::new(SubscriptionRepository::new()),
        })
    }

    // send a message to a specific topic
    pub async fn send(&self, topic: &str, message: &[u8]) -> Result<bool, MessengerError> {
        let messenger = self.inner.lock().await;
        messenger.send(topic, message)
            .map_err(|e| MessengerError::SendError(e.to_string()))
    }

    // subscribe to a topic and receive messages
    pub async fn subscribe(&self, topic: &str, callback: Callback) -> Result<SubscriberId, MessengerError> {
        let messenger = self.inner.lock().await;
        let subscriber_id = messenger.subscribe(topic, callback.clone())
            .map_err(|e| MessengerError::SubscribeError(e.to_string()))?;

        let (tx, mut rx) = tokio::sync::mpsc::channel(100); // Create a channel

        let subscription = domain::Subscription {
            subscriber: domain::Subscriber {
                id: subscriber_id.clone(),
                topic: topic.to_string(),
                sender: tx, // Add the sender
            },
            callback,
        };

        self.repository.add(subscription).await;

        Ok(subscriber_id)
    }

    // unsubscribe from a topic
    pub async fn unsubscribe(&self, topic: &str, subscriber_id: &SubscriberId) -> Result<(), MessengerError> {
        let messenger = self.inner.lock().await;
        messenger.unsubscribe(topic, subscriber_id)
            .map_err(|e| MessengerError::UnsubscribeError(e.to_string()))?;

        self.repository.remove(&topic.to_string(), subscriber_id).await;
        Ok(())
    }
}

// helper function to convert string to subscriber id
pub fn string_to_subscriber_id(id: &str) -> Option<SubscriberId> {
    SubscriberId::from_string(id)
}