 
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
use std::sync::Arc;
use tokio::sync::RwLock;
use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use crate::config::Config;
use crate::error::ConfigError;

pub struct ConfigWatcher {
    config_path: String,
    config: Arc<RwLock<Config>>,
    watcher: Option<RecommendedWatcher>,
}

impl ConfigWatcher {
    pub fn new(config_path: String, config: Arc<RwLock<Config>>) -> Self {
        Self {
            config_path,
            config,
            watcher: None,
        }
    }

    pub async fn start(&mut self) -> Result<(), ConfigError> {
        let config = Arc::clone(&self.config);
        let config_path = self.config_path.clone();

        let mut watcher = notify::recommended_watcher(move |res: Result<notify::Event, notify::Error>| {
            if let Ok(event) = res {
                if event.kind.is_modify() {
                    let config_clone = Arc::clone(&config);
                    let path_clone = config_path.clone();
                    tokio::spawn(async move {
                        if let Ok(new_config) = Config::load(&path_clone).await {
                            let mut config = config_clone.write().await;
                            *config = new_config;
                            log::info!("Configuration updated");
                        } else {
                            log::error!("Failed to reload configuration");
                        }
                    });
                }
            }
        })?;

        watcher.watch(Path::new(&self.config_path), RecursiveMode::NonRecursive)?;
        self.watcher = Some(watcher);

        Ok(())
    }

    pub async fn stop(&mut self) -> Result<(), ConfigError> {
        if let Some(watcher) = self.watcher.take() {
            drop(watcher);
        }
        Ok(())
    }
}