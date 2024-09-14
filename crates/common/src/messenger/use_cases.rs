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


use super::domain::{Topic, Message, Subscriber, Callback, Subscription};
use super::repository::SubscriptionRepository;
use std::sync::Arc;
use tokio::sync::mpsc;
use crate::utils::uid::Uid;

pub struct MessengerUseCases {
    repository: Arc<SubscriptionRepository>,
}

impl MessengerUseCases {
    pub fn new(repository: Arc<SubscriptionRepository>) -> Self {
        Self { repository }
    }

    pub async fn publish(&self, topic: Topic, message: Message) {
        let subscriptions = self.repository.get_subscriptions(&topic).await;
        for subscription in subscriptions {
            let message = message.clone();
            if let Some(callback) = subscription.callback {
                callback(message.clone());
            }
            let _ = subscription.subscriber.sender.send(message).await;
        }
    }

    pub async fn subscribe(&self, topic: Topic, callback: Option<Callback>) -> Subscriber {
        let (sender, _receiver) = mpsc::channel(100);
        let subscriber = Subscriber {
            id: Uid::new(), // Use the new Uid::new() method
            topic: topic.clone(),
            sender,
        };
        let subscription = Subscription { subscriber: subscriber.clone(), callback };
        self.repository.add(subscription).await;
        subscriber
    }

    pub async fn unsubscribe(&self, topic: Topic, subscriber_id: Uuid) {
        self.repository.remove(&topic, &subscriber_id).await;
    }
}