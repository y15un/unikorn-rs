use crate::{
    consonant::{Choseong, HalfwidthJaeum, Jongseong},
    Error,
};
use num_enum::{IntoPrimitive, TryFromPrimitive};
use std::{
    convert::TryFrom,
    fmt::{Display, Formatter, Result as FmtResult},
};

/// A complete set of consonants (자음, Jaeum).
///
/// However, the completeness is not guaranteed if `archaic-korean` feature is enabled.
#[derive(Clone, Copy, Debug, Eq, IntoPrimitive, Ord, PartialEq, PartialOrd, TryFromPrimitive)]
#[repr(u32)]
pub enum Jaeum {
    /// Represents consonant `ㄱ` (U+3131, Hangul Letter Kiyeok)
    Kiyeok = 0x3131,
    /// Represents consonant `ㄲ` (U+3132, Hangul Letter SsangKiyeok)
    SsangKiyeok,
    /// Represents consonant `ㄳ` (U+3133, Hangul Letter Kiyeok-Sios)
    KiyeokSios,
    /// Represents consonant `ㄴ` (U+3134, Hangul Letter Nieun)
    Nieun,
    /// Represents consonant `ㄵ` (U+3135, Hangul Letter Nieun-Cieuc)
    NieunCieuc,
    /// Represents consonant `ㄶ` (U+3136, Hangul Letter Nieun-Hieuh)
    NieunHieuh,
    /// Represents consonant `ㄷ` (U+3137, Hangul Letter Tikeut)
    Tikeut,
    /// Represents consonant `ㄸ` (U+3138, Hangul Letter SsangTikeut)
    SsangTikeut,
    /// Represents consonant `ㄹ` (U+3139, Hangul Letter Rieul)
    Rieul,
    /// Represents consonant `ㄺ` (U+313A, Hangul Letter Rieul-Kiyeok)
    RieulKiyeok,
    /// Represents consonant `ㄻ` (U+313B, Hangul Letter Rieul-Mieum)
    RieulMieum,
    /// Represents consonant `ㄼ` (U+313C, Hangul Letter Rieul-Pieup)
    RieulPieup,
    /// Represents consonant `ㄽ` (U+313D, Hangul Letter Rieul-Sios)
    RieulSios,
    /// Represents consonant `ㄾ` (U+313E, Hangul Letter Rieul-Thieuth)
    RieulThieuth,
    /// Represents consonant `ㄿ` (U+313F, Hangul Letter Rieul-Phieuph)
    RieulPhieuph,
    /// Represents consonant `ㅀ` (U+3140, Hangul Letter Rieul-Hieuh)
    RieulHieuh,
    /// Represents consonant `ㅁ` (U+3141, Hangul Letter Mieum)
    Mieum,
    /// Represents consonant `ㅂ` (U+3142, Hangul Letter Pieup)
    Pieup,
    /// Represents consonant `ㅃ` (U+3143, Hangul Letter SsangPieup)
    SsangPieup,
    /// Represents consonant `ㅄ` (U+3144, Hangul Letter Pieup-Sios)
    PieupSios,
    /// Represents consonant `ㅅ` (U+3145, Hangul Letter Sios)
    Sios,
    /// Represents consonant `ㅆ` (U+3146, Hangul Letter SsangSios)
    SsangSios,
    /// Represents consonant `ㅇ` (U+3147, Hangul Letter Ieung)
    Ieung,
    /// Represents consonant `ㅈ` (U+3148, Hangul Letter Cieuc)
    Cieuc,
    /// Represents consonant `ㅉ` (U+3149, Hangul Letter SsangCieuc)
    SsangCieuc,
    /// Represents consonant `ㅊ` (U+314A, Hangul Letter Chieuch)
    Chieuch,
    /// Represents consonant `ㅋ` (U+314B, Hangul Letter Khieukh)
    Khieukh,
    /// Represents consonant `ㅌ` (U+314C, Hangul Letter Thieuth)
    Thieuth,
    /// Represents consonant `ㅍ` (U+314D, Hangul Letter Phieuph)
    Phieuph,
    /// Represents consonant `ㅎ` (U+314E, Hangul Letter Hieuh)
    Hieuh,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* consonant `ㅥ` (U+3165, Hangul Letter SsangNieun)
    SsangNieun = 0x3165,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* consonant `ㅦ` (U+3166, Hangul Letter Nieun-Tikeut)
    NieunTikeut,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* consonant `ㅧ` (U+3167, Hangul Letter Nieun-Sios)
    NieunSios,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* consonant `ㅨ` (U+3168, Hangul Letter Nieun-PanSios)
    NieunPanSios,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* consonant `ㅩ` (U+3169, Hangul Letter Rieul-Kiyeok-Sios)
    RieulKiyeokSios,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* consonant `ㅪ` (U+316A, Hangul Letter Rieul-Tikeut)
    RieulTikeut,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* consonant `ㅫ` (U+316B, Hangul Letter Rieul-Pieup-Sios)
    RieulPieupSios,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* consonant `ㅬ` (U+316C, Hangul Letter Rieul-PanSios)
    RieulPanSios,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* consonant `ㅭ` (U+316D, Hangul Letter Rieul-YeorinHieuh)
    RieulYeorinHieuh,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* consonant `ㅮ` (U+316E, Hangul Letter Mieum-Pieup)
    MieumPieup,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* consonant `ㅯ` (U+316F, Hangul Letter Mieum-Sios)
    MieumSios,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* consonant `ㅰ` (U+3170, Hangul Letter Mieum-PanSios)
    MieumPanSios,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* consonant `ㅱ` (U+3171, Hangul Letter KapyeounMieum)
    KapyeounMieum,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* consonant `ㅲ` (U+3172, Hangul Letter Pieup-Kiyeok)
    PieupKiyeok,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* consonant `ㅳ` (U+3173, Hangul Letter Pieup-Tikeut)
    PieupTikeut,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* consonant `ㅴ` (U+3174, Hangul Letter Pieup-Sios-Kiyeok)
    PieupSiosKiyeok,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* consonant `ㅵ` (U+3175, Hangul Letter Pieup-Sios-Tikeut)
    PieupSiosTikeut,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* consonant `ㅶ` (U+3176, Hangul Letter Pieup-Cieuc)
    PieupCieuc,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* consonant `ㅷ` (U+3177, Hangul Letter Pieup-Thieuth)
    PieupThieuth,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* consonant `ㅸ` (U+3178, Hangul Letter KapyeounPieup)
    KapyeounPieup,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* consonant `ㅹ` (U+3179, Hangul Letter KapyeounSsangPieup)
    KapyeounSsangPieup,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* consonant `ㅺ` (U+317A, Hangul Letter Sios-Kiyeok)
    SiosKiyeok,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* consonant `ㅻ` (U+317B, Hangul Letter Sios-Nieun)
    SiosNieun,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* consonant `ㅼ` (U+317C, Hangul Letter Sios-Tikeut)
    SiosTikeut,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* consonant `ㅽ` (U+317D, Hangul Letter Sios-Pieup)
    SiosPieup,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* consonant `ㅾ` (U+317E, Hangul Letter Sios-Cieuc)
    SiosCieuc,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* consonant `ㅿ` (U+317F, Hangul Letter PanSios)
    PanSios,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* consonant `ㆀ` (U+3180, Hangul Letter SsangIeung)
    SsangIeung,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* consonant `ㆁ` (U+3181, Hangul Letter YesIeung)
    YesIeung,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* consonant `ㆂ` (U+3182, Hangul Letter YesIeung-Sios)
    YesIeungSios,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* consonant `ㆃ` (U+3183, Hangul Letter YesIeung-PanSios)
    YesIeungPanSios,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* consonant `ㆄ` (U+3184, Hangul Letter KapyeounPhieuph)
    KapyeounPhieuph,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* consonant `ㆅ` (U+3185, Hangul Letter SsangHieuh)
    SsangHieuh,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* consonant `ㆆ` (U+3186, Hangul Letter YeorinHieuh)
    YeorinHieuh,
}
impl Display for Jaeum {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", char::from(*self))
    }
}
impl From<HalfwidthJaeum> for Jaeum {
    fn from(value: HalfwidthJaeum) -> Self {
        let code_point = value as u32;

        match code_point {
            0xFFA1..=0xFFBE => Self::try_from(code_point - 0xCE70).unwrap(),
            _ => unreachable!("This operation is infallible"),
            // TODO: why not this?
            // _ => unsafe {
            //     core::hint::unreachable_unchecked()
            // }
        }
    }
}
impl From<Jaeum> for char {
    fn from(value: Jaeum) -> Self {
        // guaranteed to not fail within BMP
        char::from_u32(value as u32).unwrap()
    }
}
impl TryFrom<char> for Jaeum {
    type Error = Error;

