#![allow(dead_code)]
mod HomonymInterpreter;
mod HomonymLexer;
mod HomonymParser;
mod HomonymRepl;
mod HomonymTypechecker;
mod HomonymUtils;

use clap::{App, Arg};
use regex::Regex;

fn main() {
    let args = App::new("Homonym the Language")
        .version("0.1.0")
        .author("Daniel Chaiken")
        .about("Homonym: a programming language with variable overloading")
        .arg(
            Arg::with_name("file")
                .short("f")
                .long("file")
                .takes_value(true)
                .help("Homonym file to run NOT YET IMPLEMENTED"),
        )
        .get_matches();
    if let Some(filename) = args.value_of("file") {
        println!("{}", filename);
        println!("Sorry, parsing files is not yet implemented");
    } else {
        HomonymRepl::main_loop();
    }
}

#[test]
fn regex_test() {
    let re = Regex::new(r"([`_a-zA-Z][`_a-zA-Z0-9]*)").unwrap();
    let text = "<int, string, float,>";
    for cap in re.captures_iter(text) {
        println!("{:?}", cap);
    }
}
