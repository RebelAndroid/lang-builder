use std::{
    collections::{HashSet},
};

use lang_builder::{DictionaryEntry, Syllable, ProtoLanguage, parser, phoneme::Phoneme};

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
    //println!("{:?}", proto_language.phonology);
    parser::test();


    // c -> g where {$vowel_$vowel, stress, /$vowel_/}
    // statement = sound_change where constraint*
    // sound_change = phoneme_set -> phoneme_set
    // phoneme_set = {phoneme(,phoneme)*} | phoneme
    // phoneme = [^\s,$_\}\{]+
    
}
