use crate::Phoneme;
use logos::{Logos, Lexer};


#[derive(Logos, Debug, PartialEq)]
enum Token{
    #[token("{")]
    LeftCurlyBracket,
    #[token("}")]
    RightCurlyBracket,

    #[token("[")]
    LeftSquareBracket,
    #[token("]")]
    RightSquareBracket,

    #[token("where")]
    Where,

    #[token("=")]
    Equals,
    #[token("_")]
    Underscore,
    #[token(",")]
    Comma,

    #[regex(r"[^_\W]+", |lex| lex.slice().to_owned())]
    Phoneme(String),

    #[regex(r"\$[^_\W]+", |lex| lex.slice().to_owned())]
    Variable(String),

    #[token("->")]
    Arrow,

    #[error]
    #[regex(r"[\s]+", logos::skip)]
    Error,
}

pub fn test(){
    let lex: Lexer<Token> = Token::lexer(include_str!("../example.scl"));
    for i in lex {
        let a: Token = i;
        println!("i: {:?}", a);
    }
}