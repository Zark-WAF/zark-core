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

use std::ffi::c_void;
use std::sync::Arc;
use tokio::sync::RwLock;
use dashmap::DashMap;
use crate::error::PluginError;
use crate::plugin::{Plugin, PluginMetadata, PluginStatus};

pub struct PluginManager {
    plugins: DashMap<String, Arc<RwLock<Box<dyn Plugin>>>>,
    messenger: *mut c_void,
}

impl PluginManager {
    pub fn new(messenger: &mut c_void) -> Self {
        Self {
            plugins: DashMap::new(),
            messenger: &messenger,
        }
    }

    pub async fn add_plugin(&self, mut plugin: Box<dyn Plugin>) -> Result<(), PluginError> {
        let name = plugin.name().to_string();
        plugin.init(&self.messenger).await.map_err(|e| PluginError::InitializationError(e.to_string()))?;
        self.plugins.insert(name.clone(), Arc::new(RwLock::new(plugin)));
        Ok(())
    }

    pub async fn remove_plugin(&self, name: &str) -> Result<(), PluginError> {
        if let Some((_, plugin)) = self.plugins.remove(name) {
            let mut plugin = plugin.write().await;
            plugin.shutdown().await.map_err(|e| PluginError::ShutdownError(e.to_string()))?;
        } else {
            return Err(PluginError::PluginNotFound(name.to_string()));
        }
        Ok(())
    }

    pub async fn execute_plugin(&self, name: &str, input: serde_json::Value) -> Result<serde_json::Value, PluginError> {
        if let Some(plugin) = self.plugins.get(name) {
            let plugin = plugin.read().await;
            plugin.execute(input).await.map_err(|e| PluginError::ExecutionError(e.to_string()))
        } else {
            Err(PluginError::PluginNotFound(name.to_string()))
        }
    }

    pub async fn get_plugin_metadata(&self, name: &str) -> Result<PluginMetadata, PluginError> {
        if let Some(plugin) = self.plugins.get(name) {
            let plugin = plugin.read().await;
            Ok(PluginMetadata {
                name: plugin.name().to_string(),
                version: plugin.version().to_string(),
                description: plugin.description().to_string(),
                status: PluginStatus::Running, 
            })
        } else {
            Err(PluginError::PluginNotFound(name.to_string()))
        }
    }

    pub fn list_plugins(&self) -> Vec<PluginMetadata> {
        self.plugins.iter().map(|entry| {
            let plugin = entry.value().blocking_read();
            PluginMetadata {
                name: plugin.name().to_string(),
                version: plugin.version().to_string(),
                description: plugin.description().to_string(),
                status: PluginStatus::Running, 
            }
        }).collect()
    }
}