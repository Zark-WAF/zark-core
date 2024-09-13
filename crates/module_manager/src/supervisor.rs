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

use tokio::sync::RwLock;
use dashmap::DashMap;
use std::sync::Arc;

use futures::future::join_all;
use zark_waf_common::messaging::Messenger;
use crate::error::ModuleManagerError;
use crate::module::{Module, ModuleInfo, ModuleStatus};

pub struct ModuleSupervisor {
    modules: DashMap<String, Arc<RwLock<Box<dyn Module>>>>,
    messenger: Option<Arc<dyn Messenger>>,
}

impl ModuleSupervisor {
    pub fn new() -> Self {
        Self {
            modules: DashMap::new(),
            messenger: None,
        }
    }

    pub fn set_messenger(&mut self, messenger: Arc<dyn Messenger>) {
        self.messenger = Some(messenger);
    }

    pub fn get_messenger(&self) -> Option<Arc<dyn Messenger>> {
        self.messenger.clone()
    }

    pub async fn add_module(&self, name: String, module: Arc<RwLock<Box<dyn Module>>>) -> Result<(), ModuleManagerError> {
        self.modules.insert(name, module);
        Ok(())
    }

    pub async fn remove_module(&self, name: &str) -> Result<(), ModuleManagerError> {
        if let Some((_, module)) = self.modules.remove(name) {
            let mut module = module.write().await;
            module.shutdown().await.map_err(|e| ModuleManagerError::ShutdownError(e.to_string()))?;
        } else {
            return Err(ModuleManagerError::ModuleNotFound(name.to_string()));
        }
        Ok(())
    }

    pub async fn start_all(&self) -> Result<(), ModuleManagerError> {
        let futures: Vec<_> = self.modules.iter().map(|entry| {
            let module = entry.value().clone();
            async move {
                let module = module.read().await;
                module.execute(serde_json::json!({"action": "start"})).await
            }
        }).collect();

        join_all(futures).await.into_iter().collect::<Result<Vec<_>, _>>()
            .map_err(|e| ModuleManagerError::ExecutionError(e.to_string()))?;
        Ok(())
    }

    pub async fn stop_all(&self) -> Result<(), ModuleManagerError> {
        for module in self.modules.iter() {
            let mut module = module.value().write().await;
            module.shutdown().await
                .map_err(|e| ModuleManagerError::ShutdownError(e.to_string()))?;
        }
        Ok(())
    }

    pub async fn get_module_info(&self, name: &str) -> Result<ModuleInfo, ModuleManagerError> {
        if let Some(module) = self.modules.get(name) {
            let module = module.read().await;
            Ok(ModuleInfo {
                name: module.name().to_string(),
                version: module.version().to_string(),
                description: module.description().to_string(),
                status: ModuleStatus::Running,
            })
        } else {
            Err(ModuleManagerError::ModuleNotFound(name.to_string()))
        }
    }

    pub async fn list_modules(&self) -> Vec<ModuleInfo> {
        self.modules.iter().map(|entry| {
            let module = entry.value().blocking_read();
            ModuleInfo {
                name: module.name().to_string(),
                version: module.version().to_string(),
                description: module.description().to_string(),
                status: ModuleStatus::Running,
            }
        }).collect()
    }

}