use crate::{
    consonant::{Choseong, Jaeum, Jongseong},
    Error,
};
use num_enum::{IntoPrimitive, TryFromPrimitive};
use std::{
    convert::TryFrom,
    fmt::{Display, Formatter, Result as FmtResult},
};

/// A complete set of consonants (자음, Jaeum), but in halfwidth form.
#[derive(Clone, Copy, Debug, Eq, IntoPrimitive, Ord, PartialEq, PartialOrd, TryFromPrimitive)]
#[repr(u32)]
pub enum HalfwidthJaeum {
    /// Represents halfwidth consonant `ﾡ` (U+FFA1, Halfwidth Hangul Letter Kiyeok)
    Kiyeok = 0xFFA1,
    /// Represents halfwidth consonant `ﾢ` (U+FFA2, Halfwidth Hangul Letter SsangKiyeok)
    SsangKiyeok,
    /// Represents halfwidth consonant `ﾣ` (U+FFA3, Halfwidth Hangul Letter Kiyeok-Sios)
    KiyeokSios,
    /// Represents halfwidth consonant `ﾤ` (U+FFA4, Halfwidth Hangul Letter Nieun)
    Nieun,
    /// Represents halfwidth consonant `ﾥ` (U+FFA5, Halfwidth Hangul Letter Nieun-Cieuc)
    NieunCieuc,
    /// Represents halfwidth consonant `ﾦ` (U+FFA6, Halfwidth Hangul Letter Nieun-Hieuh)
    NieunHieuh,
    /// Represents halfwidth consonant `ﾧ` (U+FFA7, Halfwidth Hangul Letter Tikeut)
    Tikeut,
    /// Represents halfwidth consonant `ﾨ` (U+FFA8, Halfwidth Hangul Letter SsangTikeut)
    SsangTikeut,
    /// Represents halfwidth consonant `ﾩ` (U+FFA9, Halfwidth Hangul Letter Rieul)
    Rieul,
    /// Represents halfwidth consonant `ﾪ` (U+FFAA, Halfwidth Hangul Letter Rieul-Kiyeok)
    RieulKiyeok,
    /// Represents halfwidth consonant `ﾫ` (U+FFAB, Halfwidth Hangul Letter Rieul-Mieum)
    RieulMieum,
    /// Represents halfwidth consonant `ﾬ` (U+FFAC, Halfwidth Hangul Letter Rieul-Pieup)
    RieulPieup,
    /// Represents halfwidth consonant `ﾭ` (U+FFAD, Halfwidth Hangul Letter Rieul-Sios)
    RieulSios,
    /// Represents halfwidth consonant `ﾮ` (U+FFAE, Halfwidth Hangul Letter Rieul-Thieuth)
    RieulThieuth,
    /// Represents halfwidth consonant `ﾯ` (U+FFAF, Halfwidth Hangul Letter Rieul-Phieuph)
    RieulPhieuph,
    /// Represents halfwidth consonant `ﾰ` (U+FFB0, Halfwidth Hangul Letter Rieul-Hieuh)
    RieulHieuh,
    /// Represents halfwidth consonant `ﾱ` (U+FFB1, Halfwidth Hangul Letter Mieum)
    Mieum,
    /// Represents halfwidth consonant `ﾲ` (U+FFB2, Halfwidth Hangul Letter Pieup)
    Pieup,
    /// Represents halfwidth consonant `ﾳ` (U+FFB3, Halfwidth Hangul Letter SsangPieup)
    SsangPieup,
    /// Represents halfwidth consonant `ﾴ` (U+FFB4, Halfwidth Hangul Letter Pieup-Sios)
    PieupSios,
    /// Represents halfwidth consonant `ﾵ` (U+FFB5, Halfwidth Hangul Letter Sios)
    Sios,
    /// Represents halfwidth consonant `ﾶ` (U+FFB6, Halfwidth Hangul Letter SsangSios)
    SsangSios,
    /// Represents halfwidth consonant `ﾷ` (U+FFB7, Halfwidth Hangul Letter Ieung)
    Ieung,
    /// Represents halfwidth consonant `ﾸ` (U+FFB8, Halfwidth Hangul Letter Cieuc)
    Cieuc,
    /// Represents halfwidth consonant `ﾹ` (U+FFB9, Halfwidth Hangul Letter SsangCieuc)
    SsangCieuc,
    /// Represents halfwidth consonant `ﾺ` (U+FFBA, Halfwidth Hangul Letter Chieuch)
    Chieuch,
    /// Represents halfwidth consonant `ﾻ` (U+FFBB, Halfwidth Hangul Letter Khieukh)
    Khieukh,
    /// Represents halfwidth consonant `ﾼ` (U+FFBC, Halfwidth Hangul Letter Thieuth)
    Thieuth,
    /// Represents halfwidth consonant `ﾽ` (U+FFBD, Halfwidth Hangul Letter Phieuph)
    Phieuph,
    /// Represents halfwidth consonant `ﾾ` (U+FFBE, Halfwidth Hangul Letter Hieuh)
    Hieuh,
}
impl Display for HalfwidthJaeum {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", char::from(*self))
    }
}
impl From<HalfwidthJaeum> for char {
    fn from(value: HalfwidthJaeum) -> Self {
        // guaranteed to not fail within BMP
        char::from_u32(value as u32).unwrap()
    }
}
impl TryFrom<char> for HalfwidthJaeum {
    type Error = Error;

