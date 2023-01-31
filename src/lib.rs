use std::{fmt::Debug, collections::HashSet};

use phoneme::Phoneme;
pub mod parser;
pub mod phoneme;




pub trait Romanize {
    fn romanize(word: Vec<Syllable>) -> Result<String, String>;
}

pub struct ProtoLanguage {
    pub phonology: HashSet<Phoneme>,
    pub dictionary: Vec<DictionaryEntry>,
}

pub enum Evolution {
    Phonetic,
    Grammatical,
    Dictionary,
}

pub struct Language {
    pub proto_language: ProtoLanguage,
    pub evolutions: Vec<Evolution>,
    pub dictionary: Vec<DictionaryEntry>,
}

pub struct Syllable {
    pub phonemes: Vec<Phoneme>,
    pub stressed: bool,
}

pub struct DictionaryEntry {
    pub word: Vec<Syllable>,
    pub translation: String,
    pub notes: String,
}