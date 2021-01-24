// Contains utility functions for the Homonym Language
use crate::HomonymLexer::Token;
use crate::HomonymParser::Expression;
use regex::Regex;
use std::collections::VecDeque;

pub fn match_expression_to_typename_str(expr: &Expression) -> &str {
    match expr {
        Expression::BOOLEAN(_) => "bool",
        Expression::INTVAL(_) => "int",
        Expression::FLTVAL(_) => "float",
        Expression::STRINGVAL(_) => "string",
        _ => panic!(""),
    }
}

pub fn check_parens_valid(tokens: &Vec<Token>) -> bool {
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

// This function is necessary because Logos doesn't support extracting multiple values from a token...
pub fn extract_typenames(type_str: &str) -> Vec<String> {
    let re = Regex::new(r"([`_a-zA-Z][`_a-zA-Z0-9]*)").unwrap();
    re.captures_iter(type_str)
        .map(|cap| String::from(&cap[0]))
        .collect()
}

// Find the semicolons, return a series of indices to split at
pub fn expr_boundaries(tokens: &Vec<Token>) -> Vec<usize> {
    (0..tokens.len())
        .filter(|&ind| tokens[ind] == Token::SEMICOLON)
        .collect()
}
