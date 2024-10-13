use crate::{
    consonant::{Jaeum, Jongseong},
    Error,
};
use num_enum::{IntoPrimitive, TryFromPrimitive};
use std::convert::TryFrom;

/// A set of consonants applicable as initial consonant (초성, Choseong).
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
impl From<Choseong> for char {
    fn from(value: Choseong) -> Self {
        // guaranteed to not fail within BMP
        char::from_u32(value.into()).unwrap()
    }
}
impl TryFrom<char> for Choseong {
    type Error = Error;

    /// Tries to convert a [`char`] into `Choseong`.
    ///
    /// # Direct Conversion
    /// The `char` given will be tested against the following range(s):
    ///
    /// * Hangul Jamo / Initial consonants (U+1100--U+1112)
    /// * Hangul Jamo / Old initial consonants (U+1113--U+115E)
    /// * Hangul Jamo Extended-A / Old initial consonants (U+A960--U+A97C)
    ///
    /// The latter two ranges are considered valid if and only if `archaic-korean` feature is enabled.
    ///
    /// # Indirect Conversion
    /// For the following range(s), the `char` given will be tried converting into [`Jaeum`] first, then subsequently into `Choseong`:
    ///
    /// * Hangul Jamo / Final consonants (U+11A8--U+11C2)
    /// * Hangul Compatibility Jamo / Consonant letters (U+3131--U+314E)
    /// * Halfwidth and Fullwidth Forms / Halfwidth Hangul variants (U+FFA1--U+FFBE)
    /// * Hangul Jamo / Old final consonants (U+11C3--U+11FF)
    /// * Hangul Compatibility Jamo / Old consonant letters (U+3165--U+3186)
    /// * Hangul Jamo Extended-B / Old final consonants (U+D7CB--U+D7FB)
    ///
    /// The latter three ranges are considered valid if and only if `archaic-korean` feature is enabled.
    ///
    /// # Errors
    /// * [`Error::NonChoseongTryFromChar`]: the `char` given is not a valid initial consonant.
    /// * [`Error::NonJaeumTryFromChar`]: the `char` given is not a valid consonant.
    /// * [`Error::NonKoreanTryFromChar`]: the `char` given is not a valid Korean alphabet at all.
    fn try_from(value: char) -> Result<Self, Self::Error> {
        let code_point = value as u32;

        match code_point {
            0x1100..=0x1112 => return Ok(Choseong::try_from(code_point).unwrap()),
            0x1161..=0x1175
            | 0x314F..=0x3163
            | 0xFFC2..=0xFFC7
            | 0xFFCA..=0xFFCF
            | 0xFFD2..=0xFFD7
            | 0xFFDA..=0xFFDC => return Err(Error::NonChoseongTryFromChar(value)),
            0x11A8..=0x11C2 | 0x3131..=0x314E | 0xFFA1..=0xFFBE => {
                return Ok(Choseong::try_from(Jaeum::try_from(value)?)?);
            }
            _ => {}
        }

        #[cfg(feature = "archaic-korean")]
        match code_point {
            0x1113..=0x115E | 0xA960..=0xA97C => return Ok(Choseong::try_from(code_point).unwrap()),
            0x1176..=0x11A7 | 0x3187..=0x318E | 0xD7B0..=0xD7C6 => {
                return Err(Error::NonChoseongTryFromChar(value))
            }
            0x11C3..=0x11FF | 0x3165..=0x3186 | 0xD7CB..=0xD7FB => {
                return Ok(Choseong::try_from(Jaeum::try_from(value)?)?);
            }
            _ => {}
        }

        Err(Error::NonKoreanTryFromChar(value))
    }
}
impl TryFrom<Jaeum> for Choseong {
    type Error = Error;

