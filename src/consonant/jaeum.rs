use crate::{
    consonant::{Choseong, Jongseong},
    Error,
};
use num_enum::{IntoPrimitive, TryFromPrimitive};
use std::convert::TryFrom;

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
impl From<Jaeum> for char {
    fn from(value: Jaeum) -> Self {
        char::from_u32(value.into()).unwrap()
    }
}
impl TryFrom<char> for Jaeum {
    type Error = Error;

    /// Tries to convert a [`char`] into `Jaeum`.
    ///
    /// # Direct Conversion
    /// The [`char`] given will be tested against the following range(s):
    ///
    /// * Hangul Compatibility Jamo / Consonant letters (U+3131--U+314E)
    /// * Halfwidth and Fullwidth Forms / Halfwidth Hangul variants (U+FFA1--U+FFBE)
    /// * Hangul Compatibility Jamo / Old consonant letters (U+3165--U+3186)
    ///
    /// The last one range is considered valid if and only if `archaic-korean` feature is enabled.
    ///
    /// # Indirect Conversion
    /// ## Initial Consonants
    /// For the following range(s), the [`char`] given will be tried converting into [`Choseong`] first, then subsequently into `Jaeum`:
    ///
    /// * Hangul Jamo / Initial consonants (U+1100--U+1112)
    /// * Hangul Jamo / Old initial consonants (U+1113--U+115E)
    /// * Hangul Jamo Extended-A / Old initial consonants (U+A960--U+A97C)
    ///
    /// The latter two ranges are considered valid if and only if `archaic-korean` feature is enabled.
    ///
    /// ## Final Consonants
    /// For the following range(s), the [`char`] given will be tried converting into [`Jongseong`] first, then subsequently into `Jaeum`:
    ///
    /// * Hangul Jamo / Final consonants (U+11A8--U+11C2)
    /// * Hangul Jamo / Old final consonants (U+11C3--U+11FF)
    /// * Hangul Jamo Extended-B / Old final consonants (U+D7CB--U+D7FB)
    ///
    /// The latter two ranges are considered valid if and only if `archaic-korean` feature is enabled.
    ///
    /// # Errors
    /// * [`Error::NonJaeumTryFromChar`]: the [`char`] given is not a valid consonant.
    /// * [`Error::NonKoreanTryFromChar`]: the [`char`] given is not a valid Korean alphabet at all.
    fn try_from(value: char) -> Result<Self, Self::Error> {
        let code_point = value as u32;

        match code_point {
            0x1100..=0x1112 => {
                return Ok(Jaeum::try_from(Choseong::try_from(value)?)?);
            }
            0x11A8..=0x11C2 => {
                return Ok(Jaeum::try_from(Jongseong::try_from(value)?)?);
            }
            0x3131..=0x314E => return Ok(Jaeum::try_from(code_point).unwrap()),
            0xFFA1..=0xFFBE => return Ok(Jaeum::try_from(code_point - 0xCE70).unwrap()),
            0x1161..=0x1175
            | 0x314F..=0x3163
            | 0xFFC2..=0xFFC7
            | 0xFFCA..=0xFFCF
            | 0xFFD2..=0xFFD7
            | 0xFFDA..=0xFFDC => return Err(Error::NonJaeumTryFromChar(value)),
            _ => {}
        }

        #[cfg(feature = "archaic-korean")]
        match code_point {
            0x1113..=0x115E | 0xA960..=0xA97C => {
                return Ok(Jaeum::try_from(Choseong::try_from(value)?)?);
            }

            0x1176..=0x11A7 | 0x3187..=0x318E | 0xD7B0..=0xD7C6 => {
                return Err(Error::NonChoseongTryFromChar(value))
            }
            0x11C3..=0x11FF | 0xD7CB..=0xD7FB => {
                return Ok(Jaeum::try_from(Jongseong::try_from(value)?)?);
            }
            0x3165..=0x3186 => return Ok(Jaeum::try_from(code_point).unwrap()),
            _ => {}
        }

        Err(Error::NonKoreanTryFromChar(value))
    }
}
impl TryFrom<Choseong> for Jaeum {
    type Error = Error;

