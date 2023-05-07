use crate::{Error, Jaeum};
use num_enum::{IntoPrimitive, TryFromPrimitive};

/// Groups all the consonants applicable to the 'initial consonant' (초성, Choseong) position of a
/// Korean syllable.
///
/// For modern consonants/double consonants, these reside in `U+1100`--`U+1112`.
///
/// However, things get interesting when the [`archaic-korean`](crate#archaic-korean) feature is
/// enabled. There are 105 archaic consonants/consonant sequences defined for use as 'archaic
/// initial consonant' in Unicode, but splited into two separate blocks (`U+1113`--`U+115E` and
/// `U+A960`--`U+A97C`) on top of being out of order.
///
/// An iterable array constant [`IN_ORDER`](Choseong::IN_ORDER) has been defined to list everything
/// in correct alphabetical order.
#[derive(Clone, Copy, Debug, Eq, IntoPrimitive, Ord, PartialEq, PartialOrd, TryFromPrimitive)]
#[repr(u16)]
pub enum Choseong {
    /// Represents 'ᄀ'.
    Kiyeok = 0x00, // Note: the u16 values are relative to 0x1100.
    /// Represents 'ᄁ'.
    SsangKiyeok,
    /// Represents 'ᄂ'.
    Nieun,
    /// Represents 'ᄃ'.
    Tikeut,
    /// Represents 'ᄄ'.
    SsangTikeut,
    /// Represents 'ᄅ'.
    Rieul,
    /// Represents 'ᄆ'.
    Mieum,
    /// Represents 'ᄇ'.
    Pieup,
    /// Represents 'ᄈ'.
    SsangPieup,
    /// Represents 'ᄉ'.
    Sios,
    /// Represents 'ᄊ'.
    SsangSios,
    /// Represents 'ᄋ'.
    Ieung,
    /// Represents 'ᄌ'.
    Cieuc,
    /// Represents 'ᄍ'.
    SsangCieuc,
    /// Represents 'ᄎ'.
    Chieuch,
    /// Represents 'ᄏ'.
    Khieukh,
    /// Represents 'ᄐ'.
    Thieuth,
    /// Represents 'ᄑ'.
    Phieuph,
    /// Represents 'ᄒ'.
    Hieuh,
    #[cfg(feature = "archaic-korean")]
    /// Represents 'ᄓ'.
    NieunKiyeok,
    #[cfg(feature = "archaic-korean")]
    /// Represents 'ᄔ'.
    SsangNieun,
    #[cfg(feature = "archaic-korean")]
    /// Represents 'ᄕ'.
    NieunTikeut,
    #[cfg(feature = "archaic-korean")]
    /// Represents 'ᄖ'.
    NieunPieup,
    #[cfg(feature = "archaic-korean")]
    /// Represents 'ᄗ'.
    TikeutKiyeok,
    #[cfg(feature = "archaic-korean")]
    /// Represents 'ᄘ'.
    RieulNieun,
    #[cfg(feature = "archaic-korean")]
    /// Represents 'ᄙ'.
    SsangRieul,
    #[cfg(feature = "archaic-korean")]
    /// Represents 'ᄚ'.
    RieulHieuh,
    #[cfg(feature = "archaic-korean")]
    /// Represents 'ᄛ'.
    KapyeounRieul,
    #[cfg(feature = "archaic-korean")]
    /// Represents 'ᄜ'.
    MieumPieup,
    #[cfg(feature = "archaic-korean")]
    /// Represents 'ᄝ'.
    KapyeounMieum,
    #[cfg(feature = "archaic-korean")]
    /// Represents 'ᄞ'.
    PieupKiyeok,
    #[cfg(feature = "archaic-korean")]
    /// Represents 'ᄟ'.
    PieupNieun,
    #[cfg(feature = "archaic-korean")]
    /// Represents 'ᄠ'.
    PieupTikeut,
    #[cfg(feature = "archaic-korean")]
    /// Represents 'ᄡ'.
    PieupSios,
    #[cfg(feature = "archaic-korean")]
    /// Represents 'ᄢ'.
    PieupSiosKiyeok,
    #[cfg(feature = "archaic-korean")]
    /// Represents 'ᄣ'.
    PieupSiosTikeut,
    #[cfg(feature = "archaic-korean")]
    /// Represents 'ᄤ'.
    PieupSiosPieup,
    #[cfg(feature = "archaic-korean")]
    /// Represents 'ᄥ'.
    PieupSsangSios,
    #[cfg(feature = "archaic-korean")]
    /// Represents 'ᄦ'.
    PieupSiosCieuc,
    #[cfg(feature = "archaic-korean")]
    /// Represents 'ᄧ'.
    PieupCieuc,
    #[cfg(feature = "archaic-korean")]
    /// Represents 'ᄨ'.
    PieupChieuch,
    #[cfg(feature = "archaic-korean")]
    /// Represents 'ᄩ'.
    PieupThieuth,
    #[cfg(feature = "archaic-korean")]
    /// Represents 'ᄪ'.
    PieupPhieuph,
    #[cfg(feature = "archaic-korean")]
    /// Represents 'ᄫ'.
    KapyeounPieup,
    #[cfg(feature = "archaic-korean")]
    /// Represents 'ᄬ'.
    KapyeounSsangPieup,
    #[cfg(feature = "archaic-korean")]
    /// Represents 'ᄭ'.
    SiosKiyeok,
    #[cfg(feature = "archaic-korean")]
    /// Represents 'ᄮ'.
    SiosNieun,
    #[cfg(feature = "archaic-korean")]
    /// Represents 'ᄯ'.
    SiosTikeut,
    #[cfg(feature = "archaic-korean")]
    /// Represents 'ᄰ'.
    SiosRieul,
    #[cfg(feature = "archaic-korean")]
    /// Represents 'ᄱ'.
    SiosMieum,
    #[cfg(feature = "archaic-korean")]
    /// Represents 'ᄲ'.
    SiosPieup,
    #[cfg(feature = "archaic-korean")]
    /// Represents 'ᄳ'.
    SiosPieupKiyeok,
    #[cfg(feature = "archaic-korean")]
    /// Represents 'ᄴ'.
    SiosSsangSios,
    #[cfg(feature = "archaic-korean")]
    /// Represents 'ᄵ'.
    SiosIeung,
    #[cfg(feature = "archaic-korean")]
    /// Represents 'ᄶ'.
    SiosCieuc,
    #[cfg(feature = "archaic-korean")]
    /// Represents 'ᄷ'.
    SiosChieuch,
    #[cfg(feature = "archaic-korean")]
    /// Represents 'ᄸ'.
    SiosKhieukh,
    #[cfg(feature = "archaic-korean")]
    /// Represents 'ᄹ'.
    SiosThieuth,
    #[cfg(feature = "archaic-korean")]
    /// Represents 'ᄺ'.
    SiosPhieuph,
    #[cfg(feature = "archaic-korean")]
    /// Represents 'ᄻ'.
    SiosHieuh,
    #[cfg(feature = "archaic-korean")]
    /// Represents 'ᄼ'.
    ChitueumSios,
    #[cfg(feature = "archaic-korean")]
    /// Represents 'ᄽ'.
    ChitueumSsangSios,
    #[cfg(feature = "archaic-korean")]
    /// Represents 'ᄾ'.
    CeongchieumSios,
    #[cfg(feature = "archaic-korean")]
    /// Represents 'ᄿ'.
    CeongchieumSsangSios,
    #[cfg(feature = "archaic-korean")]
    /// Represents 'ᅀ'.
    PanSios,
    #[cfg(feature = "archaic-korean")]
    /// Represents 'ᅁ'.
    IeungKiyeok,
    #[cfg(feature = "archaic-korean")]
    /// Represents 'ᅂ'.
    IeungTikeut,
    #[cfg(feature = "archaic-korean")]
    /// Represents 'ᅃ'.
    IeungMieum,
    #[cfg(feature = "archaic-korean")]
    /// Represents 'ᅄ'.
    IeungPieup,
    #[cfg(feature = "archaic-korean")]
    /// Represents 'ᅅ'.
    IeungSios,
    #[cfg(feature = "archaic-korean")]
    /// Represents 'ᅆ'.
    IeungPanSios,
    #[cfg(feature = "archaic-korean")]
    /// Represents 'ᅇ'.
    SsangIeung,
    #[cfg(feature = "archaic-korean")]
    /// Represents 'ᅈ'.
    IeungCieuc,
    #[cfg(feature = "archaic-korean")]
    /// Represents 'ᅉ'.
    IeungChieuch,
    #[cfg(feature = "archaic-korean")]
    /// Represents 'ᅊ'.
    IeungThieuth,
    #[cfg(feature = "archaic-korean")]
    /// Represents 'ᅋ'.
    IeungPhieuph,
    #[cfg(feature = "archaic-korean")]
    /// Represents 'ᅌ'.
    YesIeung,
    #[cfg(feature = "archaic-korean")]
    /// Represents 'ᅍ'.
    CieucIeung,
    #[cfg(feature = "archaic-korean")]
    /// Represents 'ᅎ'.
    ChitueumCieuc,
    #[cfg(feature = "archaic-korean")]
    /// Represents 'ᅏ'.
    ChitueumSsangCieuc,
    #[cfg(feature = "archaic-korean")]
    /// Represents 'ᅐ'.
    CeongchieumCieuc,
    #[cfg(feature = "archaic-korean")]
    /// Represents 'ᅑ'.
    CeongchieumSsangCieuc,
    #[cfg(feature = "archaic-korean")]
    /// Represents 'ᅒ'.
    ChieuchKhieukh,
    #[cfg(feature = "archaic-korean")]
    /// Represents 'ᅓ'.
    ChieuchHieuh,
    #[cfg(feature = "archaic-korean")]
    /// Represents 'ᅔ'.
    ChitueumChieuch,
    #[cfg(feature = "archaic-korean")]
    /// Represents 'ᅕ'.
    CeongchieumChieuch,
    #[cfg(feature = "archaic-korean")]
    /// Represents 'ᅖ'.
    PhieuphPieup,
    #[cfg(feature = "archaic-korean")]
    /// Represents 'ᅗ'.
    KapyeounPhieuph,
    #[cfg(feature = "archaic-korean")]
    /// Represents 'ᅘ'.
    SsangHieuh,
    #[cfg(feature = "archaic-korean")]
    /// Represents 'ᅙ'.
    YeorinHieuh,
    #[cfg(feature = "archaic-korean")]
    /// Represents 'ᅚ'.
    KiyeokTikeut,
    #[cfg(feature = "archaic-korean")]
    /// Represents 'ᅛ'.
    NieunSios,
    #[cfg(feature = "archaic-korean")]
    /// Represents 'ᅜ'.
    NieunCieuc,
    #[cfg(feature = "archaic-korean")]
    /// Represents 'ᅝ'.
    NieunHieuh,
    #[cfg(feature = "archaic-korean")]
    /// Represents 'ᅞ'.
    TikeutRieul,
    #[cfg(feature = "archaic-korean")]
    /// Represents 'ꥠ'.
    TikeutMieum = 0x9860,
    #[cfg(feature = "archaic-korean")]
    /// Represents 'ꥡ'.
    TikeutPieup,
    #[cfg(feature = "archaic-korean")]
    /// Represents 'ꥢ'.
    TikeutSios,
    #[cfg(feature = "archaic-korean")]
    /// Represents 'ꥣ'.
    TikeutCieuc,
    #[cfg(feature = "archaic-korean")]
    /// Represents 'ꥤ'.
    RieulKiyeok,
    #[cfg(feature = "archaic-korean")]
    /// Represents 'ꥥ'.
    RieulSsangKiyeok,
    #[cfg(feature = "archaic-korean")]
    /// Represents 'ꥦ'.
    RieulTikeut,
    #[cfg(feature = "archaic-korean")]
    /// Represents 'ꥧ'.
    RieulSsangTikeut,
    #[cfg(feature = "archaic-korean")]
    /// Represents 'ꥨ'.
    RieulMieum,
    #[cfg(feature = "archaic-korean")]
    /// Represents 'ꥩ'.
    RieulPieup,
    #[cfg(feature = "archaic-korean")]
    /// Represents 'ꥪ'.
    RieulSsangPieup,
    #[cfg(feature = "archaic-korean")]
    /// Represents 'ꥫ'.
    RieulKapyeounPieup,
    #[cfg(feature = "archaic-korean")]
    /// Represents 'ꥬ'.
    RieulSios,
    #[cfg(feature = "archaic-korean")]
    /// Represents 'ꥭ'.
    RieulCieuc,
    #[cfg(feature = "archaic-korean")]
    /// Represents 'ꥮ'.
    RieulKhieukh,
    #[cfg(feature = "archaic-korean")]
    /// Represents 'ꥯ'.
    MieumKiyeok,
    #[cfg(feature = "archaic-korean")]
    /// Represents 'ꥰ'.
    MieumTikeut,
    #[cfg(feature = "archaic-korean")]
    /// Represents 'ꥱ'.
    MieumSios,
    #[cfg(feature = "archaic-korean")]
    /// Represents 'ꥲ'.
    PieupSiosThieuth,
    #[cfg(feature = "archaic-korean")]
    /// Represents 'ꥳ'.
    PieupKhieukh,
    #[cfg(feature = "archaic-korean")]
    /// Represents 'ꥴ'.
    PieupHieuh,
    #[cfg(feature = "archaic-korean")]
    /// Represents 'ꥵ'.
    SsangSiosPieup,
    #[cfg(feature = "archaic-korean")]
    /// Represents 'ꥶ'.
    IeungRieul,
    #[cfg(feature = "archaic-korean")]
    /// Represents 'ꥷ'.
    IeungHieuh,
    #[cfg(feature = "archaic-korean")]
    /// Represents 'ꥸ'.
    SsangCieucHieuh,
    #[cfg(feature = "archaic-korean")]
    /// Represents 'ꥹ'.
    SsangThieuth,
    #[cfg(feature = "archaic-korean")]
    /// Represents 'ꥺ'.
    PhieuphHieuh,
    #[cfg(feature = "archaic-korean")]
    /// Represents 'ꥻ'.
    HieuhSios,
    #[cfg(feature = "archaic-korean")]
    /// Represents 'ꥼ'.
    SsangYeorinHieuh,
}
impl From<Choseong> for char {
    fn from(choseong: Choseong) -> Self {
        let codepoint = 0x1100 + choseong as u16;

        char::from_u32(codepoint as u32).unwrap()
    }
}
impl TryFrom<char> for Choseong {
    type Error = Error;

