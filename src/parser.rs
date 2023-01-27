use crate::Phoneme;
use logos::{Logos, Lexer};


#[derive(Logos, Debug, PartialEq)]
enum Token{
    #[token("{")]
    LeftBracket,
    #[token("}")]
    RightBracket,
    #[token("where")]
    Where,
    #[token("=")]
    Equals,
    #[token("_", priority = 2)]
    Underscore,

    #[regex(r"[^_\W]+", |lex| lex.slice().to_owned(), priority=1)]
    Name(String),

    #[token("->")]
    Arrow,

    #[error]
    #[regex(r"[\s]+", logos::skip)]
    Error,
}

pub fn test(){
    let lex: Lexer<Token> = Token::lexer("where V_V {mʰ -> b c -> d}");
    for i in lex {
        let a: Token = i;
        println!("i: {:?}", a);
    }
}