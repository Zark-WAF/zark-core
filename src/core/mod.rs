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

mod config;
mod error;
mod state;

use std::sync::Arc;
use tokio::sync::RwLock;
use zark_waf_common::messaging::messenger::ZarkMessenger;
use zark_waf_module_manager::ModuleManager;
use zark_waf_plugin_system::PluginSystem;
use zark_waf_logger::ZarkLogger;
use crate::core::config::Config;
use crate::core::error::CoreError;
use crate::core::state::CoreState;
use zark_waf_common::Module;


pub struct ZarkWafCore {
    config: Config,
    state: Arc<RwLock<CoreState>>,
    messenger: Arc<ZarkMessenger>,
    module_manager: ModuleManager,
    plugin_system: PluginSystem,
    logger: ZarkLogger,
}

impl ZarkWafCore {
    pub async fn new(config_path: &str) -> Result<Self, CoreError> {
        let config = Config::load(config_path)?;
        let messenger = Arc::new(ZarkMessenger::new());
        let module_manager = ModuleManager::new(messenger.clone());
        let plugin_system = PluginSystem::new(messenger.clone());
        
        let logger = ZarkLogger::new(config.logger.clone());

        let mut core = Self {
            config: config.clone(),
            state: Arc::new(RwLock::new(CoreState::new())),
            messenger: messenger.clone(),
            module_manager,
            plugin_system,
            logger,
        };

        core.init().await?;

        Ok(core)
    }

    async fn init(&mut self) -> Result<(), CoreError> {
        self.logger.init(self.messenger.clone()).await.map_err(|e| CoreError::InitError(e.to_string()))?;
        
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
                // TODO: Add other sources of events
            }
        }

        // Perform cleanup
        self.shutdown().await
    }

    async fn shutdown(&self) -> Result<(), CoreError> {
        log::info!("Shutting down ZARK-WAF core");
        // Shutdown logic: stop modules, unload plugins, etc.
        Ok(())
    }
}