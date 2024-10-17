use crate::{
    vowel::{HalfwidthMoeum, Moeum},
    Error,
};
use num_enum::{IntoPrimitive, TryFromPrimitive};
use std::{
    convert::TryFrom,
    fmt::{Display, Formatter, Result as FmtResult},
};

/// A set of vowels valid as medial vowel (중성, Jungseong).
#[derive(Clone, Copy, Debug, Eq, IntoPrimitive, Ord, PartialEq, PartialOrd, TryFromPrimitive)]
#[repr(u32)]
pub enum Jungseong {
    /// Represents median vowel `ᅡ` (U+1161, Hangul Jungseong A)
    A = 0x1161,
    /// Represents median vowel `ᅢ` (U+1162, Hangul Jungseong Ae)
    Ae,
    /// Represents median vowel `ᅣ` (U+1163, Hangul Jungseong Ya)
    Ya,
    /// Represents median vowel `ᅤ` (U+1164, Hangul Jungseong Yae)
    Yae,
    /// Represents median vowel `ᅥ` (U+1165, Hangul Jungseong Eo)
    Eo,
    /// Represents median vowel `ᅦ` (U+1166, Hangul Jungseong E)
    E,
    /// Represents median vowel `ᅧ` (U+1167, Hangul Jungseong Yeo)
    Yeo,
    /// Represents median vowel `ᅨ` (U+1168, Hangul Jungseong Ye)
    Ye,
    /// Represents median vowel `ᅩ` (U+1169, Hangul Jungseong O)
    O,
    /// Represents median vowel `ᅪ` (U+116A, Hangul Jungseong Wa)
    Wa,
    /// Represents median vowel `ᅫ` (U+116B, Hangul Jungseong Wae)
    Wae,
    /// Represents median vowel `ᅬ` (U+116C, Hangul Jungseong Oe)
    Oe,
    /// Represents median vowel `ᅭ` (U+116D, Hangul Jungseong Yo)
    Yo,
    /// Represents median vowel `ᅮ` (U+116E, Hangul Jungseong U)
    U,
    /// Represents median vowel `ᅯ` (U+116F, Hangul Jungseong Weo)
    Weo,
    /// Represents median vowel `ᅰ` (U+1170, Hangul Jungseong We)
    We,
    /// Represents median vowel `ᅱ` (U+1171, Hangul Jungseong Wi)
    Wi,
    /// Represents median vowel `ᅲ` (U+1172, Hangul Jungseong Yu)
    Yu,
    /// Represents median vowel `ᅳ` (U+1173, Hangul Jungseong Eu)
    Eu,
    /// Represents median vowel `ᅴ` (U+1174, Hangul Jungseong Yi)
    Yi,
    /// Represents median vowel `ᅵ` (U+1175, Hangul Jungseong I)
    I,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* median vowel `ᅶ` (U+1176, Hangul Jungseong A-O)
    AO,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* median vowel `ᅷ` (U+1177, Hangul Jungseong A-U)
    AU,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* median vowel `ᅸ` (U+1178, Hangul Jungseong Ya-O)
    YaO,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* median vowel `ᅹ` (U+1179, Hangul Jungseong Ya-Yo)
    YaYo,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* median vowel `ᅺ` (U+117A, Hangul Jungseong Eo-O)
    EoO,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* median vowel `ᅻ` (U+117B, Hangul Jungseong Eo-U)
    EoU,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* median vowel `ᅼ` (U+117C, Hangul Jungseong Eo-Eu)
    EoEu,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* median vowel `ᅽ` (U+117D, Hangul Jungseong Yeo-O)
    YeoO,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* median vowel `ᅾ` (U+117E, Hangul Jungseong Yeo-U)
    YeoU,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* median vowel `ᅿ` (U+117F, Hangul Jungseong O-Eo)
    OEo,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* median vowel `ᆀ` (U+1180, Hangul Jungseong O-E)
    OE,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* median vowel `ᆁ` (U+1181, Hangul Jungseong O-Ye)
    OYe,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* median vowel `ᆂ` (U+1182, Hangul Jungseong O-O)
    OO,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* median vowel `ᆃ` (U+1183, Hangul Jungseong O-U)
    OU,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* median vowel `ᆄ` (U+1184, Hangul Jungseong Yo-Ya)
    YoYa,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* median vowel `ᆅ` (U+1185, Hangul Jungseong Yo-Yae)
    YoYae,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* median vowel `ᆆ` (U+1186, Hangul Jungseong Yo-Yeo)
    YoYeo,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* median vowel `ᆇ` (U+1187, Hangul Jungseong Yo-O)
    YoO,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* median vowel `ᆈ` (U+1188, Hangul Jungseong Yo-I)
    YoI,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* median vowel `ᆉ` (U+1189, Hangul Jungseong U-A)
    UA,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* median vowel `ᆊ` (U+118A, Hangul Jungseong U-Ae)
    UAe,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* median vowel `ᆋ` (U+118B, Hangul Jungseong U-Eo-Eu)
    UEoEu,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* median vowel `ᆌ` (U+118C, Hangul Jungseong U-Ye)
    UYe,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* median vowel `ᆍ` (U+118D, Hangul Jungseong U-U)
    UU,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* median vowel `ᆎ` (U+118E, Hangul Jungseong Yu-A)
    YuA,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* median vowel `ᆏ` (U+118F, Hangul Jungseong Yu-Eo)
    YuEo,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* median vowel `ᆐ` (U+1190, Hangul Jungseong Yu-E)
    YuE,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* median vowel `ᆑ` (U+1191, Hangul Jungseong Yu-Yeo)
    YuYeo,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* median vowel `ᆒ` (U+1192, Hangul Jungseong Yu-Ye)
    YuYe,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* median vowel `ᆓ` (U+1193, Hangul Jungseong Yu-U)
    YuU,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* median vowel `ᆔ` (U+1194, Hangul Jungseong Yu-I)
    YuI,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* median vowel `ᆕ` (U+1195, Hangul Jungseong Eu-U)
    EuU,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* median vowel `ᆖ` (U+1196, Hangul Jungseong Eu-Eu)
    EuEu,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* median vowel `ᆗ` (U+1197, Hangul Jungseong Yi-U)
    YiU,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* median vowel `ᆘ` (U+1198, Hangul Jungseong I-A)
    IA,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* median vowel `ᆙ` (U+1199, Hangul Jungseong I-Ya)
    IYa,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* median vowel `ᆚ` (U+119A, Hangul Jungseong I-O)
    IO,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* median vowel `ᆛ` (U+119B, Hangul Jungseong I-U)
    IU,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* median vowel `ᆜ` (U+119C, Hangul Jungseong I-Eu)
    IEu,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* median vowel `ᆝ` (U+119D, Hangul Jungseong I-AraeA)
    IAraeA,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* median vowel `ᆞ` (U+119E, Hangul Jungseong AraeA)
    AraeA,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* median vowel `ᆟ` (U+119F, Hangul Jungseong AraeA-Eo)
    AraeAEo,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* median vowel `ᆠ` (U+11A0, Hangul Jungseong AraeA-U)
    AraeAU,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* median vowel `ᆡ` (U+11A1, Hangul Jungseong AraeA-I)
    AraeAI,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* median vowel `ᆢ` (U+11A2, Hangul Jungseong SsangAraeA)
    SsangAraeA,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* median vowel `ᆣ` (U+11A3, Hangul Jungseong A-Eu)
    AEu,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* median vowel `ᆤ` (U+11A4, Hangul Jungseong YA-U)
    YaU,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* median vowel `ᆥ` (U+11A5, Hangul Jungseong Yeo-Ya)
    YeoYa,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* median vowel `ᆦ` (U+11A6, Hangul Jungseong O-Ya)
    OYa,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* median vowel `ᆧ` (U+11A7, Hangul Jungseong O-Yae)
    OYae,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* median vowel `ힰ` (U+D7B0, Hangul Jungseong O-Yeo)
    OYeo = 0xD7B0,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* median vowel `ힱ` (U+D7B1, Hangul Jungseong O-O-I)
    OOI,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* median vowel `ힲ` (U+D7B2, Hangul Jungseong Yo-A)
    YoA,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* median vowel `ힳ` (U+D7B3, Hangul Jungseong Yo-Ae)
    YoAe,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* median vowel `ힴ` (U+D7B4, Hangul Jungseong Yo-Eo)
    YoEo,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* median vowel `ힵ` (U+D7B5, Hangul Jungseong U-Yeo)
    UYeo,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* median vowel `ힶ` (U+D7B6, Hangul Jungseong U-I-I)
    UII,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* median vowel `ힷ` (U+D7B7, Hangul Jungseong Yu-Ae)
    YuAe,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* median vowel `ힸ` (U+D7B8, Hangul Jungseong Yu-O)
    YuO,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* median vowel `ힹ` (U+D7B9, Hangul Jungseong Eu-A)
    EuA,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* median vowel `ힺ` (U+D7BA, Hangul Jungseong Eu-Eo)
    EuEo,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* median vowel `ힻ` (U+D7BB, Hangul Jungseong Eu-E)
    EuE,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* median vowel `ힼ` (U+D7BC, Hangul Jungseong Eu-O)
    EuO,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* median vowel `ힽ` (U+D7BD, Hangul Jungseong I-Ya-O)
    IYaO,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* median vowel `ힾ` (U+D7BE, Hangul Jungseong I-Yae)
    IYae,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* median vowel `ힿ` (U+D7BF, Hangul Jungseong I-Yeo)
    IYeo,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* median vowel `ퟀ` (U+D7C0, Hangul Jungseong I-Ye)
    IYe,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* median vowel `ퟁ` (U+D7C1, Hangul Jungseong I-O-I)
    IOI,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* median vowel `ퟂ` (U+D7C2, Hangul Jungseong I-Yo)
    IYo,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* median vowel `ퟃ` (U+D7C3, Hangul Jungseong I-Yu)
    IYu,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* median vowel `ퟄ` (U+D7C4, Hangul Jungseong I-I)
    II,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* median vowel `ퟅ` (U+D7C5, Hangul Jungseong AraeA-A)
    AraeAA,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* median vowel `ퟆ` (U+D7C6, Hangul Jungseong AraeA-E)
    AraeAE,
}
impl Display for Jungseong {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", char::from(*self))
    }
}
impl From<HalfwidthMoeum> for Jungseong {
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
impl From<Jungseong> for char {
    fn from(value: Jungseong) -> Self {
        // guaranteed to not fail within BMP
        char::from_u32(value as u32).unwrap()
    }
}
impl From<Moeum> for Jungseong {
    fn from(value: Moeum) -> Self {
        match value {
            Moeum::A => Self::A,
            Moeum::Ae => Self::Ae,
            Moeum::Ya => Self::Ya,
            Moeum::Yae => Self::Yae,
            Moeum::Eo => Self::Eo,
            Moeum::E => Self::E,
            Moeum::Yeo => Self::Yeo,
            Moeum::Ye => Self::Ye,
            Moeum::O => Self::O,
            Moeum::Wa => Self::Wa,
            Moeum::Wae => Self::Wae,
            Moeum::Oe => Self::Oe,
            Moeum::Yo => Self::Yo,
            Moeum::U => Self::U,
            Moeum::Weo => Self::Weo,
            Moeum::We => Self::We,
            Moeum::Wi => Self::Wi,
            Moeum::Yu => Self::Yu,
            Moeum::Eu => Self::Eu,
            Moeum::Yi => Self::Yi,
            Moeum::I => Self::I,
            #[cfg(feature = "archaic-korean")]
            Moeum::YoYa => Self::YoYa,
            #[cfg(feature = "archaic-korean")]
            Moeum::YoYae => Self::YoYae,
            #[cfg(feature = "archaic-korean")]
            Moeum::YoI => Self::YoI,
            #[cfg(feature = "archaic-korean")]
            Moeum::YuYeo => Self::YuYeo,
            #[cfg(feature = "archaic-korean")]
            Moeum::YuYe => Self::YuYe,
            #[cfg(feature = "archaic-korean")]
            Moeum::YuI => Self::YuI,
            #[cfg(feature = "archaic-korean")]
            Moeum::AraeA => Self::AraeA,
            #[cfg(feature = "archaic-korean")]
            Moeum::AraeAe => Self::AraeAI,
        }
    }
}
impl TryFrom<char> for Jungseong {
    type Error = Error;

