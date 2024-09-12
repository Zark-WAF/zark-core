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


mod error;
mod plugin;
mod manager;
mod loader;

pub use error::PluginError;
pub use plugin::{Plugin, PluginMetadata};
pub use manager::PluginManager;
pub use loader::PluginLoader;

use zark_waf_common::messaging::messenger::ZarkMessenger;
use std::sync::Arc;

pub struct PluginSystem {
    manager: PluginManager,
    loader: PluginLoader,
}

impl PluginSystem {
    pub fn new(messenger: Arc<ZarkMessenger>) -> Self {
        Self {
            manager: PluginManager::new(messenger),
            loader: PluginLoader::new(),
        }
    }

    pub async fn load_plugin(&self, path: &str) -> Result<(), PluginError> {
        let plugin = self.loader.load(path).await?;
        self.manager.add_plugin(plugin).await?;
        Ok(())
    }

    pub async fn unload_plugin(&self, name: &str) -> Result<(), PluginError> {
        self.manager.remove_plugin(name).await?;
        Ok(())
    }

    pub async fn execute_plugin(&self, name: &str, input: serde_json::Value) -> Result<serde_json::Value, PluginError> {
        self.manager.execute_plugin(name, input).await
    }

    pub async fn get_plugin_metadata(&self, name: &str) -> Result<PluginMetadata, PluginError> {
        self.manager.get_plugin_metadata(name).await
    }

    pub fn list_plugins(&self) -> Vec<PluginMetadata> {
        self.manager.list_plugins()
    }
}