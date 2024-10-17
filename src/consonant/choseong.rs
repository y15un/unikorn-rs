use crate::{
    consonant::{HalfwidthJaeum, Jaeum, Jongseong},
    Error,
};
use num_enum::{IntoPrimitive, TryFromPrimitive};
use std::{
    convert::TryFrom,
    fmt::{Display, Formatter, Result as FmtResult},
};

/// A set of consonants valid as initial consonant (초성, Choseong).
#[derive(Clone, Copy, Debug, Eq, IntoPrimitive, Ord, PartialEq, PartialOrd, TryFromPrimitive)]
#[repr(u32)]
pub enum Choseong {
    /// Represents initial consonant `ᄀ` (U+1100, Hangul Choseong Kiyeok)
    Kiyeok = 0x1100,
    /// Represents initial consonant `ᄁ` (U+1101, Hangul Choseong SsangKiyeok)
    SsangKiyeok,
    /// Represents initial consonant `ᄂ` (U+1102, Hangul Choseong Nieun)
    Nieun,
    /// Represents initial consonant `ᄃ` (U+1103, Hangul Choseong Tikeut)
    Tikeut,
    /// Represents initial consonant `ᄄ` (U+1104, Hangul Choseong SsangTikeut)
    SsangTikeut,
    /// Represents initial consonant `ᄅ` (U+1105, Hangul Choseong Rieul)
    Rieul,
    /// Represents initial consonant `ᄆ` (U+1106, Hangul Choseong Mieum)
    Mieum,
    /// Represents initial consonant `ᄇ` (U+1107, Hangul Choseong Pieup)
    Pieup,
    /// Represents initial consonant `ᄈ` (U+1108, Hangul Choseong SsangPieup)
    SsangPieup,
    /// Represents initial consonant `ᄉ` (U+1109, Hangul Choseong Sios)
    Sios,
    /// Represents initial consonant `ᄊ` (U+110A, Hangul Choseong SsangSios)
    SsangSios,
    /// Represents initial consonant `ᄋ` (U+110B, Hangul Choseong Ieung)
    Ieung,
    /// Represents initial consonant `ᄌ` (U+110C, Hangul Choseong Cieuc)
    Cieuc,
    /// Represents initial consonant `ᄍ` (U+110D, Hangul Choseong SsangCieuc)
    SsangCieuc,
    /// Represents initial consonant `ᄎ` (U+110E, Hangul Choseong Chieuch)
    Chieuch,
    /// Represents initial consonant `ᄏ` (U+110F, Hangul Choseong Khieukh)
    Khieukh,
    /// Represents initial consonant `ᄐ` (U+1110, Hangul Choseong Thieuth)
    Thieuth,
    /// Represents initial consonant `ᄑ` (U+1111, Hangul Choseong Phieuph)
    Phieuph,
    /// Represents initial consonant `ᄒ` (U+1112, Hangul Choseong Hieuh)
    Hieuh,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* initial consonant `ᄓ` (U+1113, Hangul Choseong Nieun-Kiyeok)
    NieunKiyeok,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* initial consonant `ᄔ` (U+1114, Hangul Choseong SsangNieun)
    SsangNieun,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* initial consonant `ᄕ` (U+1115, Hangul Choseong Nieun-Tikeut)
    NieunTikeut,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* initial consonant `ᄖ` (U+1116, Hangul Choseong Nieun-Pieup)
    NieunPieup,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* initial consonant `ᄗ` (U+1117, Hangul Choseong Tikeut-Kiyeok)
    TikeutKiyeok,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* initial consonant `ᄘ` (U+1118, Hangul Choseong Rieul-Nieun)
    RieulNieun,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* initial consonant `ᄙ` (U+1119, Hangul Choseong SsangRieul)
    SsangRieul,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* initial consonant `ᄚ` (U+111A, Hangul Choseong Rieul-Hieuh)
    RieulHieuh,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* initial consonant `ᄛ` (U+111B, Hangul Choseong KapyeounRieul)
    KapyeounRieul,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* initial consonant `ᄜ` (U+111C, Hangul Choseong Mieum-Pieup)
    MieumPieup,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* initial consonant `ᄝ` (U+111D, Hangul Choseong KapyeounMieum)
    KapyeounMieum,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* initial consonant `ᄞ` (U+111E, Hangul Choseong Pieup-Kiyeok)
    PieupKiyeok,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* initial consonant `ᄟ` (U+111F, Hangul Choseong Pieup-Nieun)
    PieupNieun,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* initial consonant `ᄠ` (U+1120, Hangul Choseong Pieup-Tikeut)
    PieupTikeut,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* initial consonant `ᄡ` (U+1121, Hangul Choseong Pieup-Sios)
    PieupSios,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* initial consonant `ᄢ` (U+1122, Hangul Choseong Pieup-Sios-Kiyeok)
    PieupSiosKiyeok,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* initial consonant `ᄣ` (U+1123, Hangul Choseong Pieup-Sios-Tikeut)
    PieupSiosTikeut,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* initial consonant `ᄤ` (U+1124, Hangul Choseong Pieup-Sios-Pieup)
    PieupSiosPieup,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* initial consonant `ᄥ` (U+1125, Hangul Choseong Pieup-SsangSios)
    PieupSsangSios,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* initial consonant `ᄦ` (U+1126, Hangul Choseong Pieup-Sios-Cieuc)
    PieupSiosCieuc,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* initial consonant `ᄧ` (U+1127, Hangul Choseong Pieup-Cieuc)
    PieupCieuc,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* initial consonant `ᄨ` (U+1128, Hangul Choseong Pieup-Chieuch)
    PieupChieuch,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* initial consonant `ᄩ` (U+1129, Hangul Choseong Pieup-Thieuth)
    PieupThieuth,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* initial consonant `ᄪ` (U+112A, Hangul Choseong Pieup-Phieuph)
    PieupPhieuph,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* initial consonant `ᄫ` (U+112B, Hangul Choseong KapyeounPieup)
    KapyeounPieup,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* initial consonant `ᄬ` (U+112C, Hangul Choseong KapyeounSsangPieup)
    KapyeounSsangPieup,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* initial consonant `ᄭ` (U+112D, Hangul Choseong Sios-Kiyeok)
    SiosKiyeok,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* initial consonant `ᄮ` (U+112E, Hangul Choseong Sios-Nieun)
    SiosNieun,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* initial consonant `ᄯ` (U+112F, Hangul Choseong Sios-Tikeut)
    SiosTikeut,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* initial consonant `ᄰ` (U+1130, Hangul Choseong Sios-Rieul)
    SiosRieul,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* initial consonant `ᄱ` (U+1131, Hangul Choseong Sios-Mieum)
    SiosMieum,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* initial consonant `ᄲ` (U+1132, Hangul Choseong Sios-Pieup)
    SiosPieup,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* initial consonant `ᄳ` (U+1133, Hangul Choseong Sios-Pieup-Kiyeok)
    SiosPieupKiyeok,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* initial consonant `ᄴ` (U+1134, Hangul Choseong Sios-SsangSios)
    SiosSsangSios,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* initial consonant `ᄵ` (U+1135, Hangul Choseong Sios-Ieung)
    SiosIeung,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* initial consonant `ᄶ` (U+1136, Hangul Choseong Sios-Cieuc)
    SiosCieuc,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* initial consonant `ᄷ` (U+1137, Hangul Choseong Sios-Chieuch)
    SiosChieuch,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* initial consonant `ᄸ` (U+1138, Hangul Choseong Sios-Khieukh)
    SiosKhieukh,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* initial consonant `ᄹ` (U+1139, Hangul Choseong Sios-Thieuth)
    SiosThieuth,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* initial consonant `ᄺ` (U+113A, Hangul Choseong Sios-Phieuph)
    SiosPhieuph,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* initial consonant `ᄻ` (U+113B, Hangul Choseong Sios-Hieuh)
    SiosHieuh,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* initial consonant `ᄼ` (U+113C, Hangul Choseong ChitueumSios)
    ChitueumSios,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* initial consonant `ᄽ` (U+113D, Hangul Choseong ChitueumSsangSios)
    ChitueumSsangSios,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* initial consonant `ᄾ` (U+113E, Hangul Choseong CeongchieumSios)
    CeongchieumSios,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* initial consonant `ᄿ` (U+113F, Hangul Choseong CeongchieumSsangSios)
    CeongchieumSsangSios,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* initial consonant `ᅀ` (U+1140, Hangul Choseong PanSios)
    PanSios,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* initial consonant `ᅁ` (U+1141, Hangul Choseong Ieung-Kiyeok)
    IeungKiyeok,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* initial consonant `ᅂ` (U+1142, Hangul Choseong Ieung-Tikeut)
    IeungTikeut,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* initial consonant `ᅃ` (U+1143, Hangul Choseong Ieung-Mieum)
    IeungMieum,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* initial consonant `ᅄ` (U+1144, Hangul Choseong Ieung-Pieup)
    IeungPieup,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* initial consonant `ᅅ` (U+1145, Hangul Choseong Ieung-Sios)
    IeungSios,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* initial consonant `ᅆ` (U+1146, Hangul Choseong Ieung-PanSios)
    IeungPanSios,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* initial consonant `ᅇ` (U+1147, Hangul Choseong SsangIeung)
    SsangIeung,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* initial consonant `ᅈ` (U+1148, Hangul Choseong Ieung-Cieuc)
    IeungCieuc,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* initial consonant `ᅉ` (U+1149, Hangul Choseong Ieung-Chieuch)
    IeungChieuch,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* initial consonant `ᅊ` (U+114A, Hangul Choseong Ieung-Thieuth)
    IeungThieuth,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* initial consonant `ᅋ` (U+114B, Hangul Choseong Ieung-Phieuph)
    IeungPhieuph,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* initial consonant `ᅌ` (U+114C, Hangul Choseong YesIeung)
    YesIeung,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* initial consonant `ᅍ` (U+114D, Hangul Choseong Cieuc-Ieung)
    CieucIeung,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* initial consonant `ᅎ` (U+114E, Hangul Choseong ChitueumCieuc)
    ChitueumCieuc,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* initial consonant `ᅏ` (U+114F, Hangul Choseong ChitueumSsangCieuc)
    ChitueumSsangCieuc,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* initial consonant `ᅐ` (U+1150, Hangul Choseong CeongchieumCieuc)
    CeongchieumCieuc,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* initial consonant `ᅑ` (U+1151, Hangul Choseong CeongchieumSsangCieuc)
    CeongchieumSsangCieuc,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* initial consonant `ᅒ` (U+1152, Hangul Choseong Chieuch-Khieukh)
    ChieuchKhieukh,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* initial consonant `ᅓ` (U+1153, Hangul Choseong Chieuch-Hieuh)
    ChieuchHieuh,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* initial consonant `ᅔ` (U+1154, Hangul Choseong ChitueumChieuch)
    ChitueumChieuch,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* initial consonant `ᅕ` (U+1155, Hangul Choseong CeongchieumChieuch)
    CeongchieumChieuch,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* initial consonant `ᅖ` (U+1156, Hangul Choseong Phieuph-Pieup)
    PhieuphPieup,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* initial consonant `ᅗ` (U+1157, Hangul Choseong KapyeounPhieuph)
    KapyeounPhieuph,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* initial consonant `ᅘ` (U+1158, Hangul Choseong SsangHieuh)
    SsangHieuh,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* initial consonant `ᅙ` (U+1159, Hangul Choseong YeorinHieuh)
    YeorinHieuh,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* initial consonant `ᅚ` (U+115A, Hangul Choseong Kiyeok-Tikeut)
    KiyeokTikeut,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* initial consonant `ᅛ` (U+115B, Hangul Choseong Nieun-Sios)
    NieunSios,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* initial consonant `ᅜ` (U+115C, Hangul Choseong Nieun-Cieuc)
    NieunCieuc,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* initial consonant `ᅝ` (U+115D, Hangul Choseong Nieun-Hieuh)
    NieunHieuh,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* initial consonant `ᅞ` (U+115E, Hangul Choseong Tikeut-Rieul)
    TikeutRieul,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* initial consonant `ꥠ` (U+A960, Hangul Choseong Tikeut-Mieum)
    TikeutMieum = 0xA960,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* initial consonant `ꥡ` (U+A961, Hangul Choseong Tikeut-Pieup)
    TikeutPieup,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* initial consonant `ꥢ` (U+A962, Hangul Choseong Tikeut-Sios)
    TikeutSios,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* initial consonant `ꥣ` (U+A963, Hangul Choseong Tikeut-Cieuc)
    TikeutCieuc,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* initial consonant `ꥤ` (U+A964, Hangul Choseong Rieul-Kiyeok)
    RieulKiyeok,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* initial consonant `ꥥ` (U+A965, Hangul Choseong Rieul-SsangKiyeok)
    RieulSsangKiyeok,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* initial consonant `ꥦ` (U+A966, Hangul Choseong Rieul-Tikeut)
    RieulTikeut,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* initial consonant `ꥧ` (U+A967, Hangul Choseong Rieul-SsangTikeut)
    RieulSsangTikeut,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* initial consonant `ꥨ` (U+A968, Hangul Choseong Rieul-Mieum)
    RieulMieum,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* initial consonant `ꥩ` (U+A969, Hangul Choseong Rieul-Pieup)
    RieulPieup,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* initial consonant `ꥪ` (U+A96A, Hangul Choseong Rieul-SsangPieup)
    RieulSsangPieup,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* initial consonant `ꥫ` (U+A96B, Hangul Choseong Rieul-KapyeounPieup)
    RieulKapyeounPieup,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* initial consonant `ꥬ` (U+A96C, Hangul Choseong Rieul-Sios)
    RieulSios,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* initial consonant `ꥭ` (U+A96D, Hangul Choseong Rieul-Cieuc)
    RieulCieuc,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* initial consonant `ꥮ` (U+A96E, Hangul Choseong Rieul-Khieukh)
    RieulKhieukh,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* initial consonant `ꥯ` (U+A96F, Hangul Choseong Mieum-Kiyeok)
    MieumKiyeok,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* initial consonant `ꥰ` (U+A970, Hangul Choseong Mieum-Tikeut)
    MieumTikeut,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* initial consonant `ꥱ` (U+A971, Hangul Choseong Mieum-Sios)
    MieumSios,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* initial consonant `ꥲ` (U+A972, Hangul Choseong Pieup-Sios-Thieuth)
    PieupSiosThieuth,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* initial consonant `ꥳ` (U+A973, Hangul Choseong Pieup-Khieukh)
    PieupKhieukh,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* initial consonant `ꥴ` (U+A974, Hangul Choseong Pieup-Hieuh)
    PieupHieuh,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* initial consonant `ꥵ` (U+A975, Hangul Choseong SsangSios-Pieup)
    SsangSiosPieup,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* initial consonant `ꥶ` (U+A976, Hangul Choseong Ieung-Rieul)
    IeungRieul,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* initial consonant `ꥷ` (U+A977, Hangul Choseong Ieung-Hieuh)
    IeungHieuh,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* initial consonant `ꥸ` (U+A978, Hangul Choseong SsangCieuc-Hieuh)
    SsangCieucHieuh,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* initial consonant `ꥹ` (U+A979, Hangul Choseong SsangThieuth)
    SsangThieuth,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* initial consonant `ꥺ` (U+A97A, Hangul Choseong Phieuph-Hieuh)
    PhieuphHieuh,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* initial consonant `ꥻ` (U+A97B, Hangul Choseong Hieuh-Sios)
    HieuhSios,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* initial consonant `ꥼ` (U+A97C, Hangul Choseong SsangYeorinHieuh)
    SsangYeorinHieuh,
}
impl Display for Choseong {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", char::from(*self))
    }
}
impl From<Choseong> for char {
    fn from(value: Choseong) -> Self {
        // guaranteed to not fail within BMP
        char::from_u32(value as u32).unwrap()
    }
}
impl TryFrom<char> for Choseong {
    type Error = Error;