    /// Tries to convert a [`char`] into [`Jungseong`].
    ///
    /// # Direct Conversion
    /// The [`char`] given will be tested against the following range(s):
    ///
    /// * Hangul Jamo / Medial vowels (U+1161--U+1175)
    /// * Hangul Jamo / Old medial vowels (U+1176--U+11A7)
    /// * Hangul Jamo Extended-B / Old medial vowels (U+D7B0--U+D7C6)
    ///
    /// The latter two ranges are considered valid if and only if `archaic-korean` feature is enabled.
    ///
    /// # Indirect Conversion
    /// ## Via [`Moeum`] or [`HalfwidthMoeum`]
    /// For the following range(s), the [`char`] given will be tried converting into [`Moeum`] or [`HalfwidthMoeum`] accordingly first, then subsequently into [`Jungseong`]:
    ///
    /// * Hangul Compatibility Jamo / Vowel letters (U+314F--U+3163)
    /// * Halfwidth and Fullwidth Forms / Halfwidth Hangul variants (U+FFC2--U+FFC7, U+FFCA--U+FFCF, U+FFD2--U+FFD7, U+FFDA--U+FFDC)
    /// * Hangul Compatibility Jamo / Old vowel letters (U+3187--U+318E)
    ///
    /// The latter one range is considered valid if and only if `archaic-korean` feature is enabled.
    ///
    /// # Errors
    /// * [`Error::NonJungseongTryFromChar`]: the [`char`] given is not valid as initial consonant.
    /// * [`Error::NonMoeumTryFromChar`]: the [`char`] given is not a valid consonant.
    /// * [`Error::NonKoreanTryFromChar`]: the [`char`] given is not a valid Korean letter.
    fn try_from(value: char) -> Result<Self, Self::Error> {
        let code_point = value as u32;

        match code_point {
            // consonants
            0x1100..=0x1112 | 0x11A8..=0x11C2 | 0x3131..=0x314E | 0xFFA1..=0xFFBE => {
                return Err(Error::NonMoeumTryFromChar(value))
            }
            // medial vowels (self)
            0x1161..=0x1175 => return Ok(Self::try_from(code_point).unwrap()),
            // generic vowels
            0x314F..=0x3163 => return Ok(Self::from(Moeum::try_from(code_point).unwrap())),
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
            | 0xD7CB..=0xD7FB => return Err(Error::NonMoeumTryFromChar(value)),
            // old medial vowels (self)
            0x1176..=0x11A7 | 0xD7B0..=0xD7C6 => return Ok(Self::try_from(code_point).unwrap()),
            // old generic vowels
            0x3187..=0x318E => return Ok(Self::from(Moeum::try_from(code_point).unwrap())),
            _ => {}
        }

        Err(Error::NonKoreanTryFromChar(value))
    }
}
impl Jungseong {
    // This list is only exported with `archaic-korean` feature, because without it the [`Jungseong`] should be in order by itself.
    #[cfg(feature = "archaic-korean")]
    /// Lists [`Jungseong`] in correct dictionary order.
    pub const IN_ORDER: [Self; 94] = [
        Self::A,
        #[cfg(feature = "archaic-korean")]
        Self::AO,
        #[cfg(feature = "archaic-korean")]
        Self::AU,
        #[cfg(feature = "archaic-korean")]
        Self::AEu,
        Self::Ae,
        Self::Ya,
        #[cfg(feature = "archaic-korean")]
        Self::YaO,
        #[cfg(feature = "archaic-korean")]
        Self::YaYo,
        #[cfg(feature = "archaic-korean")]
        Self::YaU,
        Self::Yae,
        Self::Eo,
        #[cfg(feature = "archaic-korean")]
        Self::EoO,
        #[cfg(feature = "archaic-korean")]
        Self::EoU,
        #[cfg(feature = "archaic-korean")]
        Self::EoEu,
        Self::E,
        Self::Yeo,
        #[cfg(feature = "archaic-korean")]
        Self::YeoYa,
        #[cfg(feature = "archaic-korean")]
        Self::YeoO,
        #[cfg(feature = "archaic-korean")]
        Self::YeoU,
        Self::Ye,
        Self::O,
        Self::Wa,
        Self::Wae,
        #[cfg(feature = "archaic-korean")]
        Self::OYa,
        #[cfg(feature = "archaic-korean")]
        Self::OYae,
        #[cfg(feature = "archaic-korean")]
        Self::OEo,
        #[cfg(feature = "archaic-korean")]
        Self::OE,
        #[cfg(feature = "archaic-korean")]
        Self::OYeo,
        #[cfg(feature = "archaic-korean")]
        Self::OYe,
        #[cfg(feature = "archaic-korean")]
        Self::OO,
        #[cfg(feature = "archaic-korean")]
        Self::OOI,
        #[cfg(feature = "archaic-korean")]
        Self::OU,
        Self::Oe,
        Self::Yo,
        #[cfg(feature = "archaic-korean")]
        Self::YoA,
        #[cfg(feature = "archaic-korean")]
        Self::YoAe,
        #[cfg(feature = "archaic-korean")]
        Self::YoYa,
        #[cfg(feature = "archaic-korean")]
        Self::YoYae,
        #[cfg(feature = "archaic-korean")]
        Self::YoEo,
        #[cfg(feature = "archaic-korean")]
        Self::YoYeo,
        #[cfg(feature = "archaic-korean")]
        Self::YoO,
        #[cfg(feature = "archaic-korean")]
        Self::YoI,
        Self::U,
        #[cfg(feature = "archaic-korean")]
        Self::UA,
        #[cfg(feature = "archaic-korean")]
        Self::UAe,
        Self::Weo,
        #[cfg(feature = "archaic-korean")]
        Self::UEoEu,
        Self::We,
        #[cfg(feature = "archaic-korean")]
        Self::UYeo,
        #[cfg(feature = "archaic-korean")]
        Self::UYe,
        #[cfg(feature = "archaic-korean")]
        Self::UU,
        Self::Wi,
        #[cfg(feature = "archaic-korean")]
        Self::UII,
        Self::Yu,
        #[cfg(feature = "archaic-korean")]
        Self::YuA,
        #[cfg(feature = "archaic-korean")]
        Self::YuAe,
        #[cfg(feature = "archaic-korean")]
        Self::YuEo,
        #[cfg(feature = "archaic-korean")]
        Self::YuE,
        #[cfg(feature = "archaic-korean")]
        Self::YuYeo,
        #[cfg(feature = "archaic-korean")]
        Self::YuYe,
        #[cfg(feature = "archaic-korean")]
        Self::YuO,
        #[cfg(feature = "archaic-korean")]
        Self::YuU,
        #[cfg(feature = "archaic-korean")]
        Self::YuI,
        Self::Eu,
        #[cfg(feature = "archaic-korean")]
        Self::EuA,
        #[cfg(feature = "archaic-korean")]
        Self::EuEo,
        #[cfg(feature = "archaic-korean")]
        Self::EuE,
        #[cfg(feature = "archaic-korean")]
        Self::EuO,
        #[cfg(feature = "archaic-korean")]
        Self::EuU,
        #[cfg(feature = "archaic-korean")]
        Self::EuEu,
        Self::Yi,
        #[cfg(feature = "archaic-korean")]
        Self::YiU,
        Self::I,
        #[cfg(feature = "archaic-korean")]
        Self::IA,
        #[cfg(feature = "archaic-korean")]
        Self::IYa,
        #[cfg(feature = "archaic-korean")]
        Self::IYaO,
        #[cfg(feature = "archaic-korean")]
        Self::IYae,
        #[cfg(feature = "archaic-korean")]
        Self::IYeo,
        #[cfg(feature = "archaic-korean")]
        Self::IYe,
        #[cfg(feature = "archaic-korean")]
        Self::IO,
        #[cfg(feature = "archaic-korean")]
        Self::IOI,
        #[cfg(feature = "archaic-korean")]
        Self::IYo,
        #[cfg(feature = "archaic-korean")]
        Self::IU,
        #[cfg(feature = "archaic-korean")]
        Self::IYu,
        #[cfg(feature = "archaic-korean")]
        Self::IEu,
        #[cfg(feature = "archaic-korean")]
        Self::II,
        #[cfg(feature = "archaic-korean")]
        Self::IAraeA,
        #[cfg(feature = "archaic-korean")]
        Self::AraeA,
        #[cfg(feature = "archaic-korean")]
        Self::AraeAA,
        #[cfg(feature = "archaic-korean")]
        Self::AraeAEo,
        #[cfg(feature = "archaic-korean")]
        Self::AraeAE,
        #[cfg(feature = "archaic-korean")]
        Self::AraeAU,
        #[cfg(feature = "archaic-korean")]
        Self::AraeAI,
        #[cfg(feature = "archaic-korean")]
        Self::SsangAraeA,
    ];
}
