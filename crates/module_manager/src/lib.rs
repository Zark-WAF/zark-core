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
use std::{collections::HashMap, ffi::c_void, sync::Arc};

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
    messenger: *mut c_void,
    loader: ModuleLoader,
}





impl ModuleManager {
    pub fn new(messenger: *mut c_void) -> Self {
        Self {
            modules: HashMap::new(),
            messenger,
            loader: ModuleLoader::new(),
        }
    }

    

    pub async fn load_module(&mut self, path: &str) -> Result<(), ModuleManagerError> {
        let mut module = self.loader.load(path)?;
        let name = module.name().to_string();

        module.init(self.messenger).await
            .map_err(|e| ModuleManagerError::InitializationError(e.to_string()))?;

        self.modules.insert(name, module);
        Ok(())
    }


    pub async fn unload_module(&self, name: &str) -> Result<(), ModuleManagerError> {
        let _ = name;
        Ok(())
    }

    pub async fn start_all_modules(&self) -> Result<(), ModuleManagerError> {
        Ok(())
    }

    pub async fn stop_all_modules(&self) -> Result<(), ModuleManagerError> {
        Ok(())
    }
    pub async fn get_module_info(&self, name: &str) -> Result<ModuleInfo, ModuleManagerError> {
        self.modules.get(name)
            .ok_or(ModuleManagerError::ModuleNotFound(name.to_string()))
            .map(|module| ModuleInfo {
                name: module.name().to_string(),
                version: module.version().to_string(),
                description: module.description().to_string(),
                status: todo!(),
            })
    }

    pub async fn list_modules(&self) -> Vec<ModuleInfo> {
        let mut module_info_list = Vec::new();
        for (name, module) in &self.modules {
            if let Ok(info) = self.get_module_info(name).await {
                module_info_list.push(info);
            }
        }
        module_info_list
    }
}