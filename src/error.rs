use crate::{
    consonant::{Choseong, HalfwidthJaeum, Jaeum, Jongseong},
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
    /// The `char` given is not a valid Korean syllable/alphabet/letter.
    NonKoreanTryFromChar(char),
    /// The [`Choseong`] given does not have a valid [`HalfwidthJaeum`]-equivalent in Unicode.
    NoUnicodeHalfwidthJaeumTryFromChoseong(Choseong),
    /// The [`Jaeum`] given does not have a valid [`HalfwidthJaeum`]-equivalent in Unicode.
    NoUnicodeHalfwidthJaeumTryFromJaeum(Jaeum),
    /// The [`Jongseong`] given does not have a valid [`HalfwidthJaeum`]-equivalent in Unicode.
    NoUnicodeHalfwidthJaeumTryFromJongseong(Jongseong),
    /// The [`Choseong`] given does not have a valid [`Jaeum`]-equivalent in Unicode.
    NoUnicodeJaeumTryFromChoseong(Choseong),
    /// The [`Jongseong`] given does not have a valid [`Jaeum`]-equivalent in Unicode.
    NoUnicodeJaeumTryFromJongseong(Jongseong),
}
impl Debug for Error {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            Self::NonChoseongTryFromChar(value) => {
                write!(
                    f,
                    "{:?} (U+{:X}, '{}') is not valid as initial consonant",
                    value, *value as u32, value
                )
            }
            Self::NonChoseongTryFromHalfwidthJaeum(value) => {
                write!(
                    f,
                    "{:?} (U+{:X}, '{}') is not valid as initial consonant",
                    value, *value as u32, value
                )
            }
            Self::NonChoseongTryFromJaeum(value) => {
                write!(
                    f,
                    "{:?} (U+{:X}, '{}') is not valid as initial consonant",
                    value, *value as u32, value
                )
            }
            Self::NonChoseongTryFromJongseong(value) => {
                write!(
                    f,
                    "{:?} (U+{:X}, '{}') is not valid as initial consonant",
                    value, *value as u32, value
                )
            }
            Self::NonHalfwidthJaeumTryFromChar(value) => {
                write!(
                    f,
                    "{:?} (U+{:X}, '{}') is not a valid halfwidth consonant",
                    value, *value as u32, value
                )
            }
            Self::NonJaeumTryFromChar(value) => {
                write!(
                    f,
                    "{:?} (U+{:X}, '{}') is not a valid consonant",
                    value, *value as u32, value
                )
            }
            Self::NonJongseongTryFromChar(value) => {
                write!(
                    f,
                    "{:?} (U+{:X}, '{}') is not valid as final consonant",
                    value, *value as u32, value
                )
            }
            Self::NonJongseongTryFromChoseong(value) => {
                write!(
                    f,
                    "{:?} (U+{:X}, '{}') is not valid as final consonant",
                    value, *value as u32, value
                )
            }
            Self::NonJongseongTryFromHalfwidthJaeum(value) => {
                write!(
                    f,
                    "{:?} (U+{:X}, '{}') is not valid as final consonant",
                    value, *value as u32, value
                )
            }
            Self::NonJongseongTryFromJaeum(value) => {
                write!(
                    f,
                    "{:?} (U+{:X}, '{}') is not valid as final consonant",
                    value, *value as u32, value
                )
            }
            Self::NonKoreanTryFromChar(value) => {
                write!(
                    f,
                    "{:?} (U+{:X}, '{}') is not a valid Korean syllable/alphabet/letter",
                    value, *value as u32, value
                )
            }
            Self::NoUnicodeHalfwidthJaeumTryFromChoseong(value) => {
                write!(f, "{:?} (U+{:X}, '{}') does not have valid Halfwidth Jaeum--equivalent in Unicode", value, *value as u32, value)
            }
            Self::NoUnicodeHalfwidthJaeumTryFromJaeum(value) => {
                write!(f, "{:?} (U+{:X}, '{}') does not have valid Halfwidth Jaeum--equivalent in Unicode", value, *value as u32, value)
            }
            Self::NoUnicodeHalfwidthJaeumTryFromJongseong(value) => {
                write!(f, "{:?} (U+{:X}, '{}') does not have valid Halfwidth Jaeum--equivalent in Unicode", value, *value as u32, value)
            }
            Self::NoUnicodeJaeumTryFromChoseong(value) => {
                write!(
                    f,
                    "{:?} (U+{:X}, '{}') does not have valid Jaeum-equivalent in Unicode",
                    value, *value as u32, value
                )
            }
            Self::NoUnicodeJaeumTryFromJongseong(value) => {
                write!(
                    f,
                    "{:?} (U+{:X}, '{}') does not have valid Jaeum-equivalent in Unicode",
                    value, *value as u32, value
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
                write!(f, "'{}' is not a valid consonant (halfwidth)", value)
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
            Self::NonKoreanTryFromChar(value) => {
                write!(
                    f,
                    "'{}' is not a valid Korean syllable/alphabet/letter",
                    value
                )
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
        }
    }
}
impl StdError for Error {}
