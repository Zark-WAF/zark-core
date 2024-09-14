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

use std::sync::Arc;
use crate::utils::uid::Uid;
use tokio::sync::mpsc;

// type alias for a topic, which is just a string
pub type Topic = String;

// type alias for a message, which is a vector of bytes
pub type Message = Vec<u8>;

// type alias for a subscriber id, which is a Uid
pub type SubscriberId = Uid;

// type alias for a callback function, which takes a Message and returns nothing
pub type Callback = Arc<dyn Fn(Message) + Send + Sync>;

// struct representing a subscriber
#[derive(Clone)]
pub struct Subscriber {
    // unique identifier for the subscriber
    pub id: SubscriberId,
    // topic the subscriber is interested in
    pub topic: Topic,
    // channel sender for sending messages to this subscriber
    pub sender: mpsc::Sender<Message>,
}

// struct representing a subscription
#[derive(Clone)]
pub struct Subscription {
    // the subscriber
    pub subscriber: Subscriber,
    // the callback function to be called when a message is received
    pub callback: Callback,
}

// implement Clone for SubscriberId (which is just an alias for Uid)
impl Clone for SubscriberId {
    fn clone(&self) -> Self {
        // create a new SubscriberId from the string representation of the current one
        SubscriberId::from_string(&self.to_string()).unwrap()
    }
}

// implement PartialEq for SubscriberId to allow comparison
impl PartialEq for SubscriberId {
    fn eq(&self, other: &Self) -> bool {
        self.to_string() == other.to_string()
    }
}

// implement Eq for SubscriberId to allow use in hash maps
impl Eq for SubscriberId {}