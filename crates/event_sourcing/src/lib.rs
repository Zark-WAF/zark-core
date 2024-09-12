// MIT License
// 
// Copyright (c) 2024 ZARK-WAF
// 
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:

// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.

// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
//
// Authors: I. Zeqiri, E. Gjergji

mod event_store;
mod event_replay;
mod error;

use std::sync::Arc;
use tokio::sync::RwLock;
use zark_waf_common::messaging::messenger::ZarkMessenger;

pub use event_store::EventStore;
pub use event_replay::EventReplay;
pub use error::EventSourcingError;

pub struct EventSourcingSystem {
    event_store: Arc<RwLock<EventStore>>,
    event_replay: Arc<RwLock<EventReplay>>,
    messenger: Arc<ZarkMessenger>,
}

impl EventSourcingSystem {
    pub async fn new(config: EventSourcingConfig, messenger: Arc<ZarkMessenger>) -> Result<Self, EventSourcingError> {
        let event_store = Arc::new(RwLock::new(EventStore::new(config.event_store_config)?));
        let event_replay = Arc::new(RwLock::new(EventReplay::new(config.event_replay_config)?));

        Ok(Self {
            event_store,
            event_replay,
            messenger,
        })
    }

    pub async fn append_event(&self, event: Event) -> Result<(), EventSourcingError> {
        self.event_store.write().await.append(event).await?;
        Ok(())
    }

    pub async fn get_events(&self, start: u64, end: Option<u64>) -> Result<Vec<Event>, EventSourcingError> {
        self.event_store.read().await.get_events(start, end).await
    }

    pub async fn replay_events(&self, target_version: Option<u64>) -> Result<State, EventSourcingError> {
        self.event_replay.write().await.replay(self.event_store.clone(), target_version).await
    }
}

#[derive(Clone, Debug)]
pub struct EventSourcingConfig {
    pub event_store_config: EventStoreConfig,
    pub event_replay_config: EventReplayConfig,
}

#[derive(Clone, Debug)]
pub struct EventStoreConfig {
   //TODO: Add configuration options for event store
}

#[derive(Clone, Debug)]
pub struct EventReplayConfig {
    //TODO: Add configuration options for event replay
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Event {
    pub id: uuid::Uuid,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub event_type: String,
    pub data: serde_json::Value,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct State {
    //TODO: Define your state structure here
}

