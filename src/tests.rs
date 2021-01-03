use logos::Logos;
use HomonymLexer::Token;
use HomonymParser::{parse, Expression};
fn main() {}

#[test]
fn parse_basic_expression() {
    let lex = Token::lexer("4 + 4");
    assert_eq!(
        Expression::PLUS(
            Box::new(Expression::INTVAL(4)),
            Box::new(Expression::INTVAL(4)),
        ),
        parse(lex)
    );
}

#[test]
fn parse_order_of_ops() {
    let lex = Token::lexer("4 + 5 * 7");
    assert_eq!(
        Expression::PLUS(
            Box::new(Expression::INTVAL(4)),
            Box::new(Expression::TIMES(
                Box::new(Expression::INTVAL(5)),
                Box::new(Expression::INTVAL(7))
            ),)
        ),
        parse(lex)
    );
}

#[test]
fn parse_l_to_r() {
    let lex = Token::lexer("4 + 5 - 7");
    assert_eq!(
        Expression::MINUS(
            Box::new(Expression::PLUS(
                Box::new(Expression::INTVAL(4)),
                Box::new(Expression::INTVAL(5))
            ),),
            Box::new(Expression::INTVAL(7)),
        ),
        parse(lex)
    );
}
