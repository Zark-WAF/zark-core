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

use serde::Deserialize;
use std::fs::File;
use std::io::Read;
use crate::core::error::CoreError;
use zark_waf_logger::logger::LoggerConfig;


#[derive(Deserialize, Clone)]
pub struct Config {
    pub general: GeneralConfig,
    pub logger: LoggerConfig,
    pub web_servers: WebServersConfig,
    pub modules: ModulesConfig,
    pub plugins: PluginsConfig,
}

#[derive(Deserialize, Clone)]
pub struct GeneralConfig {
    pub thread_pool_size: usize,
    pub max_connections: usize,
}




#[derive(Deserialize, Clone)]
pub struct WebServersConfig {
    pub nginx: WebServerConfig,
    pub apache: WebServerConfig,
    pub haproxy: WebServerConfig,
    pub iis: WebServerConfig,
}

#[derive(Deserialize, Clone)]
pub struct WebServerConfig {
    pub enabled: bool,
    pub config_path: String,
}

#[derive(Deserialize, Clone)]
pub struct ModulesConfig {
    pub load_paths: Vec<String>,
}

#[derive(Deserialize, Clone)]
pub struct PluginsConfig {
    pub load_paths: Vec<String>,
}

impl Config {
    pub fn load(path: &str) -> Result<Self, CoreError> {
        let mut file = File::open(path).map_err(|e| CoreError::ConfigError(format!("Failed to open config file: {}", e)))?;
        let mut contents = String::new();
        file.read_to_string(&mut contents).map_err(|e| CoreError::ConfigError(format!("Failed to read config file: {}", e)))?;
        serde_json::from_str(&contents).map_err(|e| CoreError::ConfigError(format!("Failed to parse config: {}", e)))
    }
}