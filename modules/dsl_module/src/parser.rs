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

use quick_xml::de::from_str;
use crate::ast::*;
use crate::error::DslError;

pub fn parse(input: &str) -> Result<Rules, DslError> {
    let rules: Rules = from_str(input)?;
    Ok(rules)
}

fn parse_condition(pair: pest::iterators::Pair<Rule>) -> Condition {
    let mut inner = pair.into_inner();
    let field = inner.next().unwrap().as_str().to_string();
    let operator = match inner.next().unwrap().as_str() {
        "contains" => Operator::Contains,
        "equals" => Operator::Equals,
        "starts_with" => Operator::StartsWith,
        "ends_with" => Operator::EndsWith,
        _ => unreachable!(),
    };
    let value = inner.next().unwrap().as_str().to_string();
    Condition { field, operator, value }
}

fn parse_action(pair: pest::iterators::Pair<Rule>) -> Action {
    match pair.as_str() {
        "BLOCK" => Action::Block,
        "ALLOW" => Action::Allow,
        "LOG" => Action::Log,
        _ => unreachable!(),
    }
}

