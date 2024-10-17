use crate::{
    vowel::{Jungseong, Moeum},
    Error,
};
use num_enum::{IntoPrimitive, TryFromPrimitive};
use std::{
    convert::TryFrom,
    fmt::{Display, Formatter, Result as FmtResult},
};

/// A complete set of vowels (모음, Moeum), but in halfwidth form.
#[derive(Clone, Copy, Debug, Eq, IntoPrimitive, Ord, PartialEq, PartialOrd, TryFromPrimitive)]
#[repr(u32)]
pub enum HalfwidthMoeum {
    /// Represents halfwidth vowel `ￂ` (U+FFC2, Halfwidth Hangul Letter A)
    A = 0xFFC2,
    /// Represents halfwidth vowel `ￃ` (U+FFC3, Halfwidth Hangul Letter Ae)
    Ae,
    /// Represents halfwidth vowel `ￄ` (U+FFC4, Halfwidth Hangul Letter Ya)
    Ya,
    /// Represents halfwidth vowel `ￅ` (U+FFC5, Halfwidth Hangul Letter Yae)
    Yae,
    /// Represents halfwidth vowel `ￆ` (U+FFC6, Halfwidth Hangul Letter Eo)
    Eo,
    /// Represents halfwidth vowel `ￇ` (U+FFC7, Halfwidth Hangul Letter E)
    E,
    /// Represents halfwidth vowel `ￊ` (U+FFCA, Halfwidth Hangul Letter Yeo)
    Yeo = 0xFFCA,
    /// Represents halfwidth vowel `ￋ` (U+FFCB, Halfwidth Hangul Letter Ye)
    Ye,
    /// Represents halfwidth vowel `ￌ` (U+FFCC, Halfwidth Hangul Letter O)
    O,
    /// Represents halfwidth vowel `ￍ` (U+FFCD, Halfwidth Hangul Letter Wa)
    Wa,
    /// Represents halfwidth vowel `ￎ` (U+FFCE, Halfwidth Hangul Letter Wae)
    Wae,
    /// Represents halfwidth vowel `ￏ` (U+FFCF, Halfwidth Hangul Letter Oe)
    Oe,
    /// Represents halfwidth vowel `ￒ` (U+FFD2, Halfwidth Hangul Letter Yo)
    Yo = 0xFFD2,
    /// Represents halfwidth vowel `ￓ` (U+FFD3, Halfwidth Hangul Letter U)
    U,
    /// Represents halfwidth vowel `ￔ` (U+FFD4, Halfwidth Hangul Letter Weo)
    Weo,
    /// Represents halfwidth vowel `ￕ` (U+FFD5, Halfwidth Hangul Letter We)
    We,
    /// Represents halfwidth vowel `ￖ` (U+FFD6, Halfwidth Hangul Letter Wi)
    Wi,
    /// Represents halfwidth vowel `ￗ` (U+FFD7, Halfwidth Hangul Letter Yu)
    Yu,
    /// Represents halfwidth vowel `ￚ` (U+FFDA, Halfwidth Hangul Letter Eu)
    Eu = 0xFFDA,
    /// Represents halfwidth vowel `ￛ` (U+FFDB, Halfwidth Hangul Letter Yi)
    Yi,
    /// Represents halfwidth vowel `ￜ` (U+FFDC, Halfwidth Hangul Letter I)
    I,
}
impl Display for HalfwidthMoeum {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", char::from(*self))
    }
}
impl From<HalfwidthMoeum> for char {
    fn from(value: HalfwidthMoeum) -> Self {
        // guaranteed to not fail within BMP
        char::from_u32(value as u32).unwrap()
    }
}
impl TryFrom<char> for HalfwidthMoeum {
    type Error = Error;

