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

use crate::error::EventSourcingError;
use crate::{Event, EventReplayConfig, EventStore, State};
use std::sync::Arc;
use tokio::sync::RwLock;
use std::default::Default;

pub struct EventReplay {
    config: EventReplayConfig,
}

impl EventReplay {
    pub fn new(config: EventReplayConfig) -> Result<Self, EventSourcingError> {
        Ok(Self { config })
    }

    //TODO: Implement replay functionality
    pub async fn replay(&self, event_store: Arc<RwLock<EventStore>>, target_version: Option<u64>) -> Result<State, EventSourcingError> {
        // TODO: Implement replay functionality
        todo!("Implement replay functionality")
    }

    fn apply_event(&self, state: &mut State, event: &Event) -> Result<(), EventSourcingError> {
        // TODO: Implement logic to apply different event types to the state
        match event.event_type.as_str() {
            "RuleAdded" => {
                // TODO: Apply rule added logic
            }
            "RuleRemoved" => {
                // TODO: Apply rule removed logic
            }
            "ConfigurationChanged" => {
                // TODO: Apply configuration change logic
            }
            // TODO: Add more event types as needed
            _ => return Err(EventSourcingError::UnknownEventType(event.event_type.clone())),
        }

        Ok(())
    }
} 
