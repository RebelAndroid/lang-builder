use std::{fmt::Debug, collections::HashSet};
pub mod parser;

#[derive(PartialEq, Eq, Hash)]
pub enum Phoneme {
    VoicelessBilabialNasal,
    ///m
    VoicedBilabialNasal,
    VoicedLabiodentalNasal,
    VoicedLinguolabialNasal,
}

impl Debug for Phoneme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Phoneme::VoicelessBilabialNasal => "m̥",
                Phoneme::VoicedBilabialNasal => "m",
                Phoneme::VoicedLabiodentalNasal => "ɱ",
                Phoneme::VoicedLinguolabialNasal => "n̼",
            }
            .to_string()
        )
    }
}
impl TryFrom<&str> for Phoneme{
    type Error = String; // TODO: proper error type

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value{
            "m" => Ok(Phoneme::VoicedBilabialNasal),
            "m̥" => Ok(Phoneme::VoicelessBilabialNasal),
            _ => Err(format!("invalid phoneme: {}", value)),
        }
    }
}


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