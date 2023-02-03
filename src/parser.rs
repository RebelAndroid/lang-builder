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
    pub case: Vec<CaseItem>,
    pub changes: Vec<PhonemeChange>,
}

#[derive(Debug)]
pub enum CaseItem {
    Underscore,
    Phoneme(Phoneme),
    Variable(String),
}

pub fn sound_change<'a>() -> Parser<'a, Token, SoundChange> {
    let case = (is_a(|x: Token| matches!(x, Token::Variable(_))).map(|v| match v{
        Token::Variable(s) => CaseItem::Variable(s),
        _ => unreachable!()
    })
        | is_a(|x: Token| matches!(x, Token::Phoneme(_))).map(|p| match p{
            Token::Phoneme(p) => CaseItem::Variable(p.as_str().try_into().unwrap()),
            _ => unreachable!()
        })
        | is_a(|x: Token| matches!(x, Token::Underscore)).map(|_| CaseItem::Underscore))
    .repeat(0..);
    let change = is_a(|x: Token| matches!(x, Token::Where))
        * case
        + is_a(|x: Token| matches!(x, Token::LeftCurlyBracket))
        * list(
            phoneme_change(),
            is_a(|x: Token| matches!(x, Token::Comma)).opt(),
        )
        - is_a(|x: Token| matches!(x, Token::RightCurlyBracket));
    change.convert(|(case, changes)| {
        Ok::<SoundChange, String>(SoundChange {
            case,
            changes,
        })
    })
}

#[derive(Debug)]
pub struct PhonemeChange {
    pub start: Phoneme,
    pub end: Phoneme,
}

pub fn phoneme_change<'a>() -> Parser<'a, Token, PhonemeChange> {
    let change = is_a(|x: Token| matches!(x, Token::Phoneme(_)))
        + is_a(|x: Token| x == Token::Arrow)
        + is_a(|x: Token| matches!(x, Token::Phoneme(_)));
    let x = change.collect();
    x.convert(|tokens| {
        Ok::<PhonemeChange, String>(PhonemeChange {
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
    let assignment = is_a(|x: Token| matches!(x, Token::Variable(_)))
        + is_a(|x: Token| x == Token::Equals)
        + is_a(|x: Token| x == Token::LeftSquareBracket)
        + list(
            is_a(|x: Token| matches!(x, Token::Phoneme(_))),
            sym(Token::Comma).opt(),
        )
        + is_a(|x: Token| x == Token::RightSquareBracket);
    let y = assignment.collect();
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
