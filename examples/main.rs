use std::{
    collections::{HashSet},
};

use lang_builder::{DictionaryEntry, Syllable, ProtoLanguage, parser::{Token, variable_assignment, sound_change}, phoneme::Phoneme};
use logos::{Lexer, Logos};

fn main() {
    let mut phonology: HashSet<Phoneme> = HashSet::new();
    phonology.insert(Phoneme::VoicedBilabialNasal);
    // phonology.insert(Phoneme::VoicedLinguolabialNasal);

    let mut dictionary = vec![];
    dictionary.push(DictionaryEntry {
        word: vec![Syllable {
            phonemes: vec![Phoneme::VoicedBilabialNasal],
            stressed: true,
        }],
        translation: "example".to_string(),
        notes: "".to_string(),
    });

    let proto_language = ProtoLanguage {
        phonology,
        dictionary,
    };
    println!("Protolanguage: {:?}", proto_language);

    let mut lex: Lexer<Token> = Token::lexer(include_str!("../example.scl"));
    let mut tokens: Vec<Token> = vec![];
    while let Some(token) = lex.next() {
        tokens.push(token);
    }
    println!("{:?}", sound_change().parse(&tokens));
}
