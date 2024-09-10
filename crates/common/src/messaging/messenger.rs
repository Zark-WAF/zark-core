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
// 
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
// 
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
//
// Authors: I. Zeqiri, E. Gjergji

use crossbeam_channel::{unbounded, Sender, Receiver};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use serde::{Serialize, Deserialize};


pub struct ZarkMessenger {
    subscribers: Arc<Mutex<HashMap<String, Vec<Sender<Vec<u8>>>>>>,
}

impl ZarkMessenger {
    pub fn new() -> Self {
        ZarkMessenger {
            subscribers: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Subscribe to a topic
    pub fn subscribe(&self, topic: &str) -> Receiver<Vec<u8>> {
        let (tx, rx) = unbounded();
        let mut subs = self.subscribers.lock().unwrap();
        subs.entry(topic.to_string())
            .or_insert_with(Vec::new)
            .push(tx);
        rx
    }

    /// Publish a message to a topic
    pub fn publish(&self, topic: &str, message: Vec<u8>) {
        let subs = self.subscribers.lock().unwrap();
        if let Some(topic_subs) = subs.get(topic) {
            for sub in topic_subs {
                let _ = sub.send(message.clone());
            }
        }
    }
}
