// use std::fmt::Debug;
macro_rules! phoneme{
    ($($name:ident, $symbol:expr)+) => {
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

phoneme!(VoicedBilabialNasal, "m");

#[cfg(test)]
#[test]
fn test() {
    let x = Phoneme::VoicedBilabialNasal;
    println!("VociedBilabialNasal: {:?}", x);
    let y: Result<Phoneme, String> = "m".try_into();
    println!("m: {:?}", y);
}
