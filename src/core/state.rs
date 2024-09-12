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

use std::sync::atomic::{AtomicUsize, AtomicBool, Ordering};
use std::collections::HashMap;


pub struct CoreState {
    pub is_running: AtomicBool,
    pub active_connections: AtomicUsize,
    pub module_states: HashMap<String, ModuleState>,
    pub plugin_states: HashMap<String, PluginState>,
}

pub struct ModuleState {
    pub is_active: bool,
    pub last_execution: std::time::Instant,
    // Add more module-specific state as needed
}

pub struct PluginState {
    pub is_loaded: bool,
    pub last_execution: std::time::Instant,
    // Add more plugin-specific state as needed
}

impl CoreState {
    pub fn new() -> Self {
        Self {
            is_running: AtomicBool::new(true),
            active_connections: AtomicUsize::new(0),
            module_states: HashMap::new(),
            plugin_states: HashMap::new(),
        }
    }

    pub fn is_running(&self) -> bool {
        self.is_running.load(Ordering::Relaxed)
    }

    pub fn stop(&self) {
        self.is_running.store(false, Ordering::Relaxed);
    }

    pub fn increment_connections(&self) -> usize {
        self.active_connections.fetch_add(1, Ordering::Relaxed)
    }

    pub fn decrement_connections(&self) -> usize {
        self.active_connections.fetch_sub(1, Ordering::Relaxed)
    }

    pub fn update_module_state(&mut self, name: String, state: ModuleState) {
        self.module_states.insert(name, state);
    }

    pub fn update_plugin_state(&mut self, name: String, state: PluginState) {
        self.plugin_states.insert(name, state);
    }
}