    fn try_from(character: char) -> Result<Self, Self::Error> {
        let codepoint = character as u32;

        if codepoint >= 0x1100 && codepoint <= 0x1112 {
            return Ok(Choseong::try_from(codepoint as u16 - 0x1100).unwrap());
        }
        if (codepoint >= 0x3131 && codepoint <= 0x314E)
            || (codepoint >= 0xFFA1 && codepoint <= 0xFFBE)
        {
            return Choseong::try_from(Jaeum::try_from(character).unwrap());
        }
        #[cfg(feature = "archaic-korean")]
        if (codepoint >= 0x1113 && codepoint <= 0x115E)
            || (codepoint >= 0xA960 && codepoint <= 0xA97C)
        {
            return Ok(Choseong::try_from(codepoint as u16 - 0x1100).unwrap());
        }

        Err(Error::NonJamo(character))
    }
}
impl TryFrom<Jaeum> for Choseong {
    type Error = Error;

    fn try_from(jaeum: Jaeum) -> Result<Self, Self::Error> {
        match jaeum {
            Jaeum::Kiyeok => Ok(Self::Kiyeok),
            Jaeum::SsangKiyeok => Ok(Self::SsangKiyeok),
            Jaeum::Nieun => Ok(Self::Nieun),
            Jaeum::Tikeut => Ok(Self::Tikeut),
            Jaeum::SsangTikeut => Ok(Self::SsangTikeut),
            Jaeum::Rieul => Ok(Self::Rieul),
            Jaeum::Mieum => Ok(Self::Mieum),
            Jaeum::Pieup => Ok(Self::Pieup),
            Jaeum::SsangPieup => Ok(Self::SsangPieup),
            Jaeum::Sios => Ok(Self::Sios),
            Jaeum::SsangSios => Ok(Self::SsangSios),
            Jaeum::Ieung => Ok(Self::Ieung),
            Jaeum::Cieuc => Ok(Self::Cieuc),
            Jaeum::SsangCieuc => Ok(Self::SsangCieuc),
            Jaeum::Chieuch => Ok(Self::Chieuch),
            Jaeum::Khieukh => Ok(Self::Khieukh),
            Jaeum::Thieuth => Ok(Self::Thieuth),
            Jaeum::Phieuph => Ok(Self::Phieuph),
            Jaeum::Hieuh => Ok(Self::Hieuh),
            anything_else => Err(Error::NotApplicableToChoseong(anything_else)),
        }
    }
}
impl Choseong {
    /// Holds all variants of [`Choseong`] in correct alphabetical order.
    pub const IN_ORDER: &[Choseong] = &[
        Choseong::Kiyeok,
        Choseong::SsangKiyeok,
        #[cfg(feature = "archaic-korean")]
        Choseong::KiyeokTikeut,
        Choseong::Nieun,
        #[cfg(feature = "archaic-korean")]
        Choseong::NieunKiyeok,
        #[cfg(feature = "archaic-korean")]
        Choseong::SsangNieun,
        #[cfg(feature = "archaic-korean")]
        Choseong::NieunTikeut,
        #[cfg(feature = "archaic-korean")]
        Choseong::NieunPieup,
        #[cfg(feature = "archaic-korean")]
        Choseong::NieunSios,
        #[cfg(feature = "archaic-korean")]
        Choseong::NieunCieuc,
        #[cfg(feature = "archaic-korean")]
        Choseong::NieunHieuh,
        Choseong::Tikeut,
        #[cfg(feature = "archaic-korean")]
        Choseong::TikeutKiyeok,
        Choseong::SsangTikeut,
        #[cfg(feature = "archaic-korean")]
        Choseong::TikeutRieul,
        #[cfg(feature = "archaic-korean")]
        Choseong::TikeutMieum,
        #[cfg(feature = "archaic-korean")]
        Choseong::TikeutPieup,
        #[cfg(feature = "archaic-korean")]
        Choseong::TikeutSios,
        #[cfg(feature = "archaic-korean")]
        Choseong::TikeutCieuc,
        Choseong::Rieul,
        #[cfg(feature = "archaic-korean")]
        Choseong::RieulKiyeok,
        #[cfg(feature = "archaic-korean")]
        Choseong::RieulSsangKiyeok,
        #[cfg(feature = "archaic-korean")]
        Choseong::RieulNieun,
        #[cfg(feature = "archaic-korean")]
        Choseong::RieulTikeut,
        #[cfg(feature = "archaic-korean")]
        Choseong::RieulSsangTikeut,
        #[cfg(feature = "archaic-korean")]
        Choseong::SsangRieul,
        #[cfg(feature = "archaic-korean")]
        Choseong::RieulMieum,
        #[cfg(feature = "archaic-korean")]
        Choseong::RieulPieup,
        #[cfg(feature = "archaic-korean")]
        Choseong::RieulSsangPieup,
        #[cfg(feature = "archaic-korean")]
        Choseong::RieulKapyeounPieup,
        #[cfg(feature = "archaic-korean")]
        Choseong::RieulSios,
        #[cfg(feature = "archaic-korean")]
        Choseong::RieulCieuc,
        #[cfg(feature = "archaic-korean")]
        Choseong::RieulKhieukh,
        #[cfg(feature = "archaic-korean")]
        Choseong::RieulHieuh,
        #[cfg(feature = "archaic-korean")]
        Choseong::KapyeounRieul,
        Choseong::Mieum,
        #[cfg(feature = "archaic-korean")]
        Choseong::MieumKiyeok,
        #[cfg(feature = "archaic-korean")]
        Choseong::MieumTikeut,
        #[cfg(feature = "archaic-korean")]
        Choseong::MieumPieup,
        #[cfg(feature = "archaic-korean")]
        Choseong::MieumSios,
        #[cfg(feature = "archaic-korean")]
        Choseong::KapyeounMieum,
        Choseong::Pieup,
        #[cfg(feature = "archaic-korean")]
        Choseong::PieupKiyeok,
        #[cfg(feature = "archaic-korean")]
        Choseong::PieupNieun,
        #[cfg(feature = "archaic-korean")]
        Choseong::PieupTikeut,
        Choseong::SsangPieup,
        #[cfg(feature = "archaic-korean")]
        Choseong::PieupSios,
        #[cfg(feature = "archaic-korean")]
        Choseong::PieupSiosKiyeok,
        #[cfg(feature = "archaic-korean")]
        Choseong::PieupSiosTikeut,
        #[cfg(feature = "archaic-korean")]
        Choseong::PieupSiosPieup,
        #[cfg(feature = "archaic-korean")]
        Choseong::PieupSsangSios,
        #[cfg(feature = "archaic-korean")]
        Choseong::PieupSiosCieuc,
        #[cfg(feature = "archaic-korean")]
        Choseong::PieupSiosThieuth,
        #[cfg(feature = "archaic-korean")]
        Choseong::PieupCieuc,
        #[cfg(feature = "archaic-korean")]
        Choseong::PieupChieuch,
        #[cfg(feature = "archaic-korean")]
        Choseong::PieupKhieukh,
        #[cfg(feature = "archaic-korean")]
        Choseong::PieupThieuth,
        #[cfg(feature = "archaic-korean")]
        Choseong::PieupPhieuph,
        #[cfg(feature = "archaic-korean")]
        Choseong::PieupHieuh,
        #[cfg(feature = "archaic-korean")]
        Choseong::KapyeounPieup,
        #[cfg(feature = "archaic-korean")]
        Choseong::KapyeounSsangPieup,
        Choseong::Sios,
        #[cfg(feature = "archaic-korean")]
        Choseong::SiosKiyeok,
        #[cfg(feature = "archaic-korean")]
        Choseong::SiosNieun,
        #[cfg(feature = "archaic-korean")]
        Choseong::SiosTikeut,
        #[cfg(feature = "archaic-korean")]
        Choseong::SiosRieul,
        #[cfg(feature = "archaic-korean")]
        Choseong::SiosMieum,
        #[cfg(feature = "archaic-korean")]
        Choseong::SiosPieup,
        #[cfg(feature = "archaic-korean")]
        Choseong::SiosPieupKiyeok,
        Choseong::SsangSios,
        #[cfg(feature = "archaic-korean")]
        Choseong::SsangSiosPieup,
        #[cfg(feature = "archaic-korean")]
        Choseong::SiosSsangSios,
        #[cfg(feature = "archaic-korean")]
        Choseong::SiosIeung,
        #[cfg(feature = "archaic-korean")]
        Choseong::SiosCieuc,
        #[cfg(feature = "archaic-korean")]
        Choseong::SiosChieuch,
        #[cfg(feature = "archaic-korean")]
        Choseong::SiosKhieukh,
        #[cfg(feature = "archaic-korean")]
        Choseong::SiosThieuth,
        #[cfg(feature = "archaic-korean")]
        Choseong::SiosPhieuph,
        #[cfg(feature = "archaic-korean")]
        Choseong::SiosHieuh,
        #[cfg(feature = "archaic-korean")]
        Choseong::ChitueumSios,
        #[cfg(feature = "archaic-korean")]
        Choseong::ChitueumSsangSios,
        #[cfg(feature = "archaic-korean")]
        Choseong::CeongchieumSios,
        #[cfg(feature = "archaic-korean")]
        Choseong::CeongchieumSsangSios,
        #[cfg(feature = "archaic-korean")]
        Choseong::PanSios,
        Choseong::Ieung,
        #[cfg(feature = "archaic-korean")]
        Choseong::IeungKiyeok,
        #[cfg(feature = "archaic-korean")]
        Choseong::IeungTikeut,
        #[cfg(feature = "archaic-korean")]
        Choseong::IeungRieul,
        #[cfg(feature = "archaic-korean")]
        Choseong::IeungMieum,
        #[cfg(feature = "archaic-korean")]
        Choseong::IeungPieup,
        #[cfg(feature = "archaic-korean")]
        Choseong::IeungSios,
        #[cfg(feature = "archaic-korean")]
        Choseong::IeungPanSios,
        #[cfg(feature = "archaic-korean")]
        Choseong::SsangIeung,
        #[cfg(feature = "archaic-korean")]
        Choseong::IeungCieuc,
        #[cfg(feature = "archaic-korean")]
        Choseong::IeungChieuch,
        #[cfg(feature = "archaic-korean")]
        Choseong::IeungThieuth,
        #[cfg(feature = "archaic-korean")]
        Choseong::IeungPhieuph,
        #[cfg(feature = "archaic-korean")]
        Choseong::IeungHieuh,
        #[cfg(feature = "archaic-korean")]
        Choseong::YesIeung,
        Choseong::Cieuc,
        #[cfg(feature = "archaic-korean")]
        Choseong::CieucIeung,
        Choseong::SsangCieuc,
        #[cfg(feature = "archaic-korean")]
        Choseong::SsangCieucHieuh,
        #[cfg(feature = "archaic-korean")]
        Choseong::ChitueumCieuc,
        #[cfg(feature = "archaic-korean")]
        Choseong::ChitueumSsangCieuc,
        #[cfg(feature = "archaic-korean")]
        Choseong::CeongchieumCieuc,
        #[cfg(feature = "archaic-korean")]
        Choseong::CeongchieumSsangCieuc,
        Choseong::Chieuch,
        #[cfg(feature = "archaic-korean")]
        Choseong::ChieuchKhieukh,
        #[cfg(feature = "archaic-korean")]
        Choseong::ChieuchHieuh,
        #[cfg(feature = "archaic-korean")]
        Choseong::ChitueumChieuch,
        #[cfg(feature = "archaic-korean")]
        Choseong::CeongchieumChieuch,
        Choseong::Khieukh,
        Choseong::Thieuth,
        #[cfg(feature = "archaic-korean")]
        Choseong::SsangThieuth,
        Choseong::Phieuph,
        #[cfg(feature = "archaic-korean")]
        Choseong::PhieuphPieup,
        #[cfg(feature = "archaic-korean")]
        Choseong::PhieuphHieuh,
        #[cfg(feature = "archaic-korean")]
        Choseong::KapyeounPhieuph,
        Choseong::Hieuh,
        #[cfg(feature = "archaic-korean")]
        Choseong::HieuhSios,
        #[cfg(feature = "archaic-korean")]
        Choseong::SsangHieuh,
        #[cfg(feature = "archaic-korean")]
        Choseong::YeorinHieuh,
        #[cfg(feature = "archaic-korean")]
        Choseong::SsangYeorinHieuh,
    ];
}
