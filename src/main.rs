use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
};

#[derive(PartialEq, Eq, Hash)]
enum Phoneme {
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

type Romanization = HashMap<Phoneme, String>;

struct ProtoLanguage {
    phonology: HashSet<Phoneme>,
    dictionary: Vec<DictionaryEntry>,
}

enum Evolutions {
    Phonetic,
    Grammatical,
    Dictionary,
}

struct Language {
    proto_language: ProtoLanguage,
    evolutions: Evolutions,
    dictionary: Vec<DictionaryEntry>,
}

struct Syllable {
    phonemes: Vec<Phoneme>,
    stressed: bool,
}

struct DictionaryEntry {
    word: Vec<Syllable>,
    translation: String,
    notes: String,
}

fn main() {
    let mut phonology: HashSet<Phoneme> = HashSet::new();
    phonology.insert(Phoneme::VoicedBilabialNasal);
    phonology.insert(Phoneme::VoicedLinguolabialNasal);

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
    println!("{:?}", proto_language.phonology);
}
