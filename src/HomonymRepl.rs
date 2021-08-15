use logos::Logos;

// Functions for evaluating Homonym code in the CLI
use crate::{HomonymInterpreter, HomonymLexer::Token, HomonymParser};
use std::io::{self, Write};

pub fn take_input() -> String {
    let mut ipt = String::new();
    print!(">>> ");
    io::stdout().lock().flush().expect("Couldn't flush std out");
    io::stdin().read_line(&mut ipt).expect("Invalid Entry");
    ipt
}

pub fn main_loop() {
    let mut context = HomonymParser::Context::new();
    loop {
        let input_str = take_input();
        let lexer = Token::lexer(&input_str);
        let expr = HomonymParser::parse(lexer);
        println!("{:?}", expr);
        let val = HomonymInterpreter::interpret_expr(&expr, &mut context);
        println!("{}", val);
    }
}
