use crate::consonant::{Choseong, Jaeum, Jongseong};
use std::{
    error::Error as StdError,
    fmt::{Display, Formatter, Result as FmtResult},
};

/// Groups all the possible erroneous circumstances within this library.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Error {
    /// The [`char`] given is not a valid initial consonant.
    NonChoseongTryFromChar(char),
    /// The [`Jaeum`] given is not a valid initial consonant.
    NonChoseongTryFromJaeum(Jaeum),
    /// The `char` given is not a valid consonant.
    NonJaeumTryFromChar(char),
    /// The `char` given is not a valid final consonant.
    NonJongseongTryFromChar(char),
    /// The [`Jaeum`] given is not a valid final consonant.
    NonJongseongTryFromJaeum(Jaeum),
    /// The `char` given is not a valid Korean alphabet.
    NonKoreanTryFromChar(char),
    /// The [`Choseong`] given does not have a valid `Jaeum`-equivalent in Unicode.
    NonUnicodeJaeumTryFromChoseong(Choseong),
    /// The [`Jongseong`] given does not have a valid `Jaeum`-equivalent in Unicode.
    NonUnicodeJaeumTryFromJongseong(Jongseong),
}
impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            Self::NonChoseongTryFromChar(value) => {
                write!(f, "char {:?} is not a valid initial consonant", value)
            }
            Self::NonChoseongTryFromJaeum(value) => {
                write!(f, "{:?} is not a valid initial consonant", value)
            }
            Self::NonJaeumTryFromChar(value) => {
                write!(f, "char {:?} is not a valid consonant", value)
            }
            Self::NonJongseongTryFromChar(value) => {
                write!(f, "char {:?} is not a valid final consonant", value)
            }
            Self::NonJongseongTryFromJaeum(value) => {
                write!(f, "{:?} is not a valid final consonant", value)
            }
            Self::NonKoreanTryFromChar(value) => {
                write!(f, "char {:?} is not a valid Korean alphabet", value)
            }
            Self::NonUnicodeJaeumTryFromChoseong(value) => {
                write!(
                    f,
                    "{:?} does not have a valib Jaeum-equivalent in Unicode",
                    value
                )
            }
            Self::NonUnicodeJaeumTryFromJongseong(value) => {
                write!(
                    f,
                    "{:?} does not have a valib Jaeum-equivalent in Unicode",
                    value
                )
            }
        }
    }
}
impl StdError for Error {}
