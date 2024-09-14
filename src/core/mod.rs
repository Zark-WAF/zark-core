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
mod state;

use crate::core::error::CoreError;
use crate::core::state::CoreState;
use std::{ffi::c_void, sync::Arc};
use tokio::sync::RwLock;
use zark_waf_config_manager::config::Config;
use zark_waf_module_manager::{Module, ModuleManager};
use zark_waf_plugin_system::PluginSystem;
use zark_waf_common::messenger::Messenger;

pub struct ZarkWafCore {
    config: Config,
    state: Arc<RwLock<CoreState>>,
    module_manager: ModuleManager,
    plugin_system: PluginSystem,
    messenger: Arc<Messenger>,
}

impl ZarkWafCore {
    pub async fn new(config_path: &str) -> Result<Self, CoreError> {
        let config = Config::load(config_path).await.map_err(CoreError::ConfigError)?;

        // Create ZarkMessenger instance using the common crate messenger module
        let messenger = Arc::new(Messenger::new(""));
        
        let module_manager = ModuleManager::new();
        let plugin_system = PluginSystem::new();

        let core = Self {
            config,
            state: Arc::new(RwLock::new(CoreState::new())),
            module_manager,
            plugin_system,
            messenger,
        };

        core.init()?;

        Ok(core)
    }

    fn init(&mut self) -> Result<(), CoreError> {
        // Load modules
        for (name, path) in &self.config.modules {
            self.module_manager.load_module(path).map_err(|e| {
                CoreError::InitError(format!("Failed to load module {}: {}", name, e))
            })?;
        }

        Ok(())
    }

    pub async fn run(&self) -> Result<(), CoreError> {
        log::info!("ZARK-WAF core is running");

        // Main loop
        loop {
            tokio::select! {
                _ = tokio::signal::ctrl_c() => {
                    log::info!("Shutdown signal received");
                    break;
                }
                // Add other async tasks here if needed
            }
        }

        // Perform cleanup
        self.shutdown().await
    }

    async fn shutdown(&self) -> Result<(), CoreError> {
        log::info!("Shutting down ZARK-WAF core");

        // Unload all modules
        for (module_name, _) in &self.config.modules {
            self.module_manager
                .unload_module(module_name)
                .await
                .map_err(|e| {
                    CoreError::ShutdownError(format!(
                        "Failed to unload module {}: {}",
                        module_name, e
                    ))
                })?;
        }

        // Unload logger module separately
        self.module_manager
            .unload_module("logger")
            .await
            .map_err(|e| CoreError::ShutdownError(format!("Failed to unload logger: {}", e)))?;

        // The messenger will be automatically dropped when the Arc reference count reaches zero

        Ok(())
    }
}
