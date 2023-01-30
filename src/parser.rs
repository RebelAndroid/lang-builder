use std::fmt::Display;

use crate::Phoneme;
use logos::{Lexer, Logos};
use pom::{
    parser::{is_a, sym},
    Parser,
};

#[derive(Logos, Debug, PartialEq, Clone)]
enum Token {
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
impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(())
    }
}

struct SoundChange {
    start: Phoneme,
    end: Phoneme,
}

fn sound_change() -> Parser<Token, SoundChange> {
    let change = is_a(|x: Token| {
        if let Token::Phoneme(string) = x {
            true
        } else {
            false
        }
    }) + is_a(|x: Token| if let Token::Arrow = x { true } else { false })
        + is_a(|x: Token| {
            if let Token::Phoneme(string) = x {
                true
            } else {
                false
            }
        });
    let x = change.collect();
    x.convert(|tokens| {
        Ok::<SoundChange, String>(SoundChange {
            start: match &tokens[0]{
                Token::Phoneme(s) => {
                    s.as_str().try_into()?
                },
                _ => unreachable!(),
            },
            end: match &tokens[2]{
                Token::Phoneme(s) => s.as_str().try_into()?,
                _ => unreachable!(),
            },
        })
    })
}

pub fn test() {
    let lex: Lexer<Token> = Token::lexer(include_str!("../example.scl"));
    for i in lex {
        let a: Token = i;
        println!("i: {:?}", a);
    }
}
