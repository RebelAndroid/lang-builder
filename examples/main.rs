use std::{
    collections::{HashSet},
};

use lang_builder::{DictionaryEntry, Syllable, parser::{Token, sound_change}, phoneme::Phoneme, Language};
use logos::{Lexer, Logos};

fn main() {
    let mut phonology: HashSet<Phoneme> = HashSet::new();
    phonology.insert(Phoneme::VoicedBilabialNasal);
    // phonology.insert(Phoneme::VoicedLinguolabialNasal);

    let mut dictionary = HashSet::new();
    dictionary.insert(DictionaryEntry {
        word: vec![Syllable {
            phonemes: vec![Phoneme::VoicedBilabialNasal],
            stressed: true,
        }],
        definition: "example".to_string(),
        notes: "".to_string(),
    });
    let mut grammar = HashSet::new();
    grammar.insert("inflect it or something, I don't know".to_string());

    let proto_language = Language {
        phonology,
        dictionary,
        grammar
    };
    println!("Protolanguage: {:?}", proto_language);

    let mut lex: Lexer<Token> = Token::lexer(include_str!("../example.scl"));
    let mut tokens: Vec<Token> = vec![];
    while let Some(token) = lex.next() {
        tokens.push(token);
    }
    println!("{:?}", sound_change().parse(&tokens));
}
