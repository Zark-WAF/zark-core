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
use async_trait::async_trait;
use tokio::sync::broadcast;
use zark_waf_common::messaging::{Messenger, MessageReceiver, MessengerError};
use zark_waf_common::Module;

pub struct ZarkMessenger {
    channels: Arc<tokio::sync::RwLock<std::collections::HashMap<String, broadcast::Sender<Vec<u8>>>>>,
}

impl ZarkMessenger {
    pub fn new() -> Self {
        ZarkMessenger {
            channels: Arc::new(tokio::sync::RwLock::new(std::collections::HashMap::new())),
        }
    }
}

#[async_trait]
impl Messenger for ZarkMessenger {
    async fn send(&self, topic: &str, message: &[u8]) -> Result<(), MessengerError> {
        let channels = self.channels.read().await;
        if let Some(sender) = channels.get(topic) {
            sender.send(message.to_vec()).map_err(|e| MessengerError::SendError(e.to_string()))?;
        }
        Ok(())
    }

    async fn subscribe(&self, topic: &str) -> Result<MessageReceiver, MessengerError> {
        let mut channels = self.channels.write().await;
        let sender = channels.entry(topic.to_string())
            .or_insert_with(|| broadcast::channel(100).0);
        let receiver = sender.subscribe();
        Ok(MessageReceiver { /* initialize with receiver */ })
    }
}

#[async_trait]
impl Module for ZarkMessenger {

    fn as_ptr(&self) -> *const c_void {
        self as *const _ as *const c_void
    }

    fn name(&self) -> &str {
        "zark_messenger"
    }

    async fn init(&mut self, _broker: Arc<dyn Messenger>) -> Result<(), Box<dyn std::error::Error>> {
        // Initialization logic
        Ok(())
    }

    async fn execute(&self, _input: serde_json::Value) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        // Execution logic
        Ok(serde_json::Value::Null)
    }

    async fn shutdown(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Shutdown logic
        Ok(())
    }
}


//this is the entry point for the the messenger module
//it is used by the module manager to create the module
// it returns a boxed pointer to the module
#[no_mangle]
pub extern "C" fn create_messenger() -> *mut ZarkMessenger {
    let messenger = ZarkMessenger::new();
    let boxed = Box::new(messenger);
    Box::into_raw(boxed)
}

//this is the destructor for the messenger module
//it is used by the module manager to destroy the module
//when the module is no longer needed
#[no_mangle]
pub extern "C" fn delete_messenger(ptr: *mut ZarkMessenger) {
    if !ptr.is_null() {
        unsafe {
            Box::from_raw(ptr);
        }
    }
}