    /// Tries to convert a [`char`] into [`Jaeum`].
    ///
    /// # Direct Conversion
    /// The [`char`] given will be tested against the following range(s):
    ///
    /// * Hangul Compatibility Jamo / Consonant letters (U+3131--U+314E)
    /// * Hangul Compatibility Jamo / Old consonant letters (U+3165--U+3186)
    ///
    /// The last one range is considered valid if and only if `archaic-korean` feature is enabled.
    ///
    /// # Indirect Conversion
    /// ## Via [`Choseong`]
    /// For the following range(s), the [`char`] given will be tried converting into [`Choseong`] first, then subsequently into [`Jaeum`]:
    ///
    /// * Hangul Jamo / Initial consonants (U+1100--U+1112)
    /// * Hangul Jamo / Old initial consonants (U+1113--U+115E)
    /// * Hangul Jamo Extended-A / Old initial consonants (U+A960--U+A97C)
    ///
    /// The latter two ranges are considered valid if and only if `archaic-korean` feature is enabled.
    ///
    /// ## Via [`HalfwidthJaeum`]
    /// For the following range(s), the [`char`] given will be tried converting into [`HalfwidthJaeum`] first, then subsequently into [`Jaeum`]:
    ///
    /// * Halfwidth and Fullwidth Forms / Halfwidth Hangul variants (U+FFA1--U+FFBE)
    ///
    /// ## Via [`Jongseong`]
    /// For the following range(s), the [`char`] given will be tried converting into [`Jongseong`] first, then subsequently into [`Jaeum`]:
    ///
    /// * Hangul Jamo / Final consonants (U+11A8--U+11C2)
    /// * Hangul Jamo / Old final consonants (U+11C3--U+11FF)
    /// * Hangul Jamo Extended-B / Old final consonants (U+D7CB--U+D7FB)
    ///
    /// The latter two ranges are considered valid if and only if `archaic-korean` feature is enabled.
    ///
    /// # Errors
    /// * [`Error::NonJaeumTryFromChar`]: the [`char`] given is not a valid consonant.
    /// * [`Error::NonKoreanTryFromChar`]: the [`char`] given is not a valid Korean letter.
    fn try_from(value: char) -> Result<Self, Self::Error> {
        let code_point = value as u32;
        let naja = Err(Error::NonJaeumTryFromChar(value));

        match code_point {
            // initial consonants
            0x1100..=0x1112 => {
                return Self::try_from(Choseong::try_from(code_point).unwrap()).or(naja)
            }
            // vowels
            0x1161..=0x1175
            | 0x314F..=0x3163
            | 0xFFC2..=0xFFC7
            | 0xFFCA..=0xFFCF
            | 0xFFD2..=0xFFD7
            | 0xFFDA..=0xFFDC => return naja,
            // final consonants
            0x11A8..=0x11C2 => {
                return Self::try_from(Jongseong::try_from(code_point).unwrap()).or(naja)
            }
            // generic consonants (self)
            0x3131..=0x314E => return Ok(Self::try_from(code_point).unwrap()),
            // halfwidth consonants
            0xFFA1..=0xFFBE => {
                return Ok(Self::from(HalfwidthJaeum::try_from(code_point).unwrap()))
            }
            _ => {}
        }

        #[cfg(feature = "archaic-korean")]
        match code_point {
            // old initial consonants
            0x1113..=0x115E | 0xA960..=0xA97C => {
                return Self::try_from(Choseong::try_from(code_point).unwrap()).or(naja)
            }
            // old vowels
            0x1176..=0x11A7 | 0x3187..=0x318E | 0xD7B0..=0xD7C6 => return naja,
            // old final consonants
            0x11C3..=0x11FF | 0xD7CB..=0xD7FB => {
                return Self::try_from(Jongseong::try_from(code_point).unwrap()).or(naja)
            }
            // old generic consonants (self)
            0x3165..=0x3186 => return Ok(Self::try_from(code_point).unwrap()),
            _ => {}
        }

        Err(Error::NonKoreanTryFromChar(value))
    }
}
impl TryFrom<Choseong> for Jaeum {
    #[cfg(not(feature = "archaic-korean"))]
    type Error = std::convert::Infallible;
    #[cfg(feature = "archaic-korean")]
    type Error = Error;

