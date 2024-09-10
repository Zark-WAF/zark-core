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

use async_trait::async_trait;
use crate::core::error::ZarkWafError;


//this is the interface that all modules must implement
// it is used to define the contract between the core and the modules
// the modules are responsible for registering themselves to the core
// and for implementing the methods defined in this trait 
// the core is responsible for creating the modules and for calling the methods defined in this trait
// the modules are responsible for processing the messages and for updating the state
// the modules are responsible for sending the messages to the core
// the core is responsible for sending the messages to the modules
// the core is responsible for stopping the modules
// the core is responsible for restarting the modules
// the core is responsible for updating the modules
// the core is responsible for getting the state of the modules
// the core is responsible for setting the state of the modules
// the core is responsible for getting the messages from the modules
// the core is responsible for sending the messages to the modules
// the core is responsible for stopping the modules
// the core is responsible for restarting the modules
#[async_trait]
pub trait ZarkModuleInterface: Send + Sync {
    fn name(&self) -> &'static str;
    async fn init(&mut self) -> Result<(), ZarkWafError>;
    async fn process_message(&self, message: &[u8]) -> Result<(), ZarkWafError>;
    async fn shutdown(&mut self) -> Result<(), ZarkWafError>;
}
