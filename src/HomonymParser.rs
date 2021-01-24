use crate::HomonymLexer::Token;
use crate::HomonymUtils::{check_parens_valid, extract_typenames};
use logos::{Lexer, Logos};
use std::collections::HashMap;

#[derive(PartialEq, Debug)]
pub enum Expression {
    BOOLEAN(bool),
    INTVAL(i64),
    FLTVAL(f64),
    STRINGVAL(String),
    TEXT(String),
    PLUS(String, String, Box<Expression>, Box<Expression>),
    MINUS(String, String, Box<Expression>, Box<Expression>),
    TIMES(String, String, Box<Expression>, Box<Expression>),
    DIVIDEDBY(String, String, Box<Expression>, Box<Expression>),
    LET(String, String, Box<Expression>),
    IF(
        String,
        Box<Expression>,
        Vec<Box<Expression>>,
        Vec<Box<Expression>>,
    ),
    FUNCTION(String, HashMap<String, String>, Vec<Box<Expression>>),
    RETURN(String, Box<Expression>),
}

pub enum VariableValue {
    INT(i64),
    STRING(String),
    FLOAT(f64),
    BOOL(bool),
}

pub struct Context {
    variable_map: HashMap<String, HashMap<String, VariableValue>>,
    function_map: HashMap<String, Expression>,
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

fn split_binary_operator_expr(lex_vec: &Vec<Token>, split_ind: usize) -> Expression {
    if let Token::TYPEREF(typeref) = &lex_vec[split_ind + 1] {
        let typenames = extract_typenames(typeref);
        assert_eq!(typenames.len(), 2);
        let lsplit: &Vec<Token> = &lex_vec[..split_ind].to_vec();
        let rsplit: &Vec<Token> = &lex_vec[split_ind + 2..].to_vec();
        match lex_vec[split_ind] {
            Token::PLUS => Expression::PLUS(
                typenames[0].clone(),
                typenames[1].clone(),
                Box::<Expression>::new(parse_step(lsplit)),
                Box::<Expression>::new(parse_step(rsplit)),
            ),
            Token::DASH => Expression::MINUS(
                typenames[0].clone(),
                typenames[1].clone(),
                Box::<Expression>::new(parse_step(lsplit)),
                Box::<Expression>::new(parse_step(rsplit)),
            ),
            Token::STAR => Expression::TIMES(
                typenames[0].clone(),
                typenames[1].clone(),
                Box::<Expression>::new(parse_step(lsplit)),
                Box::<Expression>::new(parse_step(rsplit)),
            ),
            Token::FSLASH => Expression::DIVIDEDBY(
                typenames[0].clone(),
                typenames[1].clone(),
                Box::<Expression>::new(parse_step(lsplit)),
                Box::<Expression>::new(parse_step(rsplit)),
            ),
            _ => panic!("{:?} isn't a valid binary operator!", lex_vec[split_ind]),
        }
    } else {
        panic!("Binary operator at location {} is not accompanied by a type reference");
    }
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
        match lex_vec[split_ind] {
            Token::PLUS | Token::DASH | Token::STAR | Token::FSLASH => {
                split_binary_operator_expr(lex_vec, split_ind)
            }
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
