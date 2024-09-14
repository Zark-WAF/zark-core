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

use chrono::{DateTime, Utc};
use rand::Rng;
use std::time::SystemTime;

pub struct Uid(String);

impl Uid {
    pub fn new() -> Self {
        let now = SystemTime::now();
        let timestamp = now.duration_since(SystemTime::UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();

        let random_part: String = rand::thread_rng()
            .sample_iter(&rand::distributions::Alphanumeric)
            .take(12)
            .map(char::from)
            .collect();

        let uid = format!("{:X}{}", timestamp, random_part)
            .chars()
            .collect::<Vec<char>>()
            .chunks(4)
            .map(|c| c.iter().collect::<String>())
            .collect::<Vec<String>>()
            .join("-");

        Uid(uid)
    }

    pub fn to_string(&self) -> String {
        self.0.clone()
    }

    pub fn from_string(s: &str) -> Option<Self> {
        if s.len() == 23 && s.chars().filter(|&c| c == '-').count() == 3 {
            Some(Uid(s.to_string()))
        } else {
            None
        }
    }

    pub fn get_timestamp(&self) -> Option<DateTime<Utc>> {
        let timestamp_hex = self.0.split('-').next()?;
        let timestamp = u64::from_str_radix(timestamp_hex, 16).ok()?;
        let datetime = DateTime::<Utc>::from(SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(timestamp));
        Some(datetime)
    }
}


