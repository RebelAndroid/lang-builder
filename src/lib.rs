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
        for i in 0..last_evolution + 1 {
            match &self.evolutions[i] {
                Evolution::Phonetic(sound_change) => todo!(),
                Evolution::Grammatical(grammatical_evolution) => {
                    match grammatical_evolution.apply(&mut current) {
                        Ok(_) => {},
                        Err(e) => panic!("{}", e),
                    }
                }
                Evolution::Dictionary(dictionary_evolution) => {
                    match dictionary_evolution.apply(&mut current) {
                        Ok(_) => {},
                        Err(e) => panic!("{}", e),
                    }
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
        Self {
            start: None,
            end: Some(entry),
        }
    }
    pub fn remove(entry: DictionaryEntry) -> Self {
        Self {
            start: Some(entry),
            end: None,
        }
    }
    pub fn replace(start: DictionaryEntry, end: DictionaryEntry) -> Self {
        Self {
            start: Some(start),
            end: Some(end),
        }
    }

    pub fn apply(&self, language: &mut Language) -> Result<(), String>{
        match (&self.start, &self.end) {
            (None, None) => unreachable!(),
            (None, Some(entry)) => {
                if !language.dictionary.insert(entry.clone()) {
                    return Result::Err(format!("unable to add dictionary entry: {:?}", entry));
                }
            }
            (Some(entry), None) => {
                if !language.dictionary.remove(entry) {
                    return Result::Err(format!("unable to remove dictionary entry: {:?}", entry));
                }
            }
            (Some(start), Some(end)) => {
                if !language.dictionary.remove(start) {
                    return Result::Err(format!(
                        "unable to remove dictionary entry {:?} for replacement",
                        start
                    ));
                };
                if !language.dictionary.insert(end.clone()) {
                    return Result::Err(format!(
                        "unable to insert dictionary entry {:?} for replacement",
                        end
                    ));
                };
            }
        };
        return Result::Ok(());
    }
}

pub struct GrammaticalEvolution {
    start: Option<String>,
    end: Option<String>,
}

impl GrammaticalEvolution {
    pub fn add(entry: String) -> Self {
        Self {
            start: None,
            end: Some(entry),
        }
    }
    pub fn remove(entry: String) -> Self {
        Self {
            start: Some(entry),
            end: None,
        }
    }
    pub fn replace(start: String, end: String) -> Self {
        Self {
            start: Some(start),
            end: Some(end),
        }
    }

    pub fn apply(&self, language: &mut Language) -> Result<(), String> {
        match (&self.start, &self.end) {
            (None, None) => unreachable!(),
            (None, Some(entry)) => {
                if !language.grammar.insert(entry.clone()) {
                    return Result::Err(format!("unable to add grammatical entry: {}", entry));
                }
            }
            (Some(entry), None) => {
                if !language.grammar.remove(entry) {
                    return Result::Err(format!("unable to remove grammatical entry: {}", entry));
                }
            }
            (Some(start), Some(end)) => {
                if !language.grammar.remove(start) {
                    return Result::Err(format!(
                        "unable to remove grammatical entry {} for replacement",
                        start
                    ));
                };
                if !language.grammar.insert(end.clone()) {
                    return Result::Err(format!(
                        "unable to insert grammatical entry {} for replacement",
                        end
                    ));
                };
            }
        };
        return Result::Ok(());
    }
}
