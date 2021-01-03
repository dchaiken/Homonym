#![allow(dead_code)]
mod HomonymInterpreter;
mod HomonymLexer;
mod HomonymParser;

use regex::Regex;

fn main() {
    let re = Regex::new(r"([`_a-zA-Z][`_a-zA-Z0-9]*)").unwrap();
    let text = "<int, string, float,>";
    for cap in re.captures_iter(text) {
        println!("{:?}", cap);
    }
}

#[test]
fn regex_test() {}