    /// Tries to convert a [`char`] into [`Choseong`].
    ///
    /// # Direct Conversion
    /// The [`char`] given will be tested against the following range(s):
    ///
    /// * Hangul Jamo / Initial consonants (U+1100--U+1112)
    /// * Hangul Jamo / Old initial consonants (U+1113--U+115E)
    /// * Hangul Jamo Extended-A / Old initial consonants (U+A960--U+A97C)
    ///
    /// The latter two ranges are considered valid if and only if `archaic-korean` feature is enabled.
    ///
    /// # Indirect Conversion
    /// ## Via [`Jaeum`] or [`HalfwidthJaeum`]
    /// For the following range(s), the [`char`] given will be tried converting into [`Jaeum`] or [`HalfwidthJaeum`] accordingly first, then subsequently into [`Choseong`]:
    ///
    /// * Hangul Compatibility Jamo / Consonant letters (U+3131--U+314E)
    /// * Halfwidth and Fullwidth Forms / Halfwidth Hangul variants (U+FFA1--U+FFBE)
    /// * Hangul Compatibility Jamo / Old consonant letters (U+3165--U+3186)
    ///
    /// The latter one range is considered valid if and only if `archaic-korean` feature is enabled.
    ///
    /// ## Via [`Jongseong`]
    /// For the following rnage(s), the [`char`] given will be tried converting into [`Jongseong`] first, then subsequently into [`Choseong`]:
    ///
    /// * Hangul Jamo / Final consonants (U+11A8--U+11C2)
    /// * Hangul Jamo / Old final consonants (U+11C3--U+11FF)
    /// * Hangul Jamo Extended-B / Old final consonants (U+D7CB--U+D7FB)
    ///
    /// The latter two ranges are considered valid if and only if `archaic-korean` feature is enabled.
    ///
    /// # Errors
    /// * [`Error::NonChoseongTryFromChar`]: the [`char`] given is not valid as initial consonant.
    /// * [`Error::NonJaeumTryFromChar`]: the [`char`] given is not a valid consonant.
    /// * [`Error::NonKoreanTryFromChar`]: the [`char`] given is not a valid Korean letter.
    fn try_from(value: char) -> Result<Self, Self::Error> {
        let code_point = value as u32;
        let nacho = Err(Error::NonChoseongTryFromChar(value));

        match code_point {
            // initial consonants (self)
            0x1100..=0x1112 => return Ok(Self::try_from(code_point).unwrap()),
            // vowels
            0x1161..=0x1175
            | 0x314F..=0x3163
            | 0xFFC2..=0xFFC7
            | 0xFFCA..=0xFFCF
            | 0xFFD2..=0xFFD7
            | 0xFFDA..=0xFFDC => return Err(Error::NonJaeumTryFromChar(value)),
            // final consonants
            0x11A8..=0x11C2 => {
                return Self::try_from(Jongseong::try_from(code_point).unwrap()).or(nacho)
            }
            // generic consonants
            0x3131..=0x314E => {
                return Self::try_from(Jaeum::try_from(code_point).unwrap()).or(nacho)
            }
            // halfwidth consonants
            0xFFA1..=0xFFBE => {
                return Self::try_from(HalfwidthJaeum::try_from(code_point).unwrap()).or(nacho)
            }
            _ => {}
        }

        #[cfg(feature = "archaic-korean")]
        match code_point {
            // old initial consonants (self)
            0x1113..=0x115E | 0xA960..=0xA97C => return Ok(Self::try_from(code_point).unwrap()),
            // old vowels
            0x1176..=0x11A7 | 0x3187..=0x318E | 0xD7B0..=0xD7C6 => {
                return Err(Error::NonJaeumTryFromChar(value))
            }
            // old final consonants
            0x11C3..=0x11FF | 0xD7CB..=0xD7FB => {
                return Self::try_from(Jongseong::try_from(code_point).unwrap()).or(nacho)
            }
            // old generic consonants
            0x3165..=0x3186 => {
                return Self::try_from(Jaeum::try_from(code_point).unwrap()).or(nacho)
            }
            _ => {}
        }

        Err(Error::NonKoreanTryFromChar(value))
    }
}
impl TryFrom<HalfwidthJaeum> for Choseong {
    type Error = Error;

