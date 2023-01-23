use std::{collections::HashSet, fmt::Debug};

#[derive(PartialEq, Eq, Hash)]
enum Phoneme{
    PulmonicConsonant(PulmonicConsonant),
}

#[derive(PartialEq, Eq, Hash)]
enum PulmonicConsonant{
    VoicelessBilabialNasal,
    ///m
    VoicedBilabialNasal,
    VoicedLabiodentalNasal,
    VoicedLinguolabialNasal,
}
impl ToString for PulmonicConsonant{
    fn to_string(&self) -> String {
        match self{
            PulmonicConsonant::VoicelessBilabialNasal => "m̥",
            PulmonicConsonant::VoicedBilabialNasal => "m",
            PulmonicConsonant::VoicedLabiodentalNasal => "ɱ",
            PulmonicConsonant::VoicedLinguolabialNasal => "n̼",
        }.to_string()
    }
}

impl Debug for Phoneme{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::PulmonicConsonant(p) => p.to_string(),
        })
    }
}


impl Debug for PulmonicConsonant{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

trait Romanization{
    fn romanize(phoneme: Phoneme) -> String;
}

struct ProtoLanguage{
    phonology: HashSet<Phoneme>,
}

enum Evolutions{
    Phonetic,
    Grammatical,
    Dictionary
}

struct Language{
    proto_language: ProtoLanguage,
    evolutions: Evolutions,
}

fn main() {
    let mut phonology: HashSet<Phoneme> = HashSet::new();
    phonology.insert(Phoneme::PulmonicConsonant(PulmonicConsonant::VoicedBilabialNasal));
    phonology.insert(Phoneme::PulmonicConsonant(PulmonicConsonant::VoicedLinguolabialNasal));
    let proto_language = ProtoLanguage{
        phonology,
    };
    println!("{:?}", proto_language.phonology);
}
