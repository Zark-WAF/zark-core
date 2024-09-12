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

use std::path::Path;
use libloading::{Library, Symbol};
use crate::error::PluginError;
use crate::plugin::{Plugin, PluginCreate};

pub struct PluginLoader;

impl PluginLoader {
    pub fn new() -> Self {
        PluginLoader
    }

    pub async fn load<P: AsRef<Path>>(&self, path: P) -> Result<Box<dyn Plugin>, PluginError> {
        let path = path.as_ref().to_path_buf();
        
        // Load the dynamic library
        let lib = unsafe {
            Library::new(path.clone()).map_err(|e| PluginError::LoadError(e.to_string()))?
        };

        // Look up the `create_plugin` symbol
        let constructor: Symbol<PluginCreate> = unsafe {
            lib.get(b"create_plugin")
                .map_err(|e| PluginError::LoadError(format!("Failed to find 'create_plugin' symbol: {}", e)))?
        };

        // Call the constructor
        let plugin = unsafe {
            Box::from_raw(constructor())
        };

        Ok(plugin)
    }
}