    /// Tries to convert a [`HalfwidthJaeum`] into [`Choseong`].
    ///
    /// # Errors
    /// * [`Error::NonChoseongTryFromHalfwidthJaeum`]: the [`HalfwidthJaeum`] given is not valid as initial consonant.
    fn try_from(value: HalfwidthJaeum) -> Result<Self, Self::Error> {
        // TODO: consider switching to bst; but i'm not very sure of performance boost it'll yield.
        match value {
            HalfwidthJaeum::Kiyeok => Ok(Self::Kiyeok),
            HalfwidthJaeum::SsangKiyeok => Ok(Self::SsangKiyeok),
            HalfwidthJaeum::Nieun => Ok(Self::Nieun),
            #[cfg(feature = "archaic-korean")]
            HalfwidthJaeum::NieunCieuc => Ok(Self::NieunCieuc),
            #[cfg(feature = "archaic-korean")]
            HalfwidthJaeum::NieunHieuh => Ok(Self::NieunHieuh),
            HalfwidthJaeum::Tikeut => Ok(Self::Tikeut),
            HalfwidthJaeum::SsangTikeut => Ok(Self::SsangTikeut),
            HalfwidthJaeum::Rieul => Ok(Self::Rieul),
            #[cfg(feature = "archaic-korean")]
            HalfwidthJaeum::RieulKiyeok => Ok(Self::RieulKiyeok),
            #[cfg(feature = "archaic-korean")]
            HalfwidthJaeum::RieulMieum => Ok(Self::RieulMieum),
            #[cfg(feature = "archaic-korean")]
            HalfwidthJaeum::RieulPieup => Ok(Self::RieulPieup),
            #[cfg(feature = "archaic-korean")]
            HalfwidthJaeum::RieulSios => Ok(Self::RieulSios),
            #[cfg(feature = "archaic-korean")]
            HalfwidthJaeum::RieulHieuh => Ok(Self::RieulHieuh),
            HalfwidthJaeum::Mieum => Ok(Self::Mieum),
            HalfwidthJaeum::Pieup => Ok(Self::Pieup),
            HalfwidthJaeum::SsangPieup => Ok(Self::SsangPieup),
            #[cfg(feature = "archaic-korean")]
            HalfwidthJaeum::PieupSios => Ok(Self::PieupSios),
            HalfwidthJaeum::Sios => Ok(Self::Sios),
            HalfwidthJaeum::SsangSios => Ok(Self::SsangSios),
            HalfwidthJaeum::Ieung => Ok(Self::Ieung),
            HalfwidthJaeum::Cieuc => Ok(Self::Cieuc),
            HalfwidthJaeum::SsangCieuc => Ok(Self::SsangCieuc),
            HalfwidthJaeum::Chieuch => Ok(Self::Chieuch),
            HalfwidthJaeum::Khieukh => Ok(Self::Khieukh),
            HalfwidthJaeum::Thieuth => Ok(Self::Thieuth),
            HalfwidthJaeum::Phieuph => Ok(Self::Phieuph),
            HalfwidthJaeum::Hieuh => Ok(Self::Hieuh),
            _ => Err(Error::NonChoseongTryFromHalfwidthJaeum(value)),
        }
    }
}
impl TryFrom<Jaeum> for Choseong {
    type Error = Error;

