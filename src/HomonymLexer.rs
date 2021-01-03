use logos::Logos;

#[derive(Logos, Debug, PartialEq, Clone)]
pub enum Token {
    #[token(".")]
    PERIOD,
    #[token(",")]
    COMMA,
    #[token("\"")]
    QUOTE,
    #[token("'")]
    APOST,
    #[token("(")]
    LPAREN,
    #[token(")")]
    RPAREN,
    #[token("[")]
    LBRACK,
    #[token("]")]
    RBRACK,
    #[token("{")]
    LBRACE,
    #[token("}")]
    RBRACE,
    #[token(";")]
    SEMICOLON,
    #[token(":")]
    COLON,
    #[token("|")]
    PIPE,
    #[token("+")]
    PLUS,
    #[token("-")]
    DASH,
    #[token("*")]
    STAR,
    #[token("/")]
    FSLASH,
    #[token("%")]
    PERCENT,
    #[token("=")]
    ASSIGNEQUAL,
    #[token("==")]
    COMPEQUAL,
    #[token("<")]
    LESS,
    #[token(">")]
    GREATER,
    #[token("<=")]
    LEQ,
    #[token(">=")]
    GEQ,
    #[token("and")]
    AND,
    #[token("or")]
    OR,
    #[token("not")]
    NOT,
    #[token("the")]
    THE,
    #[token("int")]
    INT,
    #[token("float")]
    FLOAT,
    #[token("string")]
    STRING,
    #[token("bool")]
    BOOL,
    #[token("let")]
    LET,
    #[token("function")]
    FUNCTION,
    #[token("return")]
    RETURN,
    #[token("if")]
    IF,
    #[token("else")]
    ELSE,
    #[token("while")]
    WHILE,
    #[token("true")]
    TRUE,
    #[token("false")]
    FALSE,
    #[regex(r"-?[0-9]+", |lex| lex.slice().parse::<i64>().unwrap())]
    INTVAL(i64),
    #[regex(r"-?[0-9]*\.[0-9]+", |lex| lex.slice().parse::<f64>().unwrap())]
    FLTVAL(f64),
    #[regex("\"[^\"]*\"", |lex| String::from(lex.slice()))]
    STRINGVAL(String),
    #[regex("[`_a-zA-Z][`_a-zA-Z0-9]*", |lex| String::from(lex.slice()))]
    TEXT(String),
    #[regex("<[[`_a-zA-Z][`_a-zA-Z0-9]*,]*[`_a-zA-Z][`_a-zA-Z0-9]*,?>", |lex| String::from(lex.slice()))]
    TYPEREF(String),
    #[error]
    #[regex(r"[ \t\n\f]+", logos::skip)]
    #[regex(r"//.*\n", logos::skip)]
    ERROR,
}
#[test]
fn empty() {
    let mut lex = Token::lexer("");
    assert_eq!(lex.next(), None);
}
#[test]
fn short() {
    let mut lex = Token::lexer("a");
    assert_eq!(lex.next(), Some(Token::TEXT(String::from("a"))));
}
#[test]
fn typerefs() {
    let mut lex = Token::lexer("<int><int,string>");
    assert_eq!(lex.next(), Some(Token::TYPEREF(String::from("<int>"))));
    assert_eq!(
        lex.next(),
        Some(Token::TYPEREF(String::from("<int,string>")))
    );
}
#[test]
fn printstuff() {
    let lex = Token::lexer("let fcounts the int = 34; let fcounts the float = .34; let fcounts the string = \"EUIRHEI\";");
    for token in lex {
        println!("{:?}", token);
    }
}
