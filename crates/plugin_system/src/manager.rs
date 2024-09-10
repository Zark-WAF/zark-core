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
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::sync::RwLock;
use libloading::{Library, Symbol};
use crate::error::ZarkPluginError;
use crate::plugin::{ZarkPlugin, PluginCreate};


pub struct ZarkPluginManager {
    plugins: RwLock<HashMap<String, Box<dyn ZarkPlugin>>>,
    libraries: RwLock<HashMap<String, Arc<Library>>>,
    broker: Arc<ZarkMessenger>,
}

impl ZarkPluginManager {
    pub fn new(broker: Arc<ZarkMessenger>) -> Self {
        ZarkPluginManager {
            plugins: RwLock::new(HashMap::new()),
            libraries: RwLock::new(HashMap::new()),
            broker,
        }
    }

    pub async fn load_plugin<P: AsRef<Path>>(&self, path: P) -> Result<(), ZarkPluginError> {
        let path = path.as_ref().to_path_buf();
        let library = unsafe { Library::new(&path)? };
        let library = Arc::new(library);

        let create_plugin: Symbol<PluginCreate> = unsafe {
            library.get(b"_create_plugin")?
        };

        let plugin = unsafe {
            Box::from_raw(create_plugin())
        };

        let name = plugin.name().to_string();
        plugin.init(self.broker.clone()).await?;

        let mut plugins = self.plugins.write().await;
        let mut libraries = self.libraries.write().await;

        plugins.insert(name.clone(), plugin);
        libraries.insert(name, library);

        Ok(())
    }

    pub async fn unload_plugin(&self, name: &str) -> Result<(), ZarkPluginError> {
        let mut plugins = self.plugins.write().await;
        let mut libraries = self.libraries.write().await;

        if let Some(mut plugin) = plugins.remove(name) {
            plugin.shutdown().await?;
        }
        libraries.remove(name);

        Ok(())
    }

    pub async fn execute_plugin(&self, name: &str, input: serde_json::Value) -> Result<serde_json::Value, ZarkPluginError> {
        let plugins = self.plugins.read().await;
        if let Some(plugin) = plugins.get(name) {
            Ok(plugin.execute(input).await?)
        } else {
            Err(ZarkPluginError::PluginNotFound(name.to_string()))
        }
    }

    

}
