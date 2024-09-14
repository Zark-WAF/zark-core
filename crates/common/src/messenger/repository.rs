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

use super::domain::{Topic, SubscriberId, Subscription};
use std::collections::HashMap;
use tokio::sync::RwLock;

pub struct SubscriptionRepository {
    subscriptions: RwLock<HashMap<Topic, Vec<Subscription>>>,
}

impl SubscriptionRepository {
    pub fn new() -> Self {
        Self {
            subscriptions: RwLock::new(HashMap::new()),
        }
    }

    pub async fn add(&self, subscription: Subscription) {
        let mut subs = self.subscriptions.write().await;
        subs.entry(subscription.subscriber.topic.clone())
            .or_insert_with(Vec::new)
            .push(subscription);
    }

    pub async fn remove(&self, topic: &Topic, subscriber_id: &SubscriberId) {
        let mut subs = self.subscriptions.write().await;
        if let Some(topic_subs) = subs.get_mut(topic) {
            topic_subs.retain(|sub| sub.subscriber.id != *subscriber_id);
        }
    }

    pub async fn get_subscriptions(&self, topic: &Topic) -> Vec<Subscription> {
        let subs = self.subscriptions.read().await;
        subs.get(topic)
            .map(|subscriptions| subscriptions.iter().cloned().collect())
            .unwrap_or_default()
    }
}