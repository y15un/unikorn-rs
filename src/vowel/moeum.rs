use crate::{
    vowel::{HalfwidthMoeum, Jungseong},
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
pub enum Moeum {
    /// Represents vowel `ㅏ` (U+314F, Hangul Letter A)
    A = 0x314F,
    /// Represents vowel `ㅐ` (U+3150, Hangul Letter Ae)
    Ae,
    /// Represents vowel `ㅑ` (U+3151, Hangul Letter Ya)
    Ya,
    /// Represents vowel `ㅒ` (U+3152, Hangul Letter Yae)
    Yae,
    /// Represents vowel `ㅓ` (U+3153, Hangul Letter Eo)
    Eo,
    /// Represents vowel `ㅔ` (U+3154, Hangul Letter E)
    E,
    /// Represents vowel `ㅕ` (U+3155, Hangul Letter Yeo)
    Yeo,
    /// Represents vowel `ㅖ` (U+3156, Hangul Letter Ye)
    Ye,
    /// Represents vowel `ㅗ` (U+3157, Hangul Letter O)
    O,
    /// Represents vowel `ㅘ` (U+3158, Hangul Letter Wa)
    Wa,
    /// Represents vowel `ㅙ` (U+3159, Hangul Letter Wae)
    Wae,
    /// Represents vowel `ㅚ` (U+315A, Hangul Letter Oe)
    Oe,
    /// Represents vowel `ㅛ` (U+315B, Hangul Letter Yo)
    Yo,
    /// Represents vowel `ㅜ` (U+315C, Hangul Letter U)
    U,
    /// Represents vowel `ㅝ` (U+315D, Hangul Letter Weo)
    Weo,
    /// Represents vowel `ㅞ` (U+315E, Hangul Letter We)
    We,
    /// Represents vowel `ㅟ` (U+315F, Hangul Letter Wi)
    Wi,
    /// Represents vowel `ㅠ` (U+3160, Hangul Letter Yu)
    Yu,
    /// Represents vowel `ㅡ` (U+3161, Hangul Letter Eu)
    Eu,
    /// Represents vowel `ㅢ` (U+3162, Hangul Letter Yi)
    Yi,
    /// Represents vowel `ㅣ` (U+3163, Hangul Letter I)
    I,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* vowel `ㆇ` (U+3187, Hangul Letter Yo-Ya)
    YoYa = 0x3187,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* vowel `ㆈ` (U+3188, Hangul Letter Yo-Yae)
    YoYae,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* vowel `ㆉ` (U+3189, Hangul Letter Yo-I)
    YoI,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* vowel `ㆊ` (U+318A, Hangul Letter Yu-Yeo)
    YuYeo,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* vowel `ㆋ` (U+318B, Hangul Letter Yu-Ye)
    YuYe,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* vowel `ㆌ` (U+318C, Hangul Letter Yu-I)
    YuI,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* vowel `ㆍ` (U+318D, Hangul Letter AraeA)
    AraeA,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* vowel `ㆎ` (U+318E, Hangul Letter AraeAe)
    AraeAe,
}
impl Display for Moeum {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", char::from(*self))
    }
}
impl From<HalfwidthMoeum> for Moeum {
    fn from(value: HalfwidthMoeum) -> Self {
        match value {
            HalfwidthMoeum::A => Self::A,
            HalfwidthMoeum::Ae => Self::Ae,
            HalfwidthMoeum::Ya => Self::Ya,
            HalfwidthMoeum::Yae => Self::Yae,
            HalfwidthMoeum::Eo => Self::Eo,
            HalfwidthMoeum::E => Self::E,
            HalfwidthMoeum::Yeo => Self::Yeo,
            HalfwidthMoeum::Ye => Self::Ye,
            HalfwidthMoeum::O => Self::O,
            HalfwidthMoeum::Wa => Self::Wa,
            HalfwidthMoeum::Wae => Self::Wae,
            HalfwidthMoeum::Oe => Self::Oe,
            HalfwidthMoeum::Yo => Self::Yo,
            HalfwidthMoeum::U => Self::U,
            HalfwidthMoeum::Weo => Self::Weo,
            HalfwidthMoeum::We => Self::We,
            HalfwidthMoeum::Wi => Self::Wi,
            HalfwidthMoeum::Yu => Self::Yu,
            HalfwidthMoeum::Eu => Self::Eu,
            HalfwidthMoeum::Yi => Self::Yi,
            HalfwidthMoeum::I => Self::I,
        }
    }
}
impl From<Moeum> for char {
    fn from(value: Moeum) -> Self {
        // guaranteed to not fail within BMP
        char::from_u32(value as u32).unwrap()
    }
}
impl TryFrom<char> for Moeum {
    type Error = Error;

