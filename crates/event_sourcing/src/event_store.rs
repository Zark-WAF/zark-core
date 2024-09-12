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
use crate::{Event, EventStoreConfig};
use std::collections::VecDeque;

pub struct EventStore {
    events: VecDeque<Event>,
    config: EventStoreConfig,
}

impl EventStore {
    pub fn new(config: EventStoreConfig) -> Result<Self, EventSourcingError> {
        Ok(Self {
            events: VecDeque::new(),
            config,
        })
    }

    pub async fn append(&mut self, event: Event) -> Result<(), EventSourcingError> {
        self.events.push_back(event);
        // TODO: Implement persistence, am thinking of using a file storage lz4 compressed. i need to check how fast the file storage is compared to the db.
        // TODO: Implement compression
        Ok(())
    }

    pub async fn get_events(&self, start: u64, end: Option<u64>) -> Result<Vec<Event>, EventSourcingError> {
        let end = end.unwrap_or(self.events.len() as u64);
        Ok(self.events.range(start as usize..end as usize).cloned().collect())
    }

    pub async fn get_latest_event(&self) -> Option<Event> {
        self.events.back().cloned()
    }

    pub async fn get_event_count(&self) -> u64 {
        self.events.len() as u64
    }
}