    /// Tries to convert a [`char`] into [`HalfwidthMoeum`].
    ///
    /// # Direct Conversion
    /// The [`char`] given will be tested against the following range(s):
    ///
    /// * Halfwidth and Fullwidth Forms / Halfwidth Hangul variants (U+FFC2--U+FFC7, U+FFCA--U+FFCF, U+FFD2--U+FFD7, U+FFDA--U+FFDC)
    ///
    /// # Indirect Conversion
    /// ## Via [`Jungseong`]
    /// For the following range(s), the [`char`] given will be tried converting into [`Jungseong`] first, then subsequently into [`HalfwidthMoeum`]:
    ///
    /// * Hangul Jamo / Medial vowels (U+1161--U+1175)
    /// * Hangul Jamo / Old medial vowels (U+1176--U+11A7)
    /// * Hangul Jamo Extended-B / Old medial vowels (U+D7B0--U+D7C6)
    ///
    /// The latter two ranges are considered valid if and only if `archaic-korean` feature is enabled.
    ///
    /// ## Via [`Moeum`]
    /// For the following range(s), the [`char`] given will be tried converting into [`Moeum`] first, then subsequently into [`HalfwidthMoeum`]:
    ///
    /// * Hangul Compatibility Jamo / Vowel letters (U+314F--U+3163)
    /// * Hangul Compatibility Jamo / Old vowel letters (U+3187--U+318E)
    ///
    /// The latter one range is considered valid if and only if `archaic-korean` feature is enabled.
    ///
    /// # Errors
    /// * [`Error::NonHalfwidthMoeumTryFromChar`]: the [`char`] given is not a valid halfwidth vowel.
    /// * [`Error::NonMoeumTryFromChar`]: the [`char`] given is not a valid vowel.
    /// * [`Error::NonKoreanTryFromChar`]: the [`char`] given is not a valid Korean letter.
    fn try_from(value: char) -> Result<Self, Self::Error> {
        let code_point = value as u32;
        let nahamo = Err(Error::NonHalfwidthMoeumTryFromChar(value));

        match code_point {
            // consonants
            0x1100..=0x1112 | 0x11A8..=0x11C2 | 0x3131..=0x314E | 0xFFA1..=0xFFBE => {
                return Err(Error::NonMoeumTryFromChar(value))
            }
            // medial vowels
            0x1161..=0x1175 => {
                return Self::try_from(Jungseong::try_from(code_point).unwrap()).or(nahamo)
            }
            // generic vowels
            0x314F..=0x3163 => {
                return Self::try_from(Moeum::try_from(code_point).unwrap()).or(nahamo)
            }
            // halfwidth vowels (self)
            0xFFC2..=0xFFC7 | 0xFFCA..=0xFFCF | 0xFFD2..=0xFFD7 | 0xFFDA..=0xFFDC => {
                return Ok(Self::try_from(code_point).unwrap())
            }
            _ => {}
        }

        #[cfg(feature = "archaic-korean")]
        match code_point {
            // old consonants
            0x1113..=0x115E
            | 0x11C3..=0x11FF
            | 0x3165..=0x3186
            | 0xA960..=0xA97C
            | 0xD7CB..=0xD7FB => return Err(Error::NonMoeumTryFromChar(value)),
            // old medial vowels
            0x1176..=0x11A7 | 0xD7B0..=0xD7C6 => {
                return Self::try_from(Jungseong::try_from(code_point).unwrap()).or(nahamo)
            }
            // old generic vowels
            0x3187..=0x318E => {
                return Self::try_from(Moeum::try_from(code_point).unwrap()).or(nahamo)
            }
            _ => {}
        }

        Err(Error::NonKoreanTryFromChar(value))
    }
}
impl TryFrom<Jungseong> for HalfwidthMoeum {
    #[cfg(not(feature = "archaic-korean"))]
    type Error = std::convert::Infallible;
    #[cfg(feature = "archaic-korean")]
    type Error = Error;

