use crate::{
    consonant::{Choseong, HalfwidthJaeum, Jaeum, Jongseong},
    vowel::{Jungseong, Moeum},
};
use std::{
    error::Error as StdError,
    fmt::{Debug, Display, Formatter, Result as FmtResult},
};

/// Groups all the possible erroneous circumstances within this library.
#[derive(Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
pub enum Error {
    /// The [`char`] given is not valid as initial consonant.
    NonChoseongTryFromChar(char),
    /// The [`HalfwidthJaeum`] given is not valid as initial consonant.
    NonChoseongTryFromHalfwidthJaeum(HalfwidthJaeum),
    /// The [`Jaeum`] given is not valid as initial consonant.
    NonChoseongTryFromJaeum(Jaeum),
    /// The [`Jongseong`] given is not valid as initial consonant.
    NonChoseongTryFromJongseong(Jongseong),
    /// The [`char`] given is not a valid halfwidth consonant.
    NonHalfwidthJaeumTryFromChar(char),
    /// The [`char`] given is not a valid halfwidth vowel.
    NonHalfwidthMoeumTryFromChar(char),
    /// The [`char`] given is not a valid consonant.
    NonJaeumTryFromChar(char),
    /// The [`char`] given is not valid as final consonant.
    NonJongseongTryFromChar(char),
    /// The [`Choseong`] given is not valid as final consonant.
    NonJongseongTryFromChoseong(Choseong),
    /// The [`HalfwidthJaeum`] given is not valid as final consonant.
    NonJongseongTryFromHalfwidthJaeum(HalfwidthJaeum),
    /// The [`Jaeum`] given is not valid as final consonant.
    NonJongseongTryFromJaeum(Jaeum),
    /// The [`char`] given is not valid as medial vowel.
    NonJungseongTryFromChar(char),
    /// The `char` given is not a valid Korean syllable/alphabet/letter.
    NonKoreanTryFromChar(char),
    /// The [`char`] given is not a valid vowel.
    NonMoeumTryFromChar(char),
    /// The [`Choseong`] given does not have a valid [`HalfwidthJaeum`]-equivalent in Unicode.
    NoUnicodeHalfwidthJaeumTryFromChoseong(Choseong),
    /// The [`Jaeum`] given does not have a valid [`HalfwidthJaeum`]-equivalent in Unicode.
    NoUnicodeHalfwidthJaeumTryFromJaeum(Jaeum),
    /// The [`Jongseong`] given does not have a valid [`HalfwidthJaeum`]-equivalent in Unicode.
    NoUnicodeHalfwidthJaeumTryFromJongseong(Jongseong),
    /// The [`Jungseong`] given does not have a valid [`HalfwidthMoeum`](crate::vowel::HalfwidthMoeum)-equivalent in Unicode.
    NoUnicodeHalfwidthMoeumTryFromJungseong(Jungseong),
    /// The [`Moeum`] given does not have a valid [`HalfwidthMoeum`](crate::vowel::HalfwidthMoeum)-equivalent in Unicode.
    NoUnicodeHalfwidthMoeumTryFromMoeum(Moeum),
    /// The [`Choseong`] given does not have a valid [`Jaeum`]-equivalent in Unicode.
    NoUnicodeJaeumTryFromChoseong(Choseong),
    /// The [`Jongseong`] given does not have a valid [`Jaeum`]-equivalent in Unicode.
    NoUnicodeJaeumTryFromJongseong(Jongseong),
    /// The [`Jungseong`] given does not have a valid [`Moeum`]-equivalent in Unicode.
    NoUnicodeMoeumTryFromJungseong(Jungseong),
}
impl Debug for Error {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            Self::NonChoseongTryFromChar(value) => {
                write!(
                    f,
                    "{0:?} (U+{1:X}, '{0}') is not valid as initial consonant",
                    value, *value as u32
                )
            }
            Self::NonChoseongTryFromHalfwidthJaeum(value) => {
                write!(
                    f,
                    "{0:?} (U+{1:X}, '{0}') is not valid as initial consonant",
                    value, *value as u32
                )
            }
            Self::NonChoseongTryFromJaeum(value) => {
                write!(
                    f,
                    "{0:?} (U+{1:X}, '{0}') is not valid as initial consonant",
                    value, *value as u32
                )
            }
            Self::NonChoseongTryFromJongseong(value) => {
                write!(
                    f,
                    "{0:?} (U+{1:X}, '{0}') is not valid as initial consonant",
                    value, *value as u32
                )
            }
            Self::NonHalfwidthJaeumTryFromChar(value) => {
                write!(
                    f,
                    "{0:?} (U+{1:X}, '{0}') is not a valid halfwidth consonant",
                    value, *value as u32
                )
            }
            Self::NonHalfwidthMoeumTryFromChar(value) => {
                write!(
                    f,
                    "{0:?} (U+{1:X}, '{0}') is not a valid halfwidth vowel",
                    value, *value as u32
                )
            }
            Self::NonJaeumTryFromChar(value) => {
                write!(
                    f,
                    "{0:?} (U+{1:X}, '{0}') is not a valid consonant",
                    value, *value as u32
                )
            }
            Self::NonJongseongTryFromChar(value) => {
                write!(
                    f,
                    "{0:?} (U+{1:X}, '{0}') is not valid as final consonant",
                    value, *value as u32
                )
            }
            Self::NonJongseongTryFromChoseong(value) => {
                write!(
                    f,
                    "{0:?} (U+{1:X}, '{0}') is not valid as final consonant",
                    value, *value as u32
                )
            }
            Self::NonJongseongTryFromHalfwidthJaeum(value) => {
                write!(
                    f,
                    "{0:?} (U+{1:X}, '{0}') is not valid as final consonant",
                    value, *value as u32
                )
            }
            Self::NonJongseongTryFromJaeum(value) => {
                write!(
                    f,
                    "{0:?} (U+{1:X}, '{0}') is not valid as final consonant",
                    value, *value as u32
                )
            }
            Self::NonJungseongTryFromChar(value) => {
                write!(
                    f,
                    "{0:?} (U+{1:X}, '{0}') is not valid as medial vowel",
                    value, *value as u32
                )
            }
            Self::NonKoreanTryFromChar(value) => {
                write!(
                    f,
                    "{0:?} (U+{1:X}, '{0}') is not a valid Korean syllable/alphabet/letter",
                    value, *value as u32
                )
            }
            Self::NonMoeumTryFromChar(value) => {
                write!(
                    f,
                    "{0:?} (U+{1:X}, '{0}') is not a valid vowel",
                    value, *value as u32
                )
            }
            Self::NoUnicodeHalfwidthJaeumTryFromChoseong(value) => {
                write!(f, "{0:?} (U+{1:X}, '{0}') does not have valid Halfwidth Jaeum--equivalent in Unicode", value, *value as u32)
            }
            Self::NoUnicodeHalfwidthJaeumTryFromJaeum(value) => {
                write!(f, "{0:?} (U+{1:X}, '{0}') does not have valid Halfwidth Jaeum--equivalent in Unicode", value, *value as u32)
            }
            Self::NoUnicodeHalfwidthJaeumTryFromJongseong(value) => {
                write!(f, "{0:?} (U+{1:X}, '{0}') does not have valid Halfwidth Jaeum--equivalent in Unicode", value, *value as u32)
            }
            Self::NoUnicodeHalfwidthMoeumTryFromJungseong(value) => {
                write!(f, "{0:?} (U+{1:X}, '{0}') does not have valid Halfwidth Moeum--equivalent in Unicode", value, *value as u32)
            }
            Self::NoUnicodeHalfwidthMoeumTryFromMoeum(value) => {
                write!(f, "{0:?} (U+{1:X}, '{0}') does not have valid Halfwidth Moeum--equivalent in Unicode", value, *value as u32)
            }
            Self::NoUnicodeJaeumTryFromChoseong(value) => {
                write!(
                    f,
                    "{0:?} (U+{1:X}, '{0}') does not have valid Jaeum-equivalent in Unicode",
                    value, *value as u32
                )
            }
            Self::NoUnicodeJaeumTryFromJongseong(value) => {
                write!(
                    f,
                    "{0:?} (U+{1:X}, '{0}') does not have valid Jaeum-equivalent in Unicode",
                    value, *value as u32
                )
            }
            Self::NoUnicodeMoeumTryFromJungseong(value) => {
                write!(
                    f,
                    "{0:?} (U+{1:X}, '{0}') does not have valid Moeum-equivalent in Unicode",
                    value, *value as u32
                )
            }
        }
    }
}
impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            Self::NonChoseongTryFromChar(value) => {
                write!(f, "'{}' is not valid as initial consonant", value)
            }
            Self::NonChoseongTryFromHalfwidthJaeum(value) => {
                write!(f, "'{}' is not valid as initial consonant", value)
            }
            Self::NonChoseongTryFromJaeum(value) => {
                write!(f, "'{}' is not valid as initial consonant", value)
            }
            Self::NonChoseongTryFromJongseong(value) => {
                write!(f, "'{}' is not valid as initial consonant", value)
            }
            Self::NonHalfwidthJaeumTryFromChar(value) => {
                write!(f, "'{}' is not a valid halfwidth consonant", value)
            }
            Self::NonHalfwidthMoeumTryFromChar(value) => {
                write!(f, "'{}' is not a valid halfwidth vowel", value)
            }
            Self::NonJaeumTryFromChar(value) => {
                write!(f, "'{}' is not a valid consonant", value)
            }
            Self::NonJongseongTryFromChar(value) => {
                write!(f, "'{}' is not valid as final consonant", value)
            }
            Self::NonJongseongTryFromChoseong(value) => {
                write!(f, "'{}' is not valid as final consonant", value)
            }
            Self::NonJongseongTryFromHalfwidthJaeum(value) => {
                write!(f, "'{}' is not valid as final consonant", value)
            }
            Self::NonJongseongTryFromJaeum(value) => {
                write!(f, "'{}' is not valid as final consonant", value)
            }
            Self::NonJungseongTryFromChar(value) => {
                write!(f, "'{}' is not valid as medial vowel", value)
            }
            Self::NonKoreanTryFromChar(value) => {
                write!(
                    f,
                    "'{}' is not a valid Korean syllable/alphabet/letter",
                    value
                )
            }
            Self::NonMoeumTryFromChar(value) => {
                write!(f, "'{}' is not a valid vowel", value)
            }
            Self::NoUnicodeHalfwidthJaeumTryFromChoseong(value) => {
                write!(
                    f,
                    "'{}' does not have valid Halfwidth Jaeum--equivalent in Unicode",
                    value
                )
            }
            Self::NoUnicodeHalfwidthJaeumTryFromJaeum(value) => {
                write!(
                    f,
                    "'{}' does not have valid Halfwidth Jaeum--equivalent in Unicode",
                    value
                )
            }
            Self::NoUnicodeHalfwidthJaeumTryFromJongseong(value) => {
                write!(
                    f,
                    "'{}' does not have valid Halfwidth Jaeum--equivalent in Unicode",
                    value
                )
            }
            Self::NoUnicodeHalfwidthMoeumTryFromJungseong(value) => {
                write!(
                    f,
                    "'{}' does not have valid Halfwidth Moeum--equivalent in Unicode",
                    value
                )
            }
            Self::NoUnicodeHalfwidthMoeumTryFromMoeum(value) => {
                write!(
                    f,
                    "'{}' does not have valid Halfwidth Moeum--equivalent in Unicode",
                    value
                )
            }
            Self::NoUnicodeJaeumTryFromChoseong(value) => {
                write!(
                    f,
                    "'{}' does not have valid Jaeum-equivalent in Unicode",
                    value
                )
            }
            Self::NoUnicodeJaeumTryFromJongseong(value) => {
                write!(
                    f,
                    "'{}' does not have valid Jaeum-equivalent in Unicode",
                    value
                )
            }
            Self::NoUnicodeMoeumTryFromJungseong(value) => {
                write!(
                    f,
                    "'{}' does not have valid Moeum-equivalent in Unicode",
                    value
                )
            }
        }
    }
}
impl StdError for Error {}
