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
use crate::error::ModuleManagerError;
use crate::module::Module;

pub struct ModuleLoader;

type ModuleCreateFn = unsafe fn() -> *mut dyn Module;

impl ModuleLoader {
    pub fn new() -> Self {
        ModuleLoader
    }

    pub async fn load<P: AsRef<Path>>(&self, path: P) -> Result<Box<dyn Module>, ModuleManagerError> {
        let path = path.as_ref().to_path_buf();
        
        // Load the dynamic library
        let lib = unsafe {
            Library::new(path.clone()).map_err(|e| ModuleManagerError::LoadError(e.to_string()))?
        };

        // Look up the `create_module` symbol
        let constructor: Symbol<ModuleCreateFn> = unsafe {
            lib.get(b"create_module")
                .map_err(|e| ModuleManagerError::LoadError(format!("Failed to find 'create_module' symbol: {}", e)))?
        };

        // Call the constructor
        let module = unsafe {
            Box::from_raw(constructor())
        };

        Ok(module)
    }
}