    /// Tries to convert a [`char`] into [`HalfwidthJaeum`].
    ///
    /// # Direct Conversion
    /// The [`char`] given will be tested against the following range(s):
    ///
    /// * Halfwidth and Fullwidth Forms / Halfwidth Hangul variants (U+FFA1--U+FFBE)
    ///
    /// # Indirect Conversion
    /// ## Via [`Choseong`]
    /// For the following range(s), the [`char`] given will be tried converting into [`Choseong`] first, then subsequently into [`HalfwidthJaeum`]:
    ///
    /// * Hangul Jamo / Initial consonants (U+1100--U+1112)
    /// * Hangul Jamo / Old initial consonants (U+1113--U+115E)
    /// * Hangul Jamo Extended-A / Old initial consonants (U+A960--U+A97C)
    ///
    /// The latter two ranges are considered valid if and only if `archaic-korean` feature is enabled.
    ///
    /// ## Via [`Jaeum`]
    /// For the following range(s), the [`char`] given will be tried converting into [`Jaeum`] first, then subsequently into [`HalfwidthJaeum`]:
    ///
    /// * Hangul Compatibility Jamo / Consonant letters (U+3131--U+314E)
    /// * Hangul Compatibility Jamo / Old consonant letters (U+3165--U+3186)
    ///
    /// The latter one range is considered valid if and only if `archaic-korean` feature is enabled.
    ///
    /// ## Via [`Jongseong`]
    /// For the following range(s), the [`char`] given will be tried converting into [`Jongseong`] first, then subsequently into [`HalfwidthJaeum`]:
    ///
    /// * Hangul Jamo / Final consonants (U+11A8--U+11C2)
    /// * Hangul Jamo / Old final consonants (U+11C3--U+11FF)
    /// * Hangul Jamo Extended-B / Old final consonants (U+D7CB--U+D7FB)
    ///
    /// The latter two ranges are considered valid if and only if `archaic-korean` feature is enabled.
    ///
    /// # Errors
    /// * [`Error::NonHalfwidthJaeumTryFromChar`]: the [`char`] given is not a valid halfwidth consonant.
    /// * [`Error::NonJaeumTryFromChar`]: the [`char`] given is not a valid consonant.
    /// * [`Error::NonKoreanTryFromChar`]: the [`char`] given is not a valid Korean letter.
    fn try_from(value: char) -> Result<Self, Self::Error> {
        let code_point = value as u32;
        let nahaja = Err(Error::NonHalfwidthJaeumTryFromChar(value));

        match code_point {
            // initial consonants
            0x1100..=0x1112 => {
                return Self::try_from(Choseong::try_from(code_point).unwrap()).or(nahaja)
            }
            // vowels
            0x1161..=0x1175
            | 0x314F..=0x3163
            | 0xFFC2..=0xFFC7
            | 0xFFCA..=0xFFCF
            | 0xFFD2..=0xFFD7
            | 0xFFDA..=0xFFDC => return Err(Error::NonJaeumTryFromChar(value)),
            // final consonants
            0x11A8..=0x11C2 => {
                return Self::try_from(Jongseong::try_from(code_point).unwrap()).or(nahaja)
            }
            // generic consonants
            0x3131..=0x314E => {
                return Self::try_from(Jaeum::try_from(code_point).unwrap()).or(nahaja)
            }
            // halfwidth consonants (self)
            0xFFA1..=0xFFBE => return Ok(Self::try_from(code_point).unwrap()),
            _ => {}
        }

        #[cfg(feature = "archaic-korean")]
        match code_point {
            // old initial consonants
            0x1113..=0x115E | 0xA960..=0xA97C => {
                return Self::try_from(Choseong::try_from(code_point).unwrap()).or(nahaja)
            }
            // old vowels
            0x1176..=0x11A7 | 0x3187..=0x318E | 0xD7B0..=0xD7C6 => {
                return Err(Error::NonJaeumTryFromChar(value))
            }
            // old final consonants
            0x11C3..=0x11FF | 0xD7CB..=0xD7FB => {
                return Self::try_from(Jongseong::try_from(code_point).unwrap()).or(nahaja)
            }
            // old generic consonants
            0x3165..=0x3186 => {
                return Self::try_from(Jaeum::try_from(code_point).unwrap()).or(nahaja)
            }
            _ => {}
        }

        Err(Error::NonKoreanTryFromChar(value))
    }
}
impl TryFrom<Choseong> for HalfwidthJaeum {
    #[cfg(not(feature = "archaic-korean"))]
    type Error = std::convert::Infallible;
    #[cfg(feature = "archaic-korean")]
    type Error = Error;