    /// Tries to convert a [`Choseong`] into `Jaeum`.
    ///
    /// # Errors
    /// ## Without `archaic-korean` Feature
    /// This operation is **infallible**.
    ///
    /// ## With `archaic-korean` Feature
    /// * [`Error::NonUnicodeJaeumTryFromChoseong`]: the [`Choseong`] given does not have its `Jaeum`-equivalent in Unicode.
    fn try_from(value: Choseong) -> Result<Self, Self::Error> {
        match value {
            Choseong::Kiyeok => Ok(Jaeum::Kiyeok),
            Choseong::SsangKiyeok => Ok(Jaeum::SsangKiyeok),
            Choseong::Nieun => Ok(Jaeum::Nieun),
            Choseong::Tikeut => Ok(Jaeum::Tikeut),
            Choseong::SsangTikeut => Ok(Jaeum::SsangTikeut),
            Choseong::Rieul => Ok(Jaeum::Rieul),
            Choseong::Mieum => Ok(Jaeum::Mieum),
            Choseong::Pieup => Ok(Jaeum::Pieup),
            Choseong::SsangPieup => Ok(Jaeum::SsangPieup),
            Choseong::Sios => Ok(Jaeum::Sios),
            Choseong::SsangSios => Ok(Jaeum::SsangSios),
            Choseong::Ieung => Ok(Jaeum::Ieung),
            Choseong::Cieuc => Ok(Jaeum::Cieuc),
            Choseong::SsangCieuc => Ok(Jaeum::SsangCieuc),
            Choseong::Chieuch => Ok(Jaeum::Chieuch),
            Choseong::Khieukh => Ok(Jaeum::Khieukh),
            Choseong::Thieuth => Ok(Jaeum::Thieuth),
            Choseong::Phieuph => Ok(Jaeum::Phieuph),
            Choseong::Hieuh => Ok(Jaeum::Hieuh),
            #[cfg(feature = "archaic-korean")]
            Choseong::SsangNieun => Ok(Jaeum::SsangNieun),
            #[cfg(feature = "archaic-korean")]
            Choseong::NieunTikeut => Ok(Jaeum::NieunTikeut),
            #[cfg(feature = "archaic-korean")]
            Choseong::RieulHieuh => Ok(Jaeum::RieulHieuh),
            #[cfg(feature = "archaic-korean")]
            Choseong::MieumPieup => Ok(Jaeum::MieumPieup),
            #[cfg(feature = "archaic-korean")]
            Choseong::KapyeounMieum => Ok(Jaeum::KapyeounMieum),
            #[cfg(feature = "archaic-korean")]
            Choseong::PieupKiyeok => Ok(Jaeum::PieupKiyeok),
            #[cfg(feature = "archaic-korean")]
            Choseong::PieupTikeut => Ok(Jaeum::PieupTikeut),
            #[cfg(feature = "archaic-korean")]
            Choseong::PieupSios => Ok(Jaeum::PieupSios),
            #[cfg(feature = "archaic-korean")]
            Choseong::PieupSiosKiyeok => Ok(Jaeum::PieupSiosKiyeok),
            #[cfg(feature = "archaic-korean")]
            Choseong::PieupSiosTikeut => Ok(Jaeum::PieupSiosTikeut),
            #[cfg(feature = "archaic-korean")]
            Choseong::PieupCieuc => Ok(Jaeum::PieupCieuc),
            #[cfg(feature = "archaic-korean")]
            Choseong::PieupThieuth => Ok(Jaeum::PieupThieuth),
            #[cfg(feature = "archaic-korean")]
            Choseong::KapyeounPieup => Ok(Jaeum::KapyeounPieup),
            #[cfg(feature = "archaic-korean")]
            Choseong::KapyeounSsangPieup => Ok(Jaeum::KapyeounSsangPieup),
            #[cfg(feature = "archaic-korean")]
            Choseong::SiosKiyeok => Ok(Jaeum::SiosKiyeok),
            #[cfg(feature = "archaic-korean")]
            Choseong::SiosNieun => Ok(Jaeum::SiosNieun),
            #[cfg(feature = "archaic-korean")]
            Choseong::SiosTikeut => Ok(Jaeum::SiosTikeut),
            #[cfg(feature = "archaic-korean")]
            Choseong::SiosPieup => Ok(Jaeum::SiosPieup),
            #[cfg(feature = "archaic-korean")]
            Choseong::SiosCieuc => Ok(Jaeum::SiosCieuc),
            #[cfg(feature = "archaic-korean")]
            Choseong::PanSios => Ok(Jaeum::PanSios),
            #[cfg(feature = "archaic-korean")]
            Choseong::SsangIeung => Ok(Jaeum::SsangIeung),
            #[cfg(feature = "archaic-korean")]
            Choseong::YesIeung => Ok(Jaeum::YesIeung),
            #[cfg(feature = "archaic-korean")]
            #[cfg(feature = "archaic-korean")]
            Choseong::KapyeounPhieuph => Ok(Jaeum::KapyeounPhieuph),
            #[cfg(feature = "archaic-korean")]
            Choseong::SsangHieuh => Ok(Jaeum::SsangHieuh),
            #[cfg(feature = "archaic-korean")]
            Choseong::YeorinHieuh => Ok(Jaeum::YeorinHieuh),
            #[cfg(feature = "archaic-korean")]
            Choseong::NieunSios => Ok(Jaeum::NieunSios),
            #[cfg(feature = "archaic-korean")]
            Choseong::NieunCieuc => Ok(Jaeum::NieunCieuc),
            #[cfg(feature = "archaic-korean")]
            Choseong::NieunHieuh => Ok(Jaeum::NieunHieuh),
            #[cfg(feature = "archaic-korean")]
            Choseong::RieulKiyeok => Ok(Jaeum::RieulKiyeok),
            #[cfg(feature = "archaic-korean")]
            Choseong::RieulTikeut => Ok(Jaeum::RieulTikeut),
            #[cfg(feature = "archaic-korean")]
            Choseong::RieulMieum => Ok(Jaeum::RieulMieum),
            #[cfg(feature = "archaic-korean")]
            Choseong::RieulPieup => Ok(Jaeum::RieulPieup),
            #[cfg(feature = "archaic-korean")]
            Choseong::RieulSios => Ok(Jaeum::RieulSios),
            #[cfg(feature = "archaic-korean")]
            Choseong::MieumSios => Ok(Jaeum::MieumSios),
            #[cfg(feature = "archaic-korean")]
            _ => Err(Error::NonUnicodeJaeumTryFromChoseong(value)),
        }
    }
}
impl TryFrom<Jongseong> for Jaeum {
    type Error = Error;

