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

use async_trait::async_trait;
use chrono::Local;
use fern::colors::{Color, ColoredLevelConfig};
use log::LevelFilter;
use serde_json::Value;
use std::sync::Arc;
use zark_waf_common::{Module, Broker};
use zark_waf_config_manager::config::ZarkLogger as LoggerConfig;
use crate::error::LoggerError;

pub struct ZarkLogger {
    config: LoggerConfig,
}

impl ZarkLogger {
    pub fn new(config: LoggerConfig) -> Self {
        ZarkLogger { config }
    }

    fn setup_logger(&self) -> Result<(), LoggerError> {
        let colors = ColoredLevelConfig::new()
            .error(Color::Red)
            .warn(Color::Yellow)
            .info(Color::Green)
            .debug(Color::Blue)
            .trace(Color::Magenta);

        let mut base_config = fern::Dispatch::new();

        // Set global log level
        base_config = match self.config.log_level.as_str() {
            "trace" => base_config.level(LevelFilter::Trace),
            "debug" => base_config.level(LevelFilter::Debug),
            "info" => base_config.level(LevelFilter::Info),
            "warn" => base_config.level(LevelFilter::Warn),
            "error" => base_config.level(LevelFilter::Error),
            _ => base_config.level(LevelFilter::Info),
        };

        // Add stdout logger
        if self.config.log_type.contains(&"stdout".to_string()) {
            let stdout_config = fern::Dispatch::new()
                .format(move |out, message, record| {
                    out.finish(format_args!(
                        "[{}][{}][{}] {}",
                        Local::now().format("%Y-%m-%d %H:%M:%S"),
                        record.target(),
                        colors.color(record.level()),
                        message
                    ))
                })
                .chain(std::io::stdout());

            base_config = base_config.chain(stdout_config);
        }

        // Add file logger
        if self.config.log_type.contains(&"file".to_string()) {
            let file_config = fern::Dispatch::new()
                .format(|out, message, record| {
                    out.finish(format_args!(
                        "[{}][{}][{}] {}",
                        Local::now().format("%Y-%m-%d %H:%M:%S"),
                        record.target(),
                        record.level(),
                        message
                    ))
                })
                .chain(fern::log_file(&self.config.log_path)?);

            base_config = base_config.chain(file_config);
        }

        // Add syslog logger (Unix only)
        #[cfg(unix)]
        if self.config.log_type.contains(&"syslog".to_string()) {
            use syslog::{Facility, Formatter3164};

            let formatter = Formatter3164 {
                facility: Facility::LOG_USER,
                hostname: None,
                process: "zark_waf".into(),
                pid: 0,
            };

            let syslog = syslog::unix(formatter).map_err(LoggerError::SyslogError)?;
            let syslog_config = fern::Dispatch::new()
                .format(|out, message, record| {
                    out.finish(format_args!(
                        "[{}][{}] {}",
                        record.target(),
                        record.level(),
                        message
                    ))
                })
                .chain(syslog);

            base_config = base_config.chain(syslog_config);
        }

        // Apply the configuration
        base_config.apply()?;

        Ok(())
    }
}

#[async_trait]
impl Module for ZarkLogger {
    fn name(&self) -> &str {
        "zark_logger"
    }

    async fn init(&mut self, _broker: Arc<Broker>) -> Result<(), Box<dyn std::error::Error>> {
        self.setup_logger()?;
        log::info!("ZarkLogger initialized");
        Ok(())
    }

    async fn execute(&self, input: Value) -> Result<Value, Box<dyn std::error::Error>> {
        // The logger doesn't need to execute anything, but we'll log the input for demonstration
        log::info!("Logger received input: {}", input);
        Ok(Value::Null)
    }

    async fn shutdown(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        log::info!("ZarkLogger shutting down");
        Ok(())
    }
}

#[no_mangle]
pub extern "C" fn _create_module() -> *mut dyn Module {
    
    let config = LoggerConfig {
        log_type: vec!["stdout".to_string(), "file".to_string()],
        log_path: "/var/log/zark_waf.log".to_string(),
        log_level: "info".to_string(),
        log_max_size: 1000000,
        log_max_backups: 5,
        log_max_age: 30,
        log_compress: true,
    };

    Box::into_raw(Box::new(ZarkLogger::new(config)))
}