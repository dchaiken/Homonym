use crate::HomonymLexer::Token;
use logos::{Lexer, Logos};
use regex::Regex;
use std::collections::{HashMap, VecDeque};

#[derive(PartialEq, Debug)]
pub enum Expression {
    BOOLEAN(bool),
    INTVAL(i64),
    FLTVAL(f64),
    STRINGVAL(String),
    TEXT(String),
    TYPEREF(String),
    PLUS(Box<Expression>, Box<Expression>),
    MINUS(Box<Expression>, Box<Expression>),
    TIMES(Box<Expression>, Box<Expression>),
    DIVIDEDBY(Box<Expression>, Box<Expression>),
}

// This function is necessary because Logos doesn't support extracting multiple values from a token...
pub fn extract_typenames(type_str: &str) -> Vec<String> {
    let re = Regex::new(r"([`_a-zA-Z][`_a-zA-Z0-9]*)").unwrap();
    re.captures_iter(type_str)
        .map(|cap| String::from(&cap[0]))
        .collect()
}

pub fn match_expression_to_typename_str(expr: &Expression) -> &str {
    match expr {
        Expression::BOOLEAN(_) => "bool",
        Expression::INTVAL(_) => "int",
        Expression::FLTVAL(_) => "float",
        Expression::STRINGVAL(_) => "string",
        _ => panic!(""),
    }
}

// Return precedence of a token (lower should be evaluated first)
pub fn operator_precendence(token: &Token) -> i64 {
    match token {
        Token::TEXT(_)
        | Token::STRINGVAL(_)
        | Token::INTVAL(_)
        | Token::FLTVAL(_)
        | Token::TRUE
        | Token::FALSE => 0,
        Token::STAR | Token::FSLASH => 25,
        Token::PLUS | Token::DASH => 50,
        _ => 100,
    }
}

fn get_max_priority_operator(tokens: &Vec<Token>) -> usize {
    tokens.iter().enumerate().fold(0, |maxind, (ind, token)| {
        if operator_precendence(&tokens[maxind]) > operator_precendence(token) {
            maxind
        } else {
            ind
        }
    })
}

fn check_parens_valid(tokens: &Vec<Token>) -> bool {
    let mut parens_deque = VecDeque::<Token>::new();
    for token in tokens {
        if *token == Token::LPAREN {
            parens_deque.push_back(Token::LPAREN);
        } else if *token == Token::RPAREN {
            if parens_deque.is_empty() {
                return false;
            } else {
                // check last token to allow for other character checks later
                if parens_deque.pop_back().unwrap() != Token::LPAREN {
                    return false;
                }
            }
        }
    }
    true
}

fn split_binary_operator_expr(lex_vec: &Vec<Token>, split_ind: usize) -> Expression {
    Expression::STRINGVAL("TODO!!!".to_string())
}

pub fn parse_step(lex_vec: &Vec<Token>) -> Expression {
    //For parentheses: manually detect them in this function, use them to override precedence
    if lex_vec.len() == 1 {
        match &lex_vec[0] {
            Token::INTVAL(i) => Expression::INTVAL(*i),
            Token::FLTVAL(f) => Expression::FLTVAL(*f),
            Token::STRINGVAL(s) => Expression::STRINGVAL(s.to_string()),
            Token::TEXT(s) => Expression::TEXT(s.to_string()),
            Token::TRUE => Expression::BOOLEAN(true),
            Token::FALSE => Expression::BOOLEAN(false),
            _ => panic!("I can't parse that as its own expression"),
        }
    } else {
        //Find highest precedence operator, then split there
        let split_ind: usize = get_max_priority_operator(lex_vec);
        let lsplit: &Vec<Token> = &lex_vec[..split_ind].to_vec();
        let rsplit: &Vec<Token> = &lex_vec[split_ind + 1..].to_vec();
        match lex_vec[split_ind] {
            Token::PLUS => Expression::PLUS(
                Box::<Expression>::new(parse_step(lsplit)),
                Box::<Expression>::new(parse_step(rsplit)),
            ),
            Token::DASH => Expression::MINUS(
                Box::<Expression>::new(parse_step(lsplit)),
                Box::<Expression>::new(parse_step(rsplit)),
            ),
            Token::STAR => Expression::TIMES(
                Box::<Expression>::new(parse_step(lsplit)),
                Box::<Expression>::new(parse_step(rsplit)),
            ),
            Token::FSLASH => Expression::DIVIDEDBY(
                Box::<Expression>::new(parse_step(lsplit)),
                Box::<Expression>::new(parse_step(rsplit)),
            ),
            _ => panic!("I didn't know how to parse that!"),
        }
    }
}

pub fn parse(lexer: Lexer<Token>) -> Expression {
    let lex_vec = lexer.collect::<Vec<Token>>();
    assert!(check_parens_valid(&lex_vec));
    parse_step(&lex_vec)
}

#[test]
fn type_capture_test() {
    assert_eq!(
        extract_typenames("<int, string, float>"),
        vec!(
            String::from("int"),
            String::from("string"),
            String::from("float")
        )
    )
}