    /// Tries to convert a [`Jaeum`] into [`Choseong`].
    ///
    /// # Errors
    /// * [`Error::NonChoseongTryFromJaeum`]: the [`Jaeum`] given is not valid as initial consonant.
    fn try_from(value: Jaeum) -> Result<Self, Self::Error> {
        // TODO: consider switching to bst; but i'm not very sure of performance boost it'll yield.
        match value {
            Jaeum::Kiyeok => Ok(Self::Kiyeok),
            Jaeum::SsangKiyeok => Ok(Self::SsangKiyeok),
            Jaeum::Nieun => Ok(Self::Nieun),
            #[cfg(feature = "archaic-korean")]
            Jaeum::NieunCieuc => Ok(Self::NieunCieuc),
            #[cfg(feature = "archaic-korean")]
            Jaeum::NieunHieuh => Ok(Self::NieunHieuh),
            Jaeum::Tikeut => Ok(Self::Tikeut),
            Jaeum::SsangTikeut => Ok(Self::SsangTikeut),
            Jaeum::Rieul => Ok(Self::Rieul),
            #[cfg(feature = "archaic-korean")]
            Jaeum::RieulKiyeok => Ok(Self::RieulKiyeok),
            #[cfg(feature = "archaic-korean")]
            Jaeum::RieulMieum => Ok(Self::RieulMieum),
            #[cfg(feature = "archaic-korean")]
            Jaeum::RieulPieup => Ok(Self::RieulPieup),
            #[cfg(feature = "archaic-korean")]
            Jaeum::RieulSios => Ok(Self::RieulSios),
            #[cfg(feature = "archaic-korean")]
            Jaeum::RieulHieuh => Ok(Self::RieulHieuh),
            Jaeum::Mieum => Ok(Self::Mieum),
            Jaeum::Pieup => Ok(Self::Pieup),
            Jaeum::SsangPieup => Ok(Self::SsangPieup),
            #[cfg(feature = "archaic-korean")]
            Jaeum::PieupSios => Ok(Self::PieupSios),
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
            #[cfg(feature = "archaic-korean")]
            Jaeum::SsangNieun => Ok(Self::SsangNieun),
            #[cfg(feature = "archaic-korean")]
            Jaeum::NieunTikeut => Ok(Self::NieunTikeut),
            #[cfg(feature = "archaic-korean")]
            Jaeum::NieunSios => Ok(Self::NieunSios),
            #[cfg(feature = "archaic-korean")]
            Jaeum::RieulTikeut => Ok(Self::RieulTikeut),
            #[cfg(feature = "archaic-korean")]
            Jaeum::MieumPieup => Ok(Self::MieumPieup),
            #[cfg(feature = "archaic-korean")]
            Jaeum::MieumSios => Ok(Self::MieumSios),
            #[cfg(feature = "archaic-korean")]
            Jaeum::KapyeounMieum => Ok(Self::KapyeounMieum),
            #[cfg(feature = "archaic-korean")]
            Jaeum::PieupKiyeok => Ok(Self::PieupKiyeok),
            #[cfg(feature = "archaic-korean")]
            Jaeum::PieupTikeut => Ok(Self::PieupTikeut),
            #[cfg(feature = "archaic-korean")]
            Jaeum::PieupSiosKiyeok => Ok(Self::PieupSiosKiyeok),
            #[cfg(feature = "archaic-korean")]
            Jaeum::PieupSiosTikeut => Ok(Self::PieupSiosTikeut),
            #[cfg(feature = "archaic-korean")]
            Jaeum::PieupCieuc => Ok(Self::PieupCieuc),
            #[cfg(feature = "archaic-korean")]
            Jaeum::PieupThieuth => Ok(Self::PieupThieuth),
            #[cfg(feature = "archaic-korean")]
            Jaeum::KapyeounPieup => Ok(Self::KapyeounPieup),
            #[cfg(feature = "archaic-korean")]
            Jaeum::KapyeounSsangPieup => Ok(Self::KapyeounSsangPieup),
            #[cfg(feature = "archaic-korean")]
            Jaeum::SiosKiyeok => Ok(Self::SiosKiyeok),
            #[cfg(feature = "archaic-korean")]
            Jaeum::SiosNieun => Ok(Self::SiosNieun),
            #[cfg(feature = "archaic-korean")]
            Jaeum::SiosTikeut => Ok(Self::SiosTikeut),
            #[cfg(feature = "archaic-korean")]
            Jaeum::SiosPieup => Ok(Self::SiosPieup),
            #[cfg(feature = "archaic-korean")]
            Jaeum::SiosCieuc => Ok(Self::SiosCieuc),
            #[cfg(feature = "archaic-korean")]
            Jaeum::PanSios => Ok(Self::PanSios),
            #[cfg(feature = "archaic-korean")]
            Jaeum::SsangIeung => Ok(Self::SsangIeung),
            #[cfg(feature = "archaic-korean")]
            Jaeum::YesIeung => Ok(Self::YesIeung),
            #[cfg(feature = "archaic-korean")]
            Jaeum::KapyeounPhieuph => Ok(Self::KapyeounPhieuph),
            #[cfg(feature = "archaic-korean")]
            Jaeum::SsangHieuh => Ok(Self::SsangHieuh),
            #[cfg(feature = "archaic-korean")]
            Jaeum::YeorinHieuh => Ok(Self::YeorinHieuh),
            _ => Err(Error::NonChoseongTryFromJaeum(value)),
        }
    }
}
impl TryFrom<Jongseong> for Choseong {
    type Error = Error;

