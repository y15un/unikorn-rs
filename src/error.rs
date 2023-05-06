use crate::Jaeum;
use std::{
    error::Error as StdError,
    fmt::{Display, Formatter, Result as FmtResult},
};

/// Contains all the possible error conditions that can arise within this crate.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Error {
    /// Denotes that a [`char`] outside the Hangul Compatibility Jamo range (U+3131 'ㄱ' -- U+3163
    /// 'ㅣ') was tried converting into a [`Jaeum`], [`Moeum`], [`Choseong`], [`Jungseong`], or
    /// [`Jongseong`] respectively.
    NonJamo(char),
    /// Denotes that a [`char`] outside the Precomposed Korean Syllables range (U+AC00 '가' --
    /// U+D7A3 '힣') was tried converting into a [`Syllable`].
    NonKorean(char),
    /// Denotes that a consonant (자음, [`Jaeum`]) cannot be placed in the initial consonant (초성,
    /// [`Choseong`]) position.
    NotApplicableToChoseong(Jaeum),
    /// Denotes that a consonant (자음, [`Jaeum`]) cannot be placed in the final consonant (종성,
    /// [`Jongseong`]) position.
    NotApplicableToJongseong(Jaeum),
}
impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            Self::NonJamo(coi) => write!(f, "'{}' is not a Hangul Compatibility Jamo", coi),
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
