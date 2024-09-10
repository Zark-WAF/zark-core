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

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use zark_waf_common::{Module, Broker};
use crate::error::ModuleError;

pub struct ZarkModuleSupervisor {
    modules: RwLock<HashMap<String, Box<dyn Module>>>,
    broker: Arc<Broker>,
}

impl ZarkModuleSupervisor {
    pub fn new(broker: Arc<Broker>) -> Self {
        ZarkModuleSupervisor {
            modules: RwLock::new(HashMap::new()),
            broker,
        }
    }

    pub async fn add_module(&self, name: String, mut module: Box<dyn Module>) -> Result<(), ModuleError> {
        module.init(self.broker.clone()).await?;
        let mut modules = self.modules.write().await;
        modules.insert(name, module);
        Ok(())
    }

    pub async fn remove_module(&self, name: &str) -> Result<(), ModuleError> {
        let mut modules = self.modules.write().await;
        if let Some(mut module) = modules.remove(name) {
            module.shutdown().await?;
        }
        Ok(())
    }

    pub async fn start_all(&self) -> Result<(), ModuleError> {
        let modules = self.modules.read().await;
        for (_, module) in modules.iter() {
            module.start().await?;
        }
        Ok(())
    }

    pub async fn stop_all(&self) -> Result<(), ModuleError> {
        let modules = self.modules.read().await;
        for (_, module) in modules.iter() {
            module.stop().await?;
        }
        Ok(())
    }

    

}