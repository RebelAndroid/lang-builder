use std::{collections::HashSet, fmt::Display};

use crate::phoneme::Phoneme;
use logos::Logos;
use pom::parser::{is_a, list, sym, Parser};

#[derive(Logos, Debug, PartialEq, Eq, Clone)]
pub enum Token {
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
        write!(
            f,
            "{} {}",
            match self {
                Token::LeftCurlyBracket => "LeftCurlyBracket",
                Token::RightCurlyBracket => "RightCurlyBracket",
                Token::LeftSquareBracket => "LeftSquareBracket",
                Token::RightSquareBracket => "RightSquareBracket",
                Token::Where => "Where",
                Token::Equals => "Equals",
                Token::Underscore => "Underscore",
                Token::Comma => "Comma",
                Token::Phoneme(_) => "Phoneme:",
                Token::Variable(_) => "Variable:",
                Token::Arrow => "Arrow",
                Token::Error => "Error",
            },
            match self {
                Token::Phoneme(p) => p,
                Token::Variable(v) => v,
                _ => "",
            }
        )
    }
}

#[derive(Debug)]
pub struct SoundChange {
    pub start: Phoneme,
    pub end: Phoneme,
}

pub fn sound_change<'a>() -> Parser<'a, Token, SoundChange> {
    let change = is_a(|x: Token| matches!(x, Token::Phoneme(_)))
        + is_a(|x: Token| x == Token::Arrow)
        + is_a(|x: Token| matches!(x, Token::Phoneme(_)));
    let x = change.collect();
    x.convert(|tokens| {
        Ok::<SoundChange, String>(SoundChange {
            start: match &tokens[0] {
                Token::Phoneme(s) => s.as_str().try_into()?,
                _ => unreachable!(),
            },
            end: match &tokens[2] {
                Token::Phoneme(s) => s.as_str().try_into()?,
                _ => unreachable!(),
            },
        })
    })
}

#[derive(Debug)]
pub struct VariableAssignment {
    name: String,
    phoneme_set: HashSet<Phoneme>,
}

pub fn variable_assignment<'a>() -> Parser<'a, Token, VariableAssignment> {
    let assignemnt = is_a(|x: Token| matches!(x, Token::Variable(_)))
        + is_a(|x: Token| x == Token::Equals)
        + is_a(|x: Token| x == Token::LeftSquareBracket)
        + list(
            is_a(|x: Token| matches!(x, Token::Phoneme(_))),
            sym(Token::Comma),
        )
        + is_a(|x: Token| x == Token::RightSquareBracket);
    let y = assignemnt.collect();
    y.convert(|tokens| {
        Ok::<VariableAssignment, String>(VariableAssignment {
            name: if let Token::Variable(x) = &tokens[0] {
                x.clone()
            } else {
                unreachable!()
            },
            phoneme_set: tokens.iter().fold(HashSet::new(), |mut acc, t| {
                if let Token::Phoneme(s) = t {
                    acc.insert(s.as_str().try_into().unwrap());
                    acc
                } else {
                    acc
                }
            }),
        })
    })
}
