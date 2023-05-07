use crate::Jaeum;
use std::{
    error::Error as StdError,
    fmt::{Display, Formatter, Result as FmtResult},
};

/// Contains all the possible error conditions that can arise within this crate.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Error {
    /// Denotes that a [`char`] outside of:
    ///
    /// * Hangul Jamo / Initial consonants (`U+1100`--`U+1112`)
    /// * Hangul Jamo / Medial vowels (`U+1161`--`U+1175`)
    /// * Hangul Jamo / Final consonants (`U+11A8`--`U+11C2`)
    /// * Hangul Compatibility Jamo / Consonant letters (`U+3131`--`U+314E`)
    /// * Hangul Compatibility Jamo / Vowel letters (`U+314F`--`U+3163`)
    /// * Halfwidth and Fullwidth Forms / Halfwidth Hangul variants (`U+FFA1`--`U+FFBE`,
    ///   `U+FFC2`--`U+FFC7`, `U+FFCA`--`U+FFCF`, `U+FFD2`--`U+FFD7`, `U+FFDA`--`U+FFDC`)
    ///
    /// was tried converting into a [`Jaeum`], [`Moeum`](crate::Moeum),
    /// [`Choseong`](crate::Choseong), [`Jungseong`](crate::Jungseong), or
    /// [`Jongseong`](crate::Jongseong) respectively.
    ///
    /// Additionally, the following ranges are also tested against the [`char`] if the
    /// [`archiac-korean`](crate#archaic-korean) feature is enabled:
    ///
    /// * Hangul Jamo / Old initial consonants (`U+1113`--`U+115E`)
    /// * Hangul Jamo / Old medial vowels (`U+1176`--`U+11A7`)
    /// * Hangul Jamo / Old final consonants (`U+11C3`--`U+11FF`)
    /// * Hangul Compatibility Jamo / Old consonant letters (`U+3165`--`U+3186`)
    /// * Hangul Compatibility Jamo / Old vowel letters (`U+3187`--`U+318E`)
    /// * Hangul Jamo Extended-A / Old initial consonants (`U+A960`--`U+A97C`)
    /// * Hangul Jamo Extended-B / Old medial vowels (`U+D7B0`--`U+D7C6`)
    /// * Hangul Jamo Extended-B / Old final consonants (`U+D7CB`--`U+D7FB`)
    NonJamo(char),
    /// Denotes that a [`char`] outside the Precomposed Korean Syllables range (`U+AC00`--`U+D7A3`)
    /// was tried converting into a [`Syllable`](crate::Syllable).
    NonKorean(char),
    /// Denotes that a consonant (자음, [`Jaeum`]) cannot be placed in the initial consonant (초성,
    /// [`Choseong`](crate::Choseong)) position.
    NotApplicableToChoseong(Jaeum),
    /// Denotes that a consonant (자음, [`Jaeum`]) cannot be placed in the final consonant (종성,
    /// [`Jongseong`](crate::Jongseong)) position.
    NotApplicableToJongseong(Jaeum),
}
impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            Self::NonJamo(coi) => write!(f, "'{}' is not a correct Jamo character", coi),
            Self::NonKorean(coi) => write!(f, "'{}' is not a Precomposed Korean Sylable", coi),
            Self::NotApplicableToChoseong(jaeum) => {
                write!(f, "{:?} cannot be used as an initial consonant", jaeum)
            }
            Self::NotApplicableToJongseong(jaeum) => {
                write!(f, "{:?} cannot be used as a final consonant", jaeum)
            }
        }
    }
}
impl StdError for Error {}
