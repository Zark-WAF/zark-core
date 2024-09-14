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

use std::{collections::HashMap, sync::Arc};
use zark_waf_common::messenger::Messenger;
mod error;
mod supervisor;
mod loader;
mod module;

pub use error::ModuleManagerError;
pub use supervisor::ModuleSupervisor;
pub use loader::ModuleLoader;
pub use module::{Module, ModuleInfo};

pub struct ModuleManager {
    modules: HashMap<String, Box<dyn Module>>,
    loader: ModuleLoader,
    messenger: Arc<Messenger>,
}

impl ModuleManager {
    pub fn new() -> Self {
        Self {
            modules: HashMap::new(),
            loader: ModuleLoader::new(),
            messenger: Arc::new(Messenger::new("module_manager"))
        }
    }

    pub async fn load_module(&mut self, path: &str) -> Result<(), ModuleManagerError> {
        let module = self.loader.load(path)?;
        let name = module.name().to_string();

        self.modules.insert(name.clone(), module);
        
        // Notify about module loading
        self.messenger.send("module_manager", format!("Module '{}' loaded", name).as_bytes()).await
            .map_err(|e| ModuleManagerError::ExecutionError(e.to_string()))?;

        Ok(())
    }

    pub async fn unload_module(&mut self, name: &str) -> Result<(), ModuleManagerError> {
        if let Some(mut module) = self.modules.remove(name) {
            module.shutdown().await
                .map_err(|e| ModuleManagerError::ShutdownError(e.to_string()))?;

            // Notify about module unloading
            self.messenger.send("module_manager", format!("Module '{}' unloaded", name).as_bytes()).await
                .map_err(|e| ModuleManagerError::ExecutionError(e.to_string()))?;

            Ok(())
        } else {
            Err(ModuleManagerError::ModuleNotFound(name.to_string()))
        }
    }

    pub async fn start_all_modules(&mut self) -> Result<(), ModuleManagerError> {
        for (name, module) in self.modules.iter_mut() {
            module.execute(serde_json::json!({"action": "start"})).await
                .map_err(|e| ModuleManagerError::ExecutionError(e.to_string()))?;

            // Notify about module starting
            self.messenger.send("module_manager", format!("Module '{}' started", name).as_bytes()).await
                .map_err(|e| ModuleManagerError::ExecutionError(e.to_string()))?;
        }
        Ok(())
    }

    pub async fn stop_all_modules(&mut self) -> Result<(), ModuleManagerError> {
        for (name, module) in self.modules.iter_mut() {
            module.shutdown().await
                .map_err(|e| ModuleManagerError::ShutdownError(e.to_string()))?;

            // Notify about module stopping
            self.messenger.send("module_manager", format!("Module '{}' stopped", name).as_bytes()).await
                .map_err(|e| ModuleManagerError::ExecutionError(e.to_string()))?;
        }
        Ok(())
    }

    pub async fn get_module_info(&self, name: &str) -> Result<ModuleInfo, ModuleManagerError> {
        self.modules.get(name)
            .ok_or(ModuleManagerError::ModuleNotFound(name.to_string()))
            .map(|module| ModuleInfo {
                name: module.name().to_string(),
                version: module.version().to_string(),
                description: module.description().to_string(),
                status: module::ModuleStatus::Running, // Assuming the module is running if it's loaded
            })
    }

    pub async fn list_modules(&self) -> Vec<ModuleInfo> {
        self.modules.iter().map(|(_, module)| ModuleInfo {
            name: module.name().to_string(),
            version: module.version().to_string(),
            description: module.description().to_string(),
            status: module::ModuleStatus::Running, // Assuming all loaded modules are running
        }).collect()
    }
}