    /// Tries to convert a [`Jungseong`] into [`HalfwidthMoeum`].
    ///
    /// # Errors
    /// ## Without `archaic-korean` Feature
    /// This operation is guaranteed  [`Infallible`].
    ///
    /// ## With `archaic-korean` Feature
    /// * [`Error::NoUnicodeHalfwidthMoeumTryFromJungseong`]: the [`Jungseong`] given does not have its [`HalfwidthMoeum`]-equivalent in Unicode.
    ///
    /// [`Infallible`]: std::convert::Infallible
    fn try_from(value: Jungseong) -> Result<Self, Self::Error> {
        // TODO: consider switching to bst; but i'm not very sure of performance boost it'll yield.
        match value {
            Jungseong::A => Ok(Self::A),
            Jungseong::Ae => Ok(Self::Ae),
            Jungseong::Ya => Ok(Self::Ya),
            Jungseong::Yae => Ok(Self::Yae),
            Jungseong::Eo => Ok(Self::Eo),
            Jungseong::E => Ok(Self::E),
            Jungseong::Yeo => Ok(Self::Yeo),
            Jungseong::Ye => Ok(Self::Ye),
            Jungseong::O => Ok(Self::O),
            Jungseong::Wa => Ok(Self::Wa),
            Jungseong::Wae => Ok(Self::Wae),
            Jungseong::Oe => Ok(Self::Oe),
            Jungseong::Yo => Ok(Self::Yo),
            Jungseong::U => Ok(Self::U),
            Jungseong::Weo => Ok(Self::Weo),
            Jungseong::We => Ok(Self::We),
            Jungseong::Wi => Ok(Self::Wi),
            Jungseong::Yu => Ok(Self::Yu),
            Jungseong::Eu => Ok(Self::Eu),
            Jungseong::Yi => Ok(Self::Yi),
            Jungseong::I => Ok(Self::I),
            #[cfg(feature = "archaic-korean")]
            _ => Err(Error::NoUnicodeHalfwidthMoeumTryFromJungseong(value)),
        }
    }
}
impl TryFrom<Moeum> for HalfwidthMoeum {
    #[cfg(not(feature = "archaic-korean"))]
    type Error = std::convert::Infallible;
    #[cfg(feature = "archaic-korean")]
    type Error = Error;

    /// Tries to convert a [`Moeum`] into [`HalfwidthMoeum`].
    ///
    /// # Errors
    /// ## Without `archaic-korean` Feature
    /// This operation is guaranteed [`Infallible`].
    ///
    /// ## With `archaic-korean` Feature
    /// * [`Error::NoUnicodeHalfwidthMoeumTryFromMoeum`]: the [`Moeum`] given does not have a valid [`HalfwidthMoeum`]-equivalent in Unicode.
    ///
    /// [`Infallible`]: std::convert::Infallible
    fn try_from(value: Moeum) -> Result<Self, Self::Error> {
        // TODO: consider switching to bst; but i'm not very sure of performance boost it'll yield.
        match value {
            Moeum::A => Ok(Self::A),
            Moeum::Ae => Ok(Self::Ae),
            Moeum::Ya => Ok(Self::Ya),
            Moeum::Yae => Ok(Self::Yae),
            Moeum::Eo => Ok(Self::Eo),
            Moeum::E => Ok(Self::E),
            Moeum::Yeo => Ok(Self::Yeo),
            Moeum::Ye => Ok(Self::Ye),
            Moeum::O => Ok(Self::O),
            Moeum::Wa => Ok(Self::Wa),
            Moeum::Wae => Ok(Self::Wae),
            Moeum::Oe => Ok(Self::Oe),
            Moeum::Yo => Ok(Self::Yo),
            Moeum::U => Ok(Self::U),
            Moeum::Weo => Ok(Self::Weo),
            Moeum::We => Ok(Self::We),
            Moeum::Wi => Ok(Self::Wi),
            Moeum::Yu => Ok(Self::Yu),
            Moeum::Eu => Ok(Self::Eu),
            Moeum::Yi => Ok(Self::Yi),
            Moeum::I => Ok(Self::I),
            #[cfg(feature = "archaic-korean")]
            _ => Err(Error::NoUnicodeHalfwidthMoeumTryFromMoeum(value)),
        }
    }
}
