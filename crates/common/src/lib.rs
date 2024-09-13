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

pub mod utils;

use std::ffi::c_void;

use async_trait::async_trait;

pub mod messaging {
    

    /// A trait for messaging systems.
    pub trait Messenger: Send + Sync {
    
        /// Send a message to a topic.
        fn send(&self, topic: &str, message: &[u8]) -> Result<(), Box<dyn std::error::Error>>;
        /// Subscribe to a topic and receive messages.
        fn subscribe(&self, topic: &str) -> Box<dyn MessageReceiver>;
    }

    /// A trait for message receivers.
    pub trait MessageReceiver: Send + Sync {
        /// Receive a message from the subscribed topic.
        fn receive(&self) -> Result<Vec<u8>, Box<dyn std::error::Error>>;
    }
}

#[async_trait]
pub trait Module: Send + Sync {
    fn name(&self) -> &str;
    fn version(&self) -> &str;
    fn description(&self) -> &str;
    async fn init(&mut self, messenger: *mut c_void) -> Result<(), Box<dyn std::error::Error>>;
    async fn execute(&self, input: serde_json::Value) -> Result<serde_json::Value, Box<dyn std::error::Error>>;
    async fn shutdown(&mut self) -> Result<(), Box<dyn std::error::Error>>;
}

