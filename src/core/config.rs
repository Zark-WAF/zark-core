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
use std::io::BufReader;
use crate::core::error::ZarkWafError;

#[derive(Deserialize)]
pub struct Config {
    #[serde(rename = "zark-core")]
    pub zark_core: ZarkCore,
    #[serde(rename = "zark-logger")]
    pub zark_logger: ZarkLogger,
    #[serde(rename = "web-servers")]
    pub web_servers: WebServers,
    pub monitoring: Monitoring,
}

#[derive(Deserialize)]
pub struct ZarkCore {
    #[serde(rename = "thread-pool-size")]
    pub thread_pool_size: u32,
    #[serde(rename = "max-connections")]
    pub max_connections: u32,
}

#[derive(Deserialize)]
pub struct ZarkLogger {
    #[serde(rename = "log-type")]
    pub log_type: Vec<String>,
    #[serde(rename = "log-path")]
    pub log_path: String,
    #[serde(rename = "log-level")]
    pub log_level: String,
    #[serde(rename = "log-max-size")]
    pub log_max_size: u64,
    #[serde(rename = "log-max-backups")]
    pub log_max_backups: u32,
    #[serde(rename = "log-max-age")]
    pub log_max_age: u32,
    #[serde(rename = "log-compress")]
    pub log_compress: bool,
}

#[derive(Deserialize)]
pub struct WebServers {
    pub nginx: WebServerConfig,
    pub apache2: WebServerConfig,
    #[serde(rename = "ha-proxy")]
    pub ha_proxy: WebServerConfig,
    pub iis: WebServerConfig,
}

#[derive(Deserialize)]
pub struct WebServerConfig {
    pub enabled: bool,
    #[serde(rename = "config-path")]
    pub config_path: String,
    #[serde(rename = "pid-path")]
    pub pid_path: String,
    #[serde(rename = "log-path")]
    pub log_path: String,
    #[serde(rename = "error-log-path")]
    pub error_log_path: String,
    pub port: u16,
    pub host: String,
    #[serde(rename = "ssl-enabled")]
    pub ssl_enabled: bool,
    #[serde(rename = "ssl-cert-path")]
    pub ssl_cert_path: String,
    #[serde(rename = "ssl-key-path")]
    pub ssl_key_path: String,
    #[serde(rename = "ssl-port")]
    pub ssl_port: u16,
    #[serde(rename = "ssl-host")]
    pub ssl_host: String,
    #[serde(rename = "ssl-protocols")]
    pub ssl_protocols: Vec<String>,
}

#[derive(Deserialize)]
pub struct Monitoring {
    pub prometheus: MonitoringConfig,
    pub grafana: MonitoringConfig,
    pub alertmanager: MonitoringConfig,
}

#[derive(Deserialize)]
pub struct MonitoringConfig {
    pub enabled: bool,
    pub port: u16,
    pub host: String,
}

impl Config {
    pub fn load(path: &str) -> Result<Self, WafError> {
        let file = File::open(path).map_err(|e| WafError::ConfigError(format!("Failed to open config file: {}", e)))?;
        let reader = BufReader::new(file);
        let config: Config = serde_json::from_reader(reader)
            .map_err(|e| WafError::ConfigError(format!("Failed to parse config file: {}", e)))?;
        Ok(config)
    }
}