    /// Tries to convert a [`Choseong`] into [`Jaeum`].
    ///
    /// # Errors
    /// ## Without `archaic-korean` Feature
    /// This operation is guaranteed  **infallible**.
    ///
    /// ## With `archaic-korean` Feature
    /// * [`Error::NoUnicodeJaeumTryFromChoseong`]: the [`Choseong`] given does not have its [`Jaeum`]-equivalent in Unicode.
    fn try_from(value: Choseong) -> Result<Self, Self::Error> {
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
            Choseong::SsangNieun => Ok(Self::SsangNieun),
            #[cfg(feature = "archaic-korean")]
            Choseong::NieunTikeut => Ok(Self::NieunTikeut),
            #[cfg(feature = "archaic-korean")]
            Choseong::RieulHieuh => Ok(Self::RieulHieuh),
            #[cfg(feature = "archaic-korean")]
            Choseong::MieumPieup => Ok(Self::MieumPieup),
            #[cfg(feature = "archaic-korean")]
            Choseong::KapyeounMieum => Ok(Self::KapyeounMieum),
            #[cfg(feature = "archaic-korean")]
            Choseong::PieupKiyeok => Ok(Self::PieupKiyeok),
            #[cfg(feature = "archaic-korean")]
            Choseong::PieupTikeut => Ok(Self::PieupTikeut),
            #[cfg(feature = "archaic-korean")]
            Choseong::PieupSios => Ok(Self::PieupSios),
            #[cfg(feature = "archaic-korean")]
            Choseong::PieupSiosKiyeok => Ok(Self::PieupSiosKiyeok),
            #[cfg(feature = "archaic-korean")]
            Choseong::PieupSiosTikeut => Ok(Self::PieupSiosTikeut),
            #[cfg(feature = "archaic-korean")]
            Choseong::PieupCieuc => Ok(Self::PieupCieuc),
            #[cfg(feature = "archaic-korean")]
            Choseong::PieupThieuth => Ok(Self::PieupThieuth),
            #[cfg(feature = "archaic-korean")]
            Choseong::KapyeounPieup => Ok(Self::KapyeounPieup),
            #[cfg(feature = "archaic-korean")]
            Choseong::KapyeounSsangPieup => Ok(Self::KapyeounSsangPieup),
            #[cfg(feature = "archaic-korean")]
            Choseong::SiosKiyeok => Ok(Self::SiosKiyeok),
            #[cfg(feature = "archaic-korean")]
            Choseong::SiosNieun => Ok(Self::SiosNieun),
            #[cfg(feature = "archaic-korean")]
            Choseong::SiosTikeut => Ok(Self::SiosTikeut),
            #[cfg(feature = "archaic-korean")]
            Choseong::SiosPieup => Ok(Self::SiosPieup),
            #[cfg(feature = "archaic-korean")]
            Choseong::SiosCieuc => Ok(Self::SiosCieuc),
            #[cfg(feature = "archaic-korean")]
            Choseong::PanSios => Ok(Self::PanSios),
            #[cfg(feature = "archaic-korean")]
            Choseong::SsangIeung => Ok(Self::SsangIeung),
            #[cfg(feature = "archaic-korean")]
            Choseong::YesIeung => Ok(Self::YesIeung),
            #[cfg(feature = "archaic-korean")]
            Choseong::KapyeounPhieuph => Ok(Self::KapyeounPhieuph),
            #[cfg(feature = "archaic-korean")]
            Choseong::SsangHieuh => Ok(Self::SsangHieuh),
            #[cfg(feature = "archaic-korean")]
            Choseong::YeorinHieuh => Ok(Self::YeorinHieuh),
            #[cfg(feature = "archaic-korean")]
            Choseong::NieunSios => Ok(Self::NieunSios),
            #[cfg(feature = "archaic-korean")]
            Choseong::NieunCieuc => Ok(Self::NieunCieuc),
            #[cfg(feature = "archaic-korean")]
            Choseong::NieunHieuh => Ok(Self::NieunHieuh),
            #[cfg(feature = "archaic-korean")]
            Choseong::RieulKiyeok => Ok(Self::RieulKiyeok),
            #[cfg(feature = "archaic-korean")]
            Choseong::RieulTikeut => Ok(Self::RieulTikeut),
            #[cfg(feature = "archaic-korean")]
            Choseong::RieulMieum => Ok(Self::RieulMieum),
            #[cfg(feature = "archaic-korean")]
            Choseong::RieulPieup => Ok(Self::RieulPieup),
            #[cfg(feature = "archaic-korean")]
            Choseong::RieulSios => Ok(Self::RieulSios),
            #[cfg(feature = "archaic-korean")]
            Choseong::MieumSios => Ok(Self::MieumSios),
            #[cfg(feature = "archaic-korean")]
            _ => Err(Error::NoUnicodeJaeumTryFromChoseong(value)),
        }
    }
}
impl TryFrom<Jongseong> for Jaeum {
    #[cfg(not(feature = "archaic-korean"))]
    type Error = std::convert::Infallible;
    #[cfg(feature = "archaic-korean")]
    type Error = Error;