    /// Tries to convert a [`Jaeum`] into `Choseong`.
    ///
    /// # Errors
    /// * [`Error::NonChoseongTryFromJaeum`]: the `Jaeum` given is not a valid initial consonant.
    fn try_from(value: Jaeum) -> Result<Self, Self::Error> {
        // TODO: consider switching to bst; but i'm not very sure of performance boost it'll yield.
        match value {
            Jaeum::Kiyeok => Ok(Choseong::Kiyeok),
            Jaeum::SsangKiyeok => Ok(Choseong::SsangKiyeok),
            Jaeum::Nieun => Ok(Choseong::Nieun),
            #[cfg(feature = "archaic-korean")]
            Jaeum::NieunCieuc => Ok(Choseong::NieunCieuc),
            #[cfg(feature = "archaic-korean")]
            Jaeum::NieunHieuh => Ok(Choseong::NieunHieuh),
            Jaeum::Tikeut => Ok(Choseong::Tikeut),
            Jaeum::SsangTikeut => Ok(Choseong::SsangTikeut),
            Jaeum::Rieul => Ok(Choseong::Rieul),
            #[cfg(feature = "archaic-korean")]
            Jaeum::RieulKiyeok => Ok(Choseong::RieulKiyeok),
            #[cfg(feature = "archaic-korean")]
            Jaeum::RieulMieum => Ok(Choseong::RieulMieum),
            #[cfg(feature = "archaic-korean")]
            Jaeum::RieulPieup => Ok(Choseong::RieulPieup),
            #[cfg(feature = "archaic-korean")]
            Jaeum::RieulSios => Ok(Choseong::RieulSios),
            #[cfg(feature = "archaic-korean")]
            Jaeum::RieulHieuh => Ok(Choseong::RieulHieuh),
            Jaeum::Mieum => Ok(Choseong::Mieum),
            Jaeum::Pieup => Ok(Choseong::Pieup),
            Jaeum::SsangPieup => Ok(Choseong::SsangPieup),
            #[cfg(feature = "archaic-korean")]
            Jaeum::PieupSios => Ok(Choseong::PieupSios),
            Jaeum::Sios => Ok(Choseong::Sios),
            Jaeum::SsangSios => Ok(Choseong::SsangSios),
            Jaeum::Ieung => Ok(Choseong::Ieung),
            Jaeum::Cieuc => Ok(Choseong::Cieuc),
            Jaeum::SsangCieuc => Ok(Choseong::SsangCieuc),
            Jaeum::Chieuch => Ok(Choseong::Chieuch),
            Jaeum::Khieukh => Ok(Choseong::Khieukh),
            Jaeum::Thieuth => Ok(Choseong::Thieuth),
            Jaeum::Phieuph => Ok(Choseong::Phieuph),
            Jaeum::Hieuh => Ok(Choseong::Hieuh),
            #[cfg(feature = "archaic-korean")]
            Jaeum::SsangNieun => Ok(Choseong::SsangNieun),
            #[cfg(feature = "archaic-korean")]
            Jaeum::NieunTikeut => Ok(Choseong::NieunTikeut),
            #[cfg(feature = "archaic-korean")]
            Jaeum::NieunSios => Ok(Choseong::NieunSios),
            #[cfg(feature = "archaic-korean")]
            Jaeum::RieulTikeut => Ok(Choseong::RieulTikeut),
            #[cfg(feature = "archaic-korean")]
            Jaeum::MieumPieup => Ok(Choseong::MieumPieup),
            #[cfg(feature = "archaic-korean")]
            Jaeum::MieumSios => Ok(Choseong::MieumSios),
            #[cfg(feature = "archaic-korean")]
            Jaeum::KapyeounMieum => Ok(Choseong::KapyeounMieum),
            #[cfg(feature = "archaic-korean")]
            Jaeum::PieupKiyeok => Ok(Choseong::PieupKiyeok),
            #[cfg(feature = "archaic-korean")]
            Jaeum::PieupTikeut => Ok(Choseong::PieupTikeut),
            #[cfg(feature = "archaic-korean")]
            Jaeum::PieupSiosKiyeok => Ok(Choseong::PieupSiosKiyeok),
            #[cfg(feature = "archaic-korean")]
            Jaeum::PieupSiosTikeut => Ok(Choseong::PieupSiosTikeut),
            #[cfg(feature = "archaic-korean")]
            Jaeum::PieupCieuc => Ok(Choseong::PieupCieuc),
            #[cfg(feature = "archaic-korean")]
            Jaeum::PieupThieuth => Ok(Choseong::PieupThieuth),
            #[cfg(feature = "archaic-korean")]
            Jaeum::KapyeounPieup => Ok(Choseong::KapyeounPieup),
            #[cfg(feature = "archaic-korean")]
            Jaeum::KapyeounSsangPieup => Ok(Choseong::KapyeounSsangPieup),
            #[cfg(feature = "archaic-korean")]
            Jaeum::SiosKiyeok => Ok(Choseong::SiosKiyeok),
            #[cfg(feature = "archaic-korean")]
            Jaeum::SiosNieun => Ok(Choseong::SiosNieun),
            #[cfg(feature = "archaic-korean")]
            Jaeum::SiosTikeut => Ok(Choseong::SiosTikeut),
            #[cfg(feature = "archaic-korean")]
            Jaeum::SiosPieup => Ok(Choseong::SiosPieup),
            #[cfg(feature = "archaic-korean")]
            Jaeum::SiosCieuc => Ok(Choseong::SiosCieuc),
            #[cfg(feature = "archaic-korean")]
            Jaeum::PanSios => Ok(Choseong::PanSios),
            #[cfg(feature = "archaic-korean")]
            Jaeum::SsangIeung => Ok(Choseong::SsangIeung),
            #[cfg(feature = "archaic-korean")]
            Jaeum::YesIeung => Ok(Choseong::YesIeung),
            #[cfg(feature = "archaic-korean")]
            Jaeum::KapyeounPhieuph => Ok(Choseong::KapyeounPhieuph),
            #[cfg(feature = "archaic-korean")]
            Jaeum::SsangHieuh => Ok(Choseong::SsangHieuh),
            #[cfg(feature = "archaic-korean")]
            Jaeum::YeorinHieuh => Ok(Choseong::YeorinHieuh),
            _ => Err(Error::NonChoseongTryFromJaeum(value)),
        }
    }
}
impl TryFrom<Jongseong> for Choseong {
    type Error = Error;

    /// Tries to convert a [`Jongseong`] into `Choseong`, by leveraging existing conversion between `Jongseong` and [`Jaeum`], and between `Jaeum` and `Choseong`.
    ///
    /// Note: whether all `Choseong`-capable `Jongseong` successfully converts to `Choseong` or not has not yet been fully tested, thus consider this function **unstable**.
    ///
    /// # Errors
    /// * [`Error::NonChoseongTryFromJaeum`]: the **`Jongseong`** (actually, a `Jaeum`-ized `Jongseong`) given is not valid in initial consonant position.
    /// * [`Error::NonUnicodeJaeumTryFromJongseong`]: side effect from the approach this function took; until the conversion is fully validated, this error shall serve as a logical assertion.
    fn try_from(value: Jongseong) -> Result<Self, Self::Error> {
        Self::try_from(Jaeum::try_from(value)?)
    }
}
impl Choseong {
    // TODO: consider exporting this array even when `archaic-korean` feature is not selected.
    //       prob will require getting enum size at compile time kind of special magic thing.
    #[cfg(feature = "archaic-korean")]
    /// Lists `Choseong` in correct dictionary order.
    pub const IN_ORDER: [Choseong; 124] = [
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
