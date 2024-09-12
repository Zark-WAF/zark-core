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

pub mod config;
pub mod watcher;
pub mod updater;
pub mod error;

use std::sync::Arc;
use tokio::sync::RwLock;
use crate::config::Config;
use crate::watcher::ConfigWatcher;
use crate::updater::ConfigUpdater;
use crate::error::ConfigError;

pub struct ConfigManager {
    config: Arc<RwLock<Config>>,
    watcher: ConfigWatcher,
    // TODO: Implement usage of updater
    updater: ConfigUpdater,
}

impl ConfigManager {
    pub async fn new(config_path: &str) -> Result<Self, ConfigError> {
        let config = Config::load(config_path).await?;
        let config = Arc::new(RwLock::new(config));
        let watcher = ConfigWatcher::new(config_path.to_string(), Arc::clone(&config));
        let updater = ConfigUpdater::new(Arc::clone(&config));

        Ok(Self {
            config,
            watcher,
            updater,
        })
    }

    pub async fn start(&mut self) -> Result<(), ConfigError> {
        self.watcher.start().await?;
        Ok(())
    }

    pub async fn stop(&mut self) -> Result<(), ConfigError> {
        self.watcher.stop().await?;
        Ok(())
    }

    pub async fn get_config(&self) -> Config {
        self.config.read().await.clone()
    }
}