    /// Tries to convert a [`Jongseong`] into `Jaeum`.
    ///
    /// # Errors
    /// ## Without `archaic-korean` Feature
    /// This operation is **infallible**.
    ///
    /// ## With `archaic-korean` Feature
    /// * [`Error::NonUnicodeJaeumTryFromJongseong`]: the [`Jongseong`] given does not have its `Jaeum`-counterpart in Unicode.
    fn try_from(value: Jongseong) -> Result<Self, Self::Error> {
        match value {
            Jongseong::Kiyeok => Ok(Jaeum::Kiyeok),
            Jongseong::SsangKiyeok => Ok(Jaeum::SsangKiyeok),
            Jongseong::KiyeokSios => Ok(Jaeum::KiyeokSios),
            Jongseong::Nieun => Ok(Jaeum::Nieun),
            Jongseong::NieunCieuc => Ok(Jaeum::NieunCieuc),
            Jongseong::NieunHieuh => Ok(Jaeum::NieunHieuh),
            Jongseong::Tikeut => Ok(Jaeum::Tikeut),
            Jongseong::Rieul => Ok(Jaeum::Rieul),
            Jongseong::RieulKiyeok => Ok(Jaeum::RieulKiyeok),
            Jongseong::RieulMieum => Ok(Jaeum::RieulMieum),
            Jongseong::RieulPieup => Ok(Jaeum::RieulPieup),
            Jongseong::RieulSios => Ok(Jaeum::RieulSios),
            Jongseong::RieulThieuth => Ok(Jaeum::RieulThieuth),
            Jongseong::RieulPhieuph => Ok(Jaeum::RieulPhieuph),
            Jongseong::RieulHieuh => Ok(Jaeum::RieulHieuh),
            Jongseong::Mieum => Ok(Jaeum::Mieum),
            Jongseong::Pieup => Ok(Jaeum::Pieup),
            Jongseong::PieupSios => Ok(Jaeum::PieupSios),
            Jongseong::Sios => Ok(Jaeum::Sios),
            Jongseong::SsangSios => Ok(Jaeum::SsangSios),
            Jongseong::Ieung => Ok(Jaeum::Ieung),
            Jongseong::Cieuc => Ok(Jaeum::Cieuc),
            Jongseong::Chieuch => Ok(Jaeum::Chieuch),
            Jongseong::Khieukh => Ok(Jaeum::Khieukh),
            Jongseong::Thieuth => Ok(Jaeum::Thieuth),
            Jongseong::Phieuph => Ok(Jaeum::Phieuph),
            Jongseong::Hieuh => Ok(Jaeum::Hieuh),
            #[cfg(feature = "archaic-korean")]
            Jongseong::NieunTikeut => Ok(Jaeum::NieunTikeut),
            #[cfg(feature = "archaic-korean")]
            Jongseong::NieunSios => Ok(Jaeum::NieunSios),
            #[cfg(feature = "archaic-korean")]
            Jongseong::NieunPanSios => Ok(Jaeum::NieunPanSios),
            #[cfg(feature = "archaic-korean")]
            Jongseong::RieulKiyeokSios => Ok(Jaeum::RieulKiyeokSios),
            #[cfg(feature = "archaic-korean")]
            Jongseong::RieulTikeut => Ok(Jaeum::RieulTikeut),
            #[cfg(feature = "archaic-korean")]
            Jongseong::RieulPieupSios => Ok(Jaeum::RieulPieupSios),
            #[cfg(feature = "archaic-korean")]
            Jongseong::RieulPanSios => Ok(Jaeum::RieulPanSios),
            #[cfg(feature = "archaic-korean")]
            Jongseong::RieulYeorinHieuh => Ok(Jaeum::RieulYeorinHieuh),
            #[cfg(feature = "archaic-korean")]
            Jongseong::MieumPieup => Ok(Jaeum::MieumPieup),
            #[cfg(feature = "archaic-korean")]
            Jongseong::MieumSios => Ok(Jaeum::MieumSios),
            #[cfg(feature = "archaic-korean")]
            Jongseong::MieumPanSios => Ok(Jaeum::MieumPanSios),
            #[cfg(feature = "archaic-korean")]
            Jongseong::KapyeounMieum => Ok(Jaeum::KapyeounMieum),
            #[cfg(feature = "archaic-korean")]
            Jongseong::KapyeounPieup => Ok(Jaeum::KapyeounPieup),
            #[cfg(feature = "archaic-korean")]
            Jongseong::SiosKiyeok => Ok(Jaeum::SiosKiyeok),
            #[cfg(feature = "archaic-korean")]
            Jongseong::SiosTikeut => Ok(Jaeum::SiosTikeut),
            #[cfg(feature = "archaic-korean")]
            Jongseong::SiosPieup => Ok(Jaeum::SiosPieup),
            #[cfg(feature = "archaic-korean")]
            Jongseong::PanSios => Ok(Jaeum::PanSios),
            #[cfg(feature = "archaic-korean")]
            Jongseong::SsangIeung => Ok(Jaeum::SsangIeung),
            #[cfg(feature = "archaic-korean")]
            Jongseong::YesIeung => Ok(Jaeum::YesIeung),
            #[cfg(feature = "archaic-korean")]
            Jongseong::YesIeungSios => Ok(Jaeum::YesIeungSios),
            #[cfg(feature = "archaic-korean")]
            Jongseong::YesIeungPanSios => Ok(Jaeum::YesIeungPanSios),
            #[cfg(feature = "archaic-korean")]
            Jongseong::KapyeounPhieuph => Ok(Jaeum::KapyeounPhieuph),
            #[cfg(feature = "archaic-korean")]
            Jongseong::YeorinHieuh => Ok(Jaeum::YeorinHieuh),
            #[cfg(feature = "archaic-korean")]
            Jongseong::SsangNieun => Ok(Jaeum::SsangNieun),
            #[cfg(feature = "archaic-korean")]
            Jongseong::SsangTikeut => Ok(Jaeum::SsangTikeut),
            #[cfg(feature = "archaic-korean")]
            Jongseong::PieupTikeut => Ok(Jaeum::PieupTikeut),
            #[cfg(feature = "archaic-korean")]
            Jongseong::SsangPieup => Ok(Jaeum::SsangPieup),
            #[cfg(feature = "archaic-korean")]
            Jongseong::PieupSiosTikeut => Ok(Jaeum::PieupSiosTikeut),
            #[cfg(feature = "archaic-korean")]
            Jongseong::PieupCieuc => Ok(Jaeum::PieupCieuc),
            #[cfg(feature = "archaic-korean")]
            Jongseong::SiosCieuc => Ok(Jaeum::SiosCieuc),
            #[cfg(feature = "archaic-korean")]
            Jongseong::SsangCieuc => Ok(Jaeum::SsangCieuc),
            #[cfg(feature = "archaic-korean")]
            _ => Err(Error::NonUnicodeJaeumTryFromJongseong(value)),
        }
    }
}