    /// Tries to convert a [`Choseong`] into [`HalfwidthJaeum`].
    ///
    /// # Errors
    /// ## Without `archaic-korean` Feature
    /// This operation is guaranteed  **infallible**.
    ///
    /// ## With `archaic-korean` Feature
    /// * [`Error::NoUnicodeHalfwidthJaeumTryFromChoseong`]: the [`Choseong`] given does not have its [`HalfwidthJaeum`]-equivalent in Unicode.
    fn try_from(value: Choseong) -> Result<Self, Self::Error> {
        // TODO: consider switching to bst; but i'm not very sure of performance boost it'll yield.
        match value {
            Choseong::Kiyeok => Ok(Self::Kiyeok),
            Choseong::SsangKiyeok => Ok(Self::SsangKiyeok),
            Choseong::Nieun => Ok(Self::Nieun),
            Choseong::Tikeut => Ok(Self::Tikeut),
            Choseong::SsangTikeut => Ok(Self::SsangTikeut),
            Choseong::Rieul => Ok(Self::Rieul),
            Choseong::Mieum => Ok(Self::Mieum),
            Choseong::Pieup => Ok(Self::Pieup),
            Choseong::SsangPieup => Ok(Self::SsangPieup),
            Choseong::Sios => Ok(Self::Sios),
            Choseong::SsangSios => Ok(Self::SsangSios),
            Choseong::Ieung => Ok(Self::Ieung),
            Choseong::Cieuc => Ok(Self::Cieuc),
            Choseong::SsangCieuc => Ok(Self::SsangCieuc),
            Choseong::Chieuch => Ok(Self::Chieuch),
            Choseong::Khieukh => Ok(Self::Khieukh),
            Choseong::Thieuth => Ok(Self::Thieuth),
            Choseong::Phieuph => Ok(Self::Phieuph),
            Choseong::Hieuh => Ok(Self::Hieuh),
            #[cfg(feature = "archaic-korean")]
            Choseong::RieulHieuh => Ok(Self::RieulHieuh),
            #[cfg(feature = "archaic-korean")]
            Choseong::PieupSios => Ok(Self::PieupSios),
            #[cfg(feature = "archaic-korean")]
            Choseong::NieunCieuc => Ok(Self::NieunCieuc),
            #[cfg(feature = "archaic-korean")]
            Choseong::NieunHieuh => Ok(Self::NieunHieuh),
            #[cfg(feature = "archaic-korean")]
            Choseong::RieulKiyeok => Ok(Self::RieulKiyeok),
            #[cfg(feature = "archaic-korean")]
            Choseong::RieulMieum => Ok(Self::RieulMieum),
            #[cfg(feature = "archaic-korean")]
            Choseong::RieulPieup => Ok(Self::RieulPieup),
            #[cfg(feature = "archaic-korean")]
            Choseong::RieulSios => Ok(Self::RieulSios),
            #[cfg(feature = "archaic-korean")]
            _ => Err(Error::NoUnicodeHalfwidthJaeumTryFromChoseong(value)),
        }
    }
}
impl TryFrom<Jaeum> for HalfwidthJaeum {
    #[cfg(not(feature = "archaic-korean"))]
    type Error = std::convert::Infallible;
    #[cfg(feature = "archaic-korean")]
    type Error = Error;

