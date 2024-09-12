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

use std::env;
use std::fs;
use std::path::Path;



fn main() {
    let out_dir = env::var("ZARK_WAF_FINAL_BUILD_PATH").unwrap_or_else(|_| "default_final_build_zark".to_string());
    let dest_path = Path::new(&out_dir);

    // Create the destination directory
    fs::create_dir_all(&dest_path).expect("Failed to create output directory");

    // Copy the executable
    let exec_name = if cfg!(windows) { "zark_waf.exe" } else { "zark_waf" };
    let src_path = Path::new("target/release").join(exec_name);
    let dest_exec_path = dest_path.join(exec_name);
    fs::copy(&src_path, &dest_exec_path).expect("Failed to copy executable");

    println!("Executable copied to: {}", dest_exec_path.display());

    // copy dependencies
    let deps_path = Path::new("target/release/deps");
    if deps_path.exists() {
        let dest_deps_path = dest_path.join("deps");
        fs::create_dir_all(&dest_deps_path).expect("Failed to create deps directory");

        for entry in fs::read_dir(deps_path).expect("Failed to read deps directory") {
            let entry = entry.expect("Failed to read directory entry");
            let path = entry.path();
            if path.is_file() && path.extension().map_or(false, |ext| ext == "dll" || ext == "so") {
                let file_name = path.file_name().unwrap();
                let dest_file = dest_deps_path.join(file_name);
                fs::copy(&path, &dest_file).expect("Failed to copy dependency");
                println!("Copied dependency: {}", file_name.to_string_lossy());
            }
        }
    }

    println!("Post-build tasks completed. Output directory: {}", dest_path.display());
}