    /// Tries to convert a [`char`] into [`Moeum`].
    ///
    /// # Direct Conversion
    /// The [`char`] given will be tested against the following range(s):
    ///
    /// * Hangul Compatibility Jamo / Vowel letters (U+314F--U+3163)
    /// * Hangul Compatibility Jamo / Old vowel letters (U+3187--U+318E)
    ///
    /// The latter one range is considered valid if and only if `archaic-korean` feature is enabled.
    ///
    /// # Indirect Conversion
    /// ## Via [`Jungseong`]
    /// For the following range(s), the [`char`] given will be tried converting into [`Jungseong`] first, then subsequently into [`Moeum`]:
    ///
    /// * Hangul Jamo / Medial vowels (U+1161--U+1175)
    /// * Hangul Jamo / Old medial vowels (U+1176--U+11A7)
    /// * Hangul Jamo Extended-B / Old medial vowels (U+D7B0--U+D7C6)
    ///
    /// The latter two ranges are considered valid if and only if `archaic-korean` feature is enabled.
    ///
    /// ## Via [`HalfwidthMoeum`]
    /// For the following range(s), the [`char`] given will be tried converting into [`HalfwidthMoeum`] first, then subsequently into [`Moeum`]:
    ///
    /// * Halfwidth and Fullwidth Forms / Halfwidth Hangul variants (U+FFC2--U+FFC7, U+FFCA--U+FFCF, U+FFD2--U+FFD7, U+FFDA--U+FFDC)
    ///
    /// # Errors
    /// * [`Error::NonMoeumTryFromChar`]: the [`char`] given is not a valid vowel.
    /// * [`Error::NonKoreanTryFromChar`]: the [`char`] given is not a valid Korean letter.
    fn try_from(value: char) -> Result<Self, Self::Error> {
        let code_point = value as u32;
        let namo = Err(Error::NonMoeumTryFromChar(value));

        match code_point {
            // consonants
            0x1100..=0x1112 | 0x11A8..=0x11C2 | 0x3131..=0x314E | 0xFFA1..=0xFFBE => return namo,
            // medial vowels
            0x1161..=0x1175 => {
                return Self::try_from(Jungseong::try_from(code_point).unwrap()).or(namo)
            }
            // generic vowels (self)
            0x314F..=0x3163 => return Ok(Self::try_from(code_point).unwrap()),
            // halfwidth vowels
            0xFFC2..=0xFFC7 | 0xFFCA..=0xFFCF | 0xFFD2..=0xFFD7 | 0xFFDA..=0xFFDC => {
                return Ok(Self::from(HalfwidthMoeum::try_from(code_point).unwrap()))
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
            | 0xD7CB..=0xD7FB => return namo,
            // old medial vowels
            0x1176..=0x11A7 | 0xD7B0..=0xD7C6 => {
                return Self::try_from(Jungseong::try_from(code_point).unwrap()).or(namo)
            }
            // old generic vowels (self)
            0x3187..=0x318E => return Ok(Self::try_from(code_point).unwrap()),
            _ => {}
        }

        Err(Error::NonKoreanTryFromChar(value))
    }
}
impl TryFrom<Jungseong> for Moeum {
    #[cfg(not(feature = "archaic-korean"))]
    type Error = std::convert::Infallible;
    #[cfg(feature = "archaic-korean")]
    type Error = Error;

    /// Tries to convert a [`Jungseong`] into [`Moeum`].
    ///
    /// # Errors
    /// ## Without `archaic-korean` Feature
    /// This operation is guaranteed [`Infallible`].
    ///
    /// ## With `archaic-korean` Feature
    /// * [`Error::NoUnicodeMoeumTryFromJungseong`]: the [`Jungseong`] given does not have a valid [`Moeum`]-equivalent in Unicode.
    ///
    /// [`Infallible`]: std::convert::Infallible
    fn try_from(value: Jungseong) -> Result<Self, Self::Error> {
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
            Jungseong::YoYa => Ok(Self::YoYa),
            #[cfg(feature = "archaic-korean")]
            Jungseong::YoYae => Ok(Self::YoYae),
            #[cfg(feature = "archaic-korean")]
            Jungseong::YoI => Ok(Self::YoI),
            #[cfg(feature = "archaic-korean")]
            Jungseong::YuYeo => Ok(Self::YuYeo),
            #[cfg(feature = "archaic-korean")]
            Jungseong::YuYe => Ok(Self::YuYe),
            #[cfg(feature = "archaic-korean")]
            Jungseong::YuI => Ok(Self::YuI),
            #[cfg(feature = "archaic-korean")]
            Jungseong::AraeA => Ok(Self::AraeA),
            #[cfg(feature = "archaic-korean")]
            Jungseong::AraeAI => Ok(Self::AraeAe),
            #[cfg(feature = "archaic-korean")]
            _ => Err(Error::NoUnicodeMoeumTryFromJungseong(value)),
        }
    }
}
impl Moeum {
    // This list is only exported with `archaic-korean` feature, because without it the [`Moeum`] should be in order by itself.
    #[cfg(feature = "archaic-korean")]
    /// Lists [`Moeum`] in correct dictionary order.
    pub const IN_ORDER: [Self; 29] = [
        Self::A,
        Self::Ae,
        Self::Ya,
        Self::Yae,
        Self::Eo,
        Self::E,
        Self::Yeo,
        Self::Ye,
        Self::O,
        Self::Wa,
        Self::Wae,
        Self::Oe,
        Self::Yo,
        #[cfg(feature = "archaic-korean")]
        Self::YoYa,
        #[cfg(feature = "archaic-korean")]
        Self::YoYae,
        #[cfg(feature = "archaic-korean")]
        Self::YoI,
        Self::U,
        Self::Weo,
        Self::We,
        Self::Wi,
        Self::Yu,
        #[cfg(feature = "archaic-korean")]
        Self::YuYeo,
        #[cfg(feature = "archaic-korean")]
        Self::YuYe,
        #[cfg(feature = "archaic-korean")]
        Self::YuI,
        Self::Eu,
        Self::Yi,
        Self::I,
        #[cfg(feature = "archaic-korean")]
        Self::AraeA,
        #[cfg(feature = "archaic-korean")]
        Self::AraeAe,
    ];
}
