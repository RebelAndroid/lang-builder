// use std::fmt::Debug;
macro_rules! phonemes{
    ($($name:ident $symbol:expr,)+) => {
        #[derive(PartialEq, Eq, Hash)]
        pub enum Phoneme{
            $(
                #[doc=$symbol]
                $name,
            )+
        }
        impl std::fmt::Debug for Phoneme {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(
                    f,
                    "{}",
                    match self {
                        $(Phoneme::$name => $symbol,)+
                    }
                    .to_string()
                )
            }
        }
        impl TryFrom<&str> for Phoneme{
            type Error = String; // TODO: proper error type

            fn try_from(value: &str) -> Result<Self, Self::Error> {
                match value{
                    $(
                        $symbol => Ok(Phoneme::$name),
                    )+
                    _ => Err(format!("invalid phoneme: {}", value)),
                }
            }
        }
    };

}

phonemes!(
    VoicedBilabialNasal     "m",
    VoicelessBilabialNasal  "m̥",
    VoicedLabiodentalNasal  "ɱ",
    VoicedLinguolabialNasal "n̼",
    VoicelessAlveolarNasal  "n̥",
    VoicedAlveolarNasal     "n",
    VoicelessRetroflexNasal "ɳ̊",
    VoicedRetroflexNasal    "ɳ",
    VoicelessPalatalNasal   "ɲ̊",
    VoiceledPalatalNasal    "ɲ",
    VoicelessVelarNasal     "ŋ̊",
    VoiceledVelarNasal      "ŋ",
    VoicedUvularNasal       "ɴ",
    VoicelessBilabalPlosive "p",
    VoicedBilabialPlosive   "b",
    VoicelessLabiodentalPlosive "p̪",
    VoicedLabiodentalPlosive "b̪",
    VoicelessLinguolabialPlosive "t̼",
    VoicedLinguolabialPlosive "d̼",
    VoicelessAlveolarPlosive "t",
    VoicedAlveolarPlosive "d",
    VoicelessRetroflexPlosive "ʈ",
    VoicedRetroflexPlosive "ɖ",
    VoicelessPalatalPlosive "c",
    VoicedPalatalPlosive "ɟ",
    VoicelessVelarPlosive "k",
    VoicedVelarPlosive "g",
    VoicelessUvularPlosive "q",
    VoicedUvularPlosive "ɢ",
    EpiglottalPlosive "ʡ",
    GlottalStop "ʔ",
    VoicelessAlveolarSibilant "s",
    VoicedAlveolarSibilant "z",
    VoicelessPostalveolarSibilant "ʃ",
    VoicedPostavelolarSibilant "ʒ",
    VoicelessRetroflexSibilant "ʂ",
    VoicedRetroflexSibilant "ʐ",
    VoicelessPalatalSibilant "ɕ",
    VoicedPalatalSibilant "ʑ",
    VoicelessBilabialFricative "ɸ",
    VoicedBilabialFricative "β",
    VoicelessLabiodentalFricative "f",
    VoicedLabiodentalFricative "v",
);

#[cfg(test)]
#[test]
fn test() {
    let x = Phoneme::VoicedBilabialNasal;
    println!("VociedBilabialNasal: {:?}", x);
    let y: Result<Phoneme, String> = "m".try_into();
    println!("m: {:?}", y);
}
