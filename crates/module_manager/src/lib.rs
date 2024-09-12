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

mod error;
mod supervisor;
mod loader;
mod module;

pub use error::ModuleManagerError;
pub use supervisor::ModuleSupervisor;
pub use loader::ModuleLoader;
pub use module::{Module, ModuleInfo};

use zark_waf_common::messaging::messenger::ZarkMessenger;

pub struct ModuleManager {
    supervisor: Arc<ModuleSupervisor>,
    loader: ModuleLoader,
}

impl ModuleManager {
    pub fn new(messenger: Arc<ZarkMessenger>) -> Self {
        Self {
            supervisor: Arc::new(ModuleSupervisor::new(messenger)),
            loader: ModuleLoader::new(),
        }
    }

    pub async fn load_module(&self, path: &str) -> Result<(), ModuleManagerError> {
        let module = self.loader.load(path).await?;
        self.supervisor.add_module(module).await?;
        Ok(())
    }

    pub async fn unload_module(&self, name: &str) -> Result<(), ModuleManagerError> {
        self.supervisor.remove_module(name).await?;
        Ok(())
    }

    pub async fn start_all_modules(&self) -> Result<(), ModuleManagerError> {
        self.supervisor.start_all().await?;
        Ok(())
    }

    pub async fn stop_all_modules(&self) -> Result<(), ModuleManagerError> {
        self.supervisor.stop_all().await?;
        Ok(())
    }

    pub async fn get_module_info(&self, name: &str) -> Result<ModuleInfo, ModuleManagerError> {
        self.supervisor.get_module_info(name).await
    }

    pub async fn list_modules(&self) -> Vec<ModuleInfo> {
        self.supervisor.list_modules().await
    }
}