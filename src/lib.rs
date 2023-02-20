use std::{collections::HashSet, fmt::Debug, hash::Hash};

use parser::SoundChange;
use phoneme::Phoneme;
pub mod parser;
pub mod phoneme;

pub trait Romanize {
    fn romanize(word: Vec<Syllable>) -> Result<String, String>;
}

#[derive(Debug, Clone)]
pub struct Language {
    pub phonology: HashSet<Phoneme>,
    pub dictionary: HashSet<DictionaryEntry>,
    pub grammar: HashSet<String>,
}

pub enum Evolution {
    Phonetic(SoundChange),
    Grammatical(GrammaticalEvolution),
    Dictionary(DictionaryEvolution),
}

pub struct LanguageHistory {
    pub proto_language: Language,
    pub evolutions: Vec<Evolution>,
}

impl LanguageHistory {
    pub fn get_snapshot(&self, last_evolution: usize) -> Language {
        if last_evolution >= self.evolutions.len() {
            panic!();
        }
        let mut current = self.proto_language.clone();
        for i in 0..last_evolution + 1{
            match &self.evolutions[i] {
                Evolution::Phonetic(sound_change) => todo!(),
                Evolution::Grammatical(grammatical_evolution) => todo!(),
                Evolution::Dictionary(dictionary_evolution) => {
                    dictionary_evolution.apply(&mut current)
                }
            }
        }
        current
    }
    pub fn get_current(&self) -> Language {
        self.get_snapshot(self.evolutions.len() - 1)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Syllable {
    pub phonemes: Vec<Phoneme>,
    pub stressed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DictionaryEntry {
    pub word: Vec<Syllable>,
    pub definition: String,
    pub notes: String,
}

#[derive(PartialEq, Eq)]
pub struct DictionaryEvolution {
    start: Option<DictionaryEntry>,
    end: Option<DictionaryEntry>,
}
impl DictionaryEvolution {
    pub fn add(entry: DictionaryEntry) -> Self {
        DictionaryEvolution {
            start: None,
            end: Some(entry),
        }
    }
    pub fn remove(entry: DictionaryEntry) -> Self {
        DictionaryEvolution {
            start: Some(entry),
            end: None,
        }
    }
    pub fn replace(start: DictionaryEntry, end: DictionaryEntry) -> Self {
        DictionaryEvolution {
            start: Some(start),
            end: Some(end),
        }
    }

    pub fn apply(&self, language: &mut Language) {
        match (&self.start, &self.end) {
            (None, None) => unreachable!(),
            (None, Some(entry)) => language.dictionary.insert(entry.clone()),
            (Some(entry), None) => language.dictionary.remove(entry),
            (Some(start), Some(end)) => {
                language.dictionary.remove(start);
                language.dictionary.insert(end.clone())
            },
        };
    }
}

pub struct GrammaticalEvolution {
    start: Option<String>,
    end: Option<String>,
}
