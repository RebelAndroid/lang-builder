use std::collections::HashSet;

use lang_builder::{
    parser::{sound_change, Token},
    phoneme::Phoneme,
    DictionaryEntry, DictionaryEvolution, Evolution, GrammaticalEvolution, Language,
    LanguageHistory, Syllable,
};
use logos::{Lexer, Logos};
use pom::set::Set;

fn main() {
    let mut phonology: HashSet<Phoneme> = HashSet::new();
    phonology.insert(Phoneme::VoicedBilabialNasal);

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
        grammar,
    };
    println!("Protolanguage: {:?}", proto_language);

    let language_history = LanguageHistory {
        proto_language,
        evolutions: vec![
            Evolution::Dictionary(DictionaryEvolution::add(DictionaryEntry {
                word: vec![Syllable {
                    phonemes: vec![Phoneme::VoicedAlveolarNasal],
                    stressed: true,
                }],
                definition: "todo!()".to_string(),
                notes: "".to_string(),
            })),
            // Evolution::Grammatical(GrammaticalEvolution::add("How many grammatical numbers? Yes.".to_string())),
            // Evolution::Grammatical(GrammaticalEvolution::remove("inflect it or something, I don't know".to_string())),
            Evolution::Grammatical(GrammaticalEvolution::replace(
                "inflect it or something, I don't know".to_string(),
                "How many grammatical numbers? Yes.".to_string(),
            )),
        ],
    };

    let modern_language = language_history.get_current();
    println!("modern language: {:?}", modern_language);

    // let mut lex: Lexer<Token> = Token::lexer(include_str!("../example.scl"));
    // let mut tokens: Vec<Token> = vec![];
    // while let Some(token) = lex.next() {
    //     tokens.push(token);
    // }
    // println!("{:?}", sound_change().parse(&tokens));
}