    /// Tries to convert a [`Jongseong`] into [`Jaeum`].
    ///
    /// # Errors
    /// ## Without `archaic-korean` Feature
    /// This operation is guaranteed  **infallible**.
    ///
    /// ## With `archaic-korean` Feature
    /// * [`Error::NoUnicodeJaeumTryFromJongseong`]: the [`Jongseong`] given does not have its [`Jaeum`]-equivalent in Unicode.
    fn try_from(value: Jongseong) -> Result<Self, Self::Error> {
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
            Jongseong::NieunTikeut => Ok(Self::NieunTikeut),
            #[cfg(feature = "archaic-korean")]
            Jongseong::NieunSios => Ok(Self::NieunSios),
            #[cfg(feature = "archaic-korean")]
            Jongseong::NieunPanSios => Ok(Self::NieunPanSios),
            #[cfg(feature = "archaic-korean")]
            Jongseong::RieulKiyeokSios => Ok(Self::RieulKiyeokSios),
            #[cfg(feature = "archaic-korean")]
            Jongseong::RieulTikeut => Ok(Self::RieulTikeut),
            #[cfg(feature = "archaic-korean")]
            Jongseong::RieulPieupSios => Ok(Self::RieulPieupSios),
            #[cfg(feature = "archaic-korean")]
            Jongseong::RieulPanSios => Ok(Self::RieulPanSios),
            #[cfg(feature = "archaic-korean")]
            Jongseong::RieulYeorinHieuh => Ok(Self::RieulYeorinHieuh),
            #[cfg(feature = "archaic-korean")]
            Jongseong::MieumPieup => Ok(Self::MieumPieup),
            #[cfg(feature = "archaic-korean")]
            Jongseong::MieumSios => Ok(Self::MieumSios),
            #[cfg(feature = "archaic-korean")]
            Jongseong::MieumPanSios => Ok(Self::MieumPanSios),
            #[cfg(feature = "archaic-korean")]
            Jongseong::KapyeounMieum => Ok(Self::KapyeounMieum),
            #[cfg(feature = "archaic-korean")]
            Jongseong::KapyeounPieup => Ok(Self::KapyeounPieup),
            #[cfg(feature = "archaic-korean")]
            Jongseong::SiosKiyeok => Ok(Self::SiosKiyeok),
            #[cfg(feature = "archaic-korean")]
            Jongseong::SiosTikeut => Ok(Self::SiosTikeut),
            #[cfg(feature = "archaic-korean")]
            Jongseong::SiosPieup => Ok(Self::SiosPieup),
            #[cfg(feature = "archaic-korean")]
            Jongseong::PanSios => Ok(Self::PanSios),
            #[cfg(feature = "archaic-korean")]
            Jongseong::SsangIeung => Ok(Self::SsangIeung),
            #[cfg(feature = "archaic-korean")]
            Jongseong::YesIeung => Ok(Self::YesIeung),
            #[cfg(feature = "archaic-korean")]
            Jongseong::YesIeungSios => Ok(Self::YesIeungSios),
            #[cfg(feature = "archaic-korean")]
            Jongseong::YesIeungPanSios => Ok(Self::YesIeungPanSios),
            #[cfg(feature = "archaic-korean")]
            Jongseong::KapyeounPhieuph => Ok(Self::KapyeounPhieuph),
            #[cfg(feature = "archaic-korean")]
            Jongseong::YeorinHieuh => Ok(Self::YeorinHieuh),
            #[cfg(feature = "archaic-korean")]
            Jongseong::SsangNieun => Ok(Self::SsangNieun),
            #[cfg(feature = "archaic-korean")]
            Jongseong::SsangTikeut => Ok(Self::SsangTikeut),
            #[cfg(feature = "archaic-korean")]
            Jongseong::PieupTikeut => Ok(Self::PieupTikeut),
            #[cfg(feature = "archaic-korean")]
            Jongseong::SsangPieup => Ok(Self::SsangPieup),
            #[cfg(feature = "archaic-korean")]
            Jongseong::PieupSiosTikeut => Ok(Self::PieupSiosTikeut),
            #[cfg(feature = "archaic-korean")]
            Jongseong::PieupCieuc => Ok(Self::PieupCieuc),
            #[cfg(feature = "archaic-korean")]
            Jongseong::SiosCieuc => Ok(Self::SiosCieuc),
            #[cfg(feature = "archaic-korean")]
            Jongseong::SsangCieuc => Ok(Self::SsangCieuc),
            #[cfg(feature = "archaic-korean")]
            _ => Err(Error::NoUnicodeJaeumTryFromJongseong(value)),
        }
    }
}
impl Jaeum {
    // This list is only exported with `archaic-korean` feature, because without it the [`Jaeum`] should be in order by itself.
    #[cfg(feature = "archaic-korean")]
    /// Lists [`Jaeum`] in correct dictionary order.
    pub const IN_ORDER: [Self; 64] = [
        Self::Kiyeok,
        Self::SsangKiyeok,
        Self::KiyeokSios,
        Self::Nieun,
        #[cfg(feature = "archaic-korean")]
        Self::SsangNieun,
        #[cfg(feature = "archaic-korean")]
        Self::NieunTikeut,
        #[cfg(feature = "archaic-korean")]
        Self::NieunSios,
        #[cfg(feature = "archaic-korean")]
        Self::NieunPanSios,
        Self::NieunCieuc,
        Self::NieunHieuh,
        Self::Tikeut,
        Self::SsangTikeut,
        Self::Rieul,
        Self::RieulKiyeok,
        #[cfg(feature = "archaic-korean")]
        Self::RieulKiyeokSios,
        #[cfg(feature = "archaic-korean")]
        Self::RieulTikeut,
        Self::RieulMieum,
        Self::RieulPieup,
        #[cfg(feature = "archaic-korean")]
        Self::RieulPieupSios,
        Self::RieulSios,
        #[cfg(feature = "archaic-korean")]
        Self::RieulPanSios,
        Self::RieulThieuth,
        Self::RieulPhieuph,
        Self::RieulHieuh,
        #[cfg(feature = "archaic-korean")]
        Self::RieulYeorinHieuh,
        Self::Mieum,
        #[cfg(feature = "archaic-korean")]
        Self::MieumPieup,
        #[cfg(feature = "archaic-korean")]
        Self::MieumSios,
        #[cfg(feature = "archaic-korean")]
        Self::MieumPanSios,
        #[cfg(feature = "archaic-korean")]
        Self::KapyeounMieum,
        Self::Pieup,
        #[cfg(feature = "archaic-korean")]
        Self::PieupKiyeok,
        #[cfg(feature = "archaic-korean")]
        Self::PieupTikeut,
        Self::SsangPieup,
        Self::PieupSios,
        #[cfg(feature = "archaic-korean")]
        Self::PieupSiosKiyeok,
        #[cfg(feature = "archaic-korean")]
        Self::PieupSiosTikeut,
        #[cfg(feature = "archaic-korean")]
        Self::PieupCieuc,
        #[cfg(feature = "archaic-korean")]
        Self::PieupThieuth,
        #[cfg(feature = "archaic-korean")]
        Self::KapyeounPieup,
        #[cfg(feature = "archaic-korean")]
        Self::KapyeounSsangPieup,
        Self::Sios,
        #[cfg(feature = "archaic-korean")]
        Self::SiosKiyeok,
        #[cfg(feature = "archaic-korean")]
        Self::SiosNieun,
        #[cfg(feature = "archaic-korean")]
        Self::SiosTikeut,
        #[cfg(feature = "archaic-korean")]
        Self::SiosPieup,
        Self::SsangSios,
        #[cfg(feature = "archaic-korean")]
        Self::SiosCieuc,
        #[cfg(feature = "archaic-korean")]
        Self::PanSios,
        Self::Ieung,
        #[cfg(feature = "archaic-korean")]
        Self::SsangIeung,
        #[cfg(feature = "archaic-korean")]
        Self::YesIeung,
        #[cfg(feature = "archaic-korean")]
        Self::YesIeungSios,
        #[cfg(feature = "archaic-korean")]
        Self::YesIeungPanSios,
        Self::Cieuc,
        Self::SsangCieuc,
        Self::Chieuch,
        Self::Khieukh,
        Self::Thieuth,
        Self::Phieuph,
        #[cfg(feature = "archaic-korean")]
        Self::KapyeounPhieuph,
        Self::Hieuh,
        #[cfg(feature = "archaic-korean")]
        Self::SsangHieuh,
        #[cfg(feature = "archaic-korean")]
        Self::YeorinHieuh,
    ];
}
