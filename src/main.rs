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


mod core;

use clap::Parser;
use log::{error, info};
use crate::core::ZarkWafCore;

#[derive(Parser)]
#[clap(version = env!("CARGO_PKG_VERSION"), author = "I. Zeqiri, E. Gjergji")]
struct Opts {
    #[clap(short, long, default_value = "config/config.json")]
    config: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    env_logger::init();

    // Parse command line arguments
    let opts: Opts = Opts::parse();

    info!("Starting ZARK-WAF...");

    // Initialize the core
    let core = match ZarkWafCore::new(&opts.config).await {
        Ok(core) => core,
        Err(e) => {
            error!("Failed to initialize ZARK-WAF core: {}", e);
            return Err(e.into());
        }
    };
    

    // Run the core
    if let Err(e) = core.run().await {
        error!("ZARK-WAF core encountered an error: {}", e);
        return Err(e.into());
    }

    info!("ZARK-WAF shutting down...");
    Ok(())
}