    /// Tries to convert a [`Jaeum`] into [`HalfwidthJaeum`].
    ///
    /// # Errors
    /// ## Without `archaic-korean` Feature
    /// This operation is guaranteed **infallible**.
    ///
    /// ## With `archaic-korean` Feature
    /// * [`Error::NoUnicodeHalfwidthJaeumTryFromJaeum`]: the [`Jaeum`] given does not have a valid [`HalfwidthJaeum`]-equivalent in Unicode.
    fn try_from(value: Jaeum) -> Result<Self, Self::Error> {
        // TODO: consider switching to bst; but i'm not very sure of performance boost it'll yield.
        let code_point = value as u32;

        match code_point {
            0x3131..=0x314E => Ok(Self::try_from(code_point + 0xCE70).unwrap()),
            _ => Err(Error::NoUnicodeHalfwidthJaeumTryFromJaeum(value)),
        }
    }
}
impl TryFrom<Jongseong> for HalfwidthJaeum {
    #[cfg(not(feature = "archaic-korean"))]
    type Error = std::convert::Infallible;
    #[cfg(feature = "archaic-korean")]
    type Error = Error;

    /// Tries to convert a [`Jongseong`] into [`HalfwidthJaeum`].
    ///
    /// # Errors
    /// ## Without `archaic-korean` Feature
    /// This operation is guaranteed **infallible**.
    ///
    /// ## With `archaic-korean` Feature
    /// * [`Error::NoUnicodeHalfwidthJaeumTryFromJongseong`]: the [`Jongseong`] given does not have a valid [`HalfwidthJaeum`]-equivalent in Unicode.
    fn try_from(value: Jongseong) -> Result<Self, Self::Error> {
        // TODO: consider switching to bst; but i'm not very sure of performance boost it'll yield.
        match value {
            Jongseong::Kiyeok => Ok(Self::Kiyeok),
            Jongseong::SsangKiyeok => Ok(Self::SsangKiyeok),
            Jongseong::KiyeokSios => Ok(Self::KiyeokSios),
            Jongseong::Nieun => Ok(Self::Nieun),
            Jongseong::NieunCieuc => Ok(Self::NieunCieuc),
            Jongseong::NieunHieuh => Ok(Self::NieunHieuh),
            Jongseong::Tikeut => Ok(Self::Tikeut),
            Jongseong::Rieul => Ok(Self::Rieul),
            Jongseong::RieulKiyeok => Ok(Self::RieulKiyeok),
            Jongseong::RieulMieum => Ok(Self::RieulMieum),
            Jongseong::RieulPieup => Ok(Self::RieulPieup),
            Jongseong::RieulSios => Ok(Self::RieulSios),
            Jongseong::RieulThieuth => Ok(Self::RieulThieuth),
            Jongseong::RieulPhieuph => Ok(Self::RieulPhieuph),
            Jongseong::RieulHieuh => Ok(Self::RieulHieuh),
            Jongseong::Mieum => Ok(Self::Mieum),
            Jongseong::Pieup => Ok(Self::Pieup),
            Jongseong::PieupSios => Ok(Self::PieupSios),
            Jongseong::Sios => Ok(Self::Sios),
            Jongseong::SsangSios => Ok(Self::SsangSios),
            Jongseong::Ieung => Ok(Self::Ieung),
            Jongseong::Cieuc => Ok(Self::Cieuc),
            Jongseong::Chieuch => Ok(Self::Chieuch),
            Jongseong::Khieukh => Ok(Self::Khieukh),
            Jongseong::Thieuth => Ok(Self::Thieuth),
            Jongseong::Phieuph => Ok(Self::Phieuph),
            Jongseong::Hieuh => Ok(Self::Hieuh),
            #[cfg(feature = "archaic-korean")]
            Jongseong::SsangTikeut => Ok(Self::SsangTikeut),
            #[cfg(feature = "archaic-korean")]
            Jongseong::SsangPieup => Ok(Self::SsangPieup),
            #[cfg(feature = "archaic-korean")]
            Jongseong::SsangCieuc => Ok(Self::SsangCieuc),
            #[cfg(feature = "archaic-korean")]
            _ => Err(Error::NoUnicodeHalfwidthJaeumTryFromJongseong(value)),
        }
    }
}