    /// Tries to convert a [`Jongseong`] into [`Choseong`].
    ///
    /// # Errors
    /// * [`Error::NonChoseongTryFromJongseong`]: the [`Jongseong`] given is not valid as initial consonant.
    fn try_from(value: Jongseong) -> Result<Self, Self::Error> {
        // TODO: consider switching to bst; but i'm not very sure of performance boost it'll yield.
        match value {
            Jongseong::Kiyeok => Ok(Self::Kiyeok),
            Jongseong::SsangKiyeok => Ok(Self::SsangKiyeok),
            Jongseong::Nieun => Ok(Self::Nieun),
            #[cfg(feature = "archaic-korean")]
            Jongseong::NieunKiyeok => Ok(Self::NieunKiyeok),
            #[cfg(feature = "archaic-korean")]
            Jongseong::SsangNieun => Ok(Self::SsangNieun),
            #[cfg(feature = "archaic-korean")]
            Jongseong::NieunTikeut => Ok(Self::NieunTikeut),
            #[cfg(feature = "archaic-korean")]
            Jongseong::NieunSios => Ok(Self::NieunSios),
            #[cfg(feature = "archaic-korean")]
            Jongseong::NieunCieuc => Ok(Self::NieunCieuc),
            #[cfg(feature = "archaic-korean")]
            Jongseong::NieunHieuh => Ok(Self::NieunHieuh),
            Jongseong::Tikeut => Ok(Self::Tikeut),
            #[cfg(feature = "archaic-korean")]
            Jongseong::TikeutKiyeok => Ok(Self::TikeutKiyeok),
            #[cfg(feature = "archaic-korean")]
            Jongseong::SsangTikeut => Ok(Self::SsangTikeut),
            #[cfg(feature = "archaic-korean")]
            Jongseong::TikeutRieul => Ok(Self::TikeutRieul),
            #[cfg(feature = "archaic-korean")]
            Jongseong::TikeutPieup => Ok(Self::TikeutPieup),
            #[cfg(feature = "archaic-korean")]
            Jongseong::TikeutSios => Ok(Self::TikeutSios),
            #[cfg(feature = "archaic-korean")]
            Jongseong::TikeutCieuc => Ok(Self::TikeutCieuc),
            Jongseong::Rieul => Ok(Self::Rieul),
            #[cfg(feature = "archaic-korean")]
            Jongseong::RieulKiyeok => Ok(Self::RieulKiyeok),
            #[cfg(feature = "archaic-korean")]
            Jongseong::RieulSsangKiyeok => Ok(Self::RieulSsangKiyeok),
            #[cfg(feature = "archaic-korean")]
            Jongseong::RieulNieun => Ok(Self::RieulNieun),
            #[cfg(feature = "archaic-korean")]
            Jongseong::RieulTikeut => Ok(Self::RieulTikeut),
            #[cfg(feature = "archaic-korean")]
            Jongseong::SsangRieul => Ok(Self::SsangRieul),
            #[cfg(feature = "archaic-korean")]
            Jongseong::RieulMieum => Ok(Self::RieulMieum),
            #[cfg(feature = "archaic-korean")]
            Jongseong::RieulPieup => Ok(Self::RieulPieup),
            #[cfg(feature = "archaic-korean")]
            Jongseong::RieulKapyeounPieup => Ok(Self::RieulKapyeounPieup),
            #[cfg(feature = "archaic-korean")]
            Jongseong::RieulSios => Ok(Self::RieulSios),
            #[cfg(feature = "archaic-korean")]
            Jongseong::RieulKhieukh => Ok(Self::RieulKhieukh),
            #[cfg(feature = "archaic-korean")]
            Jongseong::RieulHieuh => Ok(Self::RieulHieuh),
            #[cfg(feature = "archaic-korean")]
            Jongseong::KapyeounRieul => Ok(Self::KapyeounRieul),
            Jongseong::Mieum => Ok(Self::Mieum),
            #[cfg(feature = "archaic-korean")]
            Jongseong::MieumKiyeok => Ok(Self::MieumKiyeok),
            #[cfg(feature = "archaic-korean")]
            Jongseong::MieumPieup => Ok(Self::MieumPieup),
            #[cfg(feature = "archaic-korean")]
            Jongseong::MieumSios => Ok(Self::MieumSios),
            #[cfg(feature = "archaic-korean")]
            Jongseong::KapyeounMieum => Ok(Self::KapyeounMieum),
            Jongseong::Pieup => Ok(Self::Pieup),
            #[cfg(feature = "archaic-korean")]
            Jongseong::PieupTikeut => Ok(Self::PieupTikeut),
            #[cfg(feature = "archaic-korean")]
            Jongseong::SsangPieup => Ok(Self::SsangPieup),
            #[cfg(feature = "archaic-korean")]
            Jongseong::PieupSios => Ok(Self::PieupSios),
            #[cfg(feature = "archaic-korean")]
            Jongseong::PieupSiosTikeut => Ok(Self::PieupSiosTikeut),
            #[cfg(feature = "archaic-korean")]
            Jongseong::PieupCieuc => Ok(Self::PieupCieuc),
            #[cfg(feature = "archaic-korean")]
            Jongseong::PieupChieuch => Ok(Self::PieupChieuch),
            #[cfg(feature = "archaic-korean")]
            Jongseong::PieupPhieuph => Ok(Self::PieupPhieuph),
            #[cfg(feature = "archaic-korean")]
            Jongseong::PieupHieuh => Ok(Self::PieupHieuh),
            #[cfg(feature = "archaic-korean")]
            Jongseong::KapyeounPieup => Ok(Self::KapyeounPieup),
            Jongseong::Sios => Ok(Self::Sios),
            #[cfg(feature = "archaic-korean")]
            Jongseong::SiosKiyeok => Ok(Self::SiosKiyeok),
            #[cfg(feature = "archaic-korean")]
            Jongseong::SiosTikeut => Ok(Self::SiosTikeut),
            #[cfg(feature = "archaic-korean")]
            Jongseong::SiosRieul => Ok(Self::SiosRieul),
            #[cfg(feature = "archaic-korean")]
            Jongseong::SiosMieum => Ok(Self::SiosMieum),
            #[cfg(feature = "archaic-korean")]
            Jongseong::SiosPieup => Ok(Self::SiosPieup),
            Jongseong::SsangSios => Ok(Self::SsangSios),
            #[cfg(feature = "archaic-korean")]
            Jongseong::SiosCieuc => Ok(Self::SiosCieuc),
            #[cfg(feature = "archaic-korean")]
            Jongseong::SiosChieuch => Ok(Self::SiosChieuch),
            #[cfg(feature = "archaic-korean")]
            Jongseong::SiosThieuth => Ok(Self::SiosThieuth),
            #[cfg(feature = "archaic-korean")]
            Jongseong::SiosHieuh => Ok(Self::SiosHieuh),
            #[cfg(feature = "archaic-korean")]
            Jongseong::PanSios => Ok(Self::PanSios),
            Jongseong::Ieung => Ok(Self::Ieung),
            #[cfg(feature = "archaic-korean")]
            Jongseong::YesIeung => Ok(Self::YesIeung),
            Jongseong::Cieuc => Ok(Self::Cieuc),
            #[cfg(feature = "archaic-korean")]
            Jongseong::SsangCieuc => Ok(Self::SsangCieuc),
            Jongseong::Chieuch => Ok(Self::Chieuch),
            Jongseong::Khieukh => Ok(Self::Khieukh),
            Jongseong::Thieuth => Ok(Self::Thieuth),
            Jongseong::Phieuph => Ok(Self::Phieuph),
            #[cfg(feature = "archaic-korean")]
            Jongseong::PhieuphPieup => Ok(Self::PhieuphPieup),
            #[cfg(feature = "archaic-korean")]
            Jongseong::KapyeounPhieuph => Ok(Self::KapyeounPhieuph),
            Jongseong::Hieuh => Ok(Self::Hieuh),
            #[cfg(feature = "archaic-korean")]
            Jongseong::YeorinHieuh => Ok(Self::YeorinHieuh),
            _ => Err(Error::NonChoseongTryFromJongseong(value)),
        }
    }
}
impl Choseong {
    // This list is only exported with `archaic-korean` feature, because without it the [`Choseong`] should be in order by itself.
    #[cfg(feature = "archaic-korean")]
    /// Lists [`Choseong`] in correct dictionary order.
    pub const IN_ORDER: [Self; 124] = [
        Self::Kiyeok,
        Self::SsangKiyeok,
        #[cfg(feature = "archaic-korean")]
        Self::KiyeokTikeut,
        Self::Nieun,
        #[cfg(feature = "archaic-korean")]
        Self::NieunKiyeok,
        #[cfg(feature = "archaic-korean")]
        Self::SsangNieun,
        #[cfg(feature = "archaic-korean")]
        Self::NieunTikeut,
        #[cfg(feature = "archaic-korean")]
        Self::NieunPieup,
        #[cfg(feature = "archaic-korean")]
        Self::NieunSios,
        #[cfg(feature = "archaic-korean")]
        Self::NieunCieuc,
        #[cfg(feature = "archaic-korean")]
        Self::NieunHieuh,
        Self::Tikeut,
        #[cfg(feature = "archaic-korean")]
        Self::TikeutKiyeok,
        Self::SsangTikeut,
        #[cfg(feature = "archaic-korean")]
        Self::TikeutRieul,
        #[cfg(feature = "archaic-korean")]
        Self::TikeutMieum,
        #[cfg(feature = "archaic-korean")]
        Self::TikeutPieup,
        #[cfg(feature = "archaic-korean")]
        Self::TikeutSios,
        #[cfg(feature = "archaic-korean")]
        Self::TikeutCieuc,
        Self::Rieul,
        #[cfg(feature = "archaic-korean")]
        Self::RieulKiyeok,
        #[cfg(feature = "archaic-korean")]
        Self::RieulSsangKiyeok,
        #[cfg(feature = "archaic-korean")]
        Self::RieulNieun,
        #[cfg(feature = "archaic-korean")]
        Self::RieulTikeut,
        #[cfg(feature = "archaic-korean")]
        Self::RieulSsangTikeut,
        #[cfg(feature = "archaic-korean")]
        Self::SsangRieul,
        #[cfg(feature = "archaic-korean")]
        Self::RieulMieum,
        #[cfg(feature = "archaic-korean")]
        Self::RieulPieup,
        #[cfg(feature = "archaic-korean")]
        Self::RieulSsangPieup,
        #[cfg(feature = "archaic-korean")]
        Self::RieulKapyeounPieup,
        #[cfg(feature = "archaic-korean")]
        Self::RieulSios,
        #[cfg(feature = "archaic-korean")]
        Self::RieulCieuc,
        #[cfg(feature = "archaic-korean")]
        Self::RieulKhieukh,
        #[cfg(feature = "archaic-korean")]
        Self::RieulHieuh,
        #[cfg(feature = "archaic-korean")]
        Self::KapyeounRieul,
        Self::Mieum,
        #[cfg(feature = "archaic-korean")]
        Self::MieumKiyeok,
        #[cfg(feature = "archaic-korean")]
        Self::MieumTikeut,
        #[cfg(feature = "archaic-korean")]
        Self::MieumPieup,
        #[cfg(feature = "archaic-korean")]
        Self::MieumSios,
        #[cfg(feature = "archaic-korean")]
        Self::KapyeounMieum,
        Self::Pieup,
        #[cfg(feature = "archaic-korean")]
        Self::PieupKiyeok,
        #[cfg(feature = "archaic-korean")]
        Self::PieupNieun,
        #[cfg(feature = "archaic-korean")]
        Self::PieupTikeut,
        Self::SsangPieup,
        #[cfg(feature = "archaic-korean")]
        Self::PieupSios,
        #[cfg(feature = "archaic-korean")]
        Self::PieupSiosKiyeok,
        #[cfg(feature = "archaic-korean")]
        Self::PieupSiosTikeut,
        #[cfg(feature = "archaic-korean")]
        Self::PieupSiosPieup,
        #[cfg(feature = "archaic-korean")]
        Self::PieupSsangSios,
        #[cfg(feature = "archaic-korean")]
        Self::PieupSiosCieuc,
        #[cfg(feature = "archaic-korean")]
        Self::PieupSiosThieuth,
        #[cfg(feature = "archaic-korean")]
        Self::PieupCieuc,
        #[cfg(feature = "archaic-korean")]
        Self::PieupChieuch,
        #[cfg(feature = "archaic-korean")]
        Self::PieupKhieukh,
        #[cfg(feature = "archaic-korean")]
        Self::PieupThieuth,
        #[cfg(feature = "archaic-korean")]
        Self::PieupPhieuph,
        #[cfg(feature = "archaic-korean")]
        Self::PieupHieuh,
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
        Self::SiosRieul,
        #[cfg(feature = "archaic-korean")]
        Self::SiosMieum,
        #[cfg(feature = "archaic-korean")]
        Self::SiosPieup,
        #[cfg(feature = "archaic-korean")]
        Self::SiosPieupKiyeok,
        Self::SsangSios,
        #[cfg(feature = "archaic-korean")]
        Self::SsangSiosPieup,
        #[cfg(feature = "archaic-korean")]
        Self::SiosSsangSios,
        #[cfg(feature = "archaic-korean")]
        Self::SiosIeung,
        #[cfg(feature = "archaic-korean")]
        Self::SiosCieuc,
        #[cfg(feature = "archaic-korean")]
        Self::SiosChieuch,
        #[cfg(feature = "archaic-korean")]
        Self::SiosKhieukh,
        #[cfg(feature = "archaic-korean")]
        Self::SiosThieuth,
        #[cfg(feature = "archaic-korean")]
        Self::SiosPhieuph,
        #[cfg(feature = "archaic-korean")]
        Self::SiosHieuh,
        #[cfg(feature = "archaic-korean")]
        Self::ChitueumSios,
        #[cfg(feature = "archaic-korean")]
        Self::ChitueumSsangSios,
        #[cfg(feature = "archaic-korean")]
        Self::CeongchieumSios,
        #[cfg(feature = "archaic-korean")]
        Self::CeongchieumSsangSios,
        #[cfg(feature = "archaic-korean")]
        Self::PanSios,
        Self::Ieung,
        #[cfg(feature = "archaic-korean")]
        Self::IeungKiyeok,
        #[cfg(feature = "archaic-korean")]
        Self::IeungTikeut,
        #[cfg(feature = "archaic-korean")]
        Self::IeungRieul,
        #[cfg(feature = "archaic-korean")]
        Self::IeungMieum,
        #[cfg(feature = "archaic-korean")]
        Self::IeungPieup,
        #[cfg(feature = "archaic-korean")]
        Self::IeungSios,
        #[cfg(feature = "archaic-korean")]
        Self::IeungPanSios,
        #[cfg(feature = "archaic-korean")]
        Self::SsangIeung,
        #[cfg(feature = "archaic-korean")]
        Self::IeungCieuc,
        #[cfg(feature = "archaic-korean")]
        Self::IeungChieuch,
        #[cfg(feature = "archaic-korean")]
        Self::IeungThieuth,
        #[cfg(feature = "archaic-korean")]
        Self::IeungPhieuph,
        #[cfg(feature = "archaic-korean")]
        Self::IeungHieuh,
        #[cfg(feature = "archaic-korean")]
        Self::YesIeung,
        Self::Cieuc,
        #[cfg(feature = "archaic-korean")]
        Self::CieucIeung,
        Self::SsangCieuc,
        #[cfg(feature = "archaic-korean")]
        Self::SsangCieucHieuh,
        #[cfg(feature = "archaic-korean")]
        Self::ChitueumCieuc,
        #[cfg(feature = "archaic-korean")]
        Self::ChitueumSsangCieuc,
        #[cfg(feature = "archaic-korean")]
        Self::CeongchieumCieuc,
        #[cfg(feature = "archaic-korean")]
        Self::CeongchieumSsangCieuc,
        Self::Chieuch,
        #[cfg(feature = "archaic-korean")]
        Self::ChieuchKhieukh,
        #[cfg(feature = "archaic-korean")]
        Self::ChieuchHieuh,
        #[cfg(feature = "archaic-korean")]
        Self::ChitueumChieuch,
        #[cfg(feature = "archaic-korean")]
        Self::CeongchieumChieuch,
        Self::Khieukh,
        Self::Thieuth,
        #[cfg(feature = "archaic-korean")]
        Self::SsangThieuth,
        Self::Phieuph,
        #[cfg(feature = "archaic-korean")]
        Self::PhieuphPieup,
        #[cfg(feature = "archaic-korean")]
        Self::PhieuphHieuh,
        #[cfg(feature = "archaic-korean")]
        Self::KapyeounPhieuph,
        Self::Hieuh,
        #[cfg(feature = "archaic-korean")]
        Self::HieuhSios,
        #[cfg(feature = "archaic-korean")]
        Self::SsangHieuh,
        #[cfg(feature = "archaic-korean")]
        Self::YeorinHieuh,
        #[cfg(feature = "archaic-korean")]
        Self::SsangYeorinHieuh,
    ];
}
