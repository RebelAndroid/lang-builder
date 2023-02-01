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
    //nasals
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
    // plosives
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
    // sibilant fricatives
    VoicelessAlveolarSibilant "s",
    VoicedAlveolarSibilant "z",
    VoicelessPostalveolarSibilant "ʃ",
    VoicedPostavelolarSibilant "ʒ",
    VoicelessRetroflexSibilant "ʂ",
    VoicedRetroflexSibilant "ʐ",
    VoicelessPalatalSibilant "ɕ",
    VoicedPalatalSibilant "ʑ",
    // non-sibilant fricatives
    VoicelessBilabialFricative "ɸ",
    VoicedBilabialFricative "β",
    VoicelessLabiodentalFricative "f",
    VoicedLabiodentalFricative "v",
    VoicelessLinguolabialFricative "θ̼",
    VoicedLinguolabialFricative "ð̼",
    VoicelessDentalFricative "θ",
    VoicedDentalFricative "ð",
    VoicelessAlveolarFricative "θ̠",
    VoicedAlveolarFricative "ð̠",
    VoicelessPostalveolarFricative "ɹ̠̊˔",
    VoicedPostalveolarFricative "ɹ̠˔",
    VoicelessRetroflexFricative "ɻ̊˔",
    VoicedRetroflexFricative "ɻ˔",
    VoicelessPalatalFricative "ç",
    VoicedPalatalFricative "ʝ",
    VoicelessVelarFricative "x",
    VoicedVelarFricative "ɣ",
    VoicelessUvularFricative "χ",
    VoicedUvularFricative "ʁ",
    VoicelessEpiglottalFricative "ħ",
    VoicedEpiglottalFricative "ʕ",
    VoicelessGlottalFricative "h",
    VoicedGlottalFricative "ɦ",
    // approximants
    VoicedLabiodentalApproximant "ʋ",
    VoicedAlveolarApproximant "ɹ",
    VoicedRetroflexApproximant "ɻ",
    VoicedPalatalApproximant "j",
    VoicedVelarApproximant "ɰ",
    CreakyVoicedGlottalApproximant "ʔ̞",
    // taps/flaps
    VoicedBilabialFlap "ⱱ̟",
    VoicedLabiodentalFlap "ⱱ",
    VoicedLinguolabialFlap "ɾ̼",
    VoicelessAlveolarFlap "ɾ̥",
    VoicedAlveolarFlap "ɾ",
    VoicelessRetroflexFlap "ɽ̊",
    VoicedRetroflexFlap "ɽ",
    VoicedUvularFlap "ɢ̆",
    VoicedEpiglottalFlap "ʡ̆",
    // trills
    VoicelessBilabialTrill "ʙ̥",
    VoicedBilabialTrill "ʙ",
    VoicelessAlveolarTrill "r̥",
    VoicedAlveolarTrill "r",
    VoicelessRetroflexTrill "ɽ̊r̥",
    VoicedRetroflexTrill "ɽr",
    VoicelessUvularTrill "ʀ̥",
    VoicedUvularTrill "ʀ",
    VoicelessEpiglottalTrill "ʜ",
    VoicedEpiglottalTrill "ʢ",
    // lateral fricatives
    VoicelessAlveolarLateralFricative "ɬ",
    VoicedAlveolarLateralFricative "ɮ",
    VoicelessRetroflexLateralFricative "ꞎ",
    VoicedRetroflexLateralFricative "ɭ˔",
    VoicedPalatalLateralFricative "ʎ̝",
    VoicedVelarLateralFricative "ʟ̝",
    // lateral approximants
    VoicedAlveolarLateralApproximant "l",
    VoicedRetroflexLateralApproximant "ɭ",
    VoicedPalatalLateralApproximant "ʎ",
    VoicedVelarLateralApproximant "ʟ",
    VoicedUvularLateralApproximant "ʟ̠",
    // lateral taps/flaps
    VoicelessAlveolarLateralFlap "ɺ̥",
    VoicedAlveolarLateralFlap "ɺ",
    VoicedPalatalLateralFlap "ʎ̆",
    VoicedVelarLateralFlap "ʟ̆",

);

#[cfg(test)]
#[test]
fn test() {
    let x = Phoneme::VoicelessLinguolabialFricative;
    println!("VoicelessLinguolabialFricative: {:?}", x);
    let y: Result<Phoneme, String> = "m".try_into();
    println!("m: {:?}", y);
    assert!(false);
}
