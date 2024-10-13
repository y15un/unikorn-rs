use crate::{
    consonant::{Choseong, Jaeum},
    Error,
};
use num_enum::{IntoPrimitive, TryFromPrimitive};
use std::convert::TryFrom;

/// A set of consonants applicable as final consonant (종성, Jongseong).
#[derive(Clone, Copy, Debug, Eq, IntoPrimitive, Ord, PartialEq, PartialOrd, TryFromPrimitive)]
#[repr(u32)]
pub enum Jongseong {
    /// Represents final consonant `ᆨ` (U+11A8, Hangul Jongseong Kiyeok)
    Kiyeok = 0x11A8,
    /// Represents final consonant `ᆩ` (U+11A9, Hangul Jongseong SsangKiyeok)
    SsangKiyeok,
    /// Represents final consonant `ᆪ` (U+11AA, Hangul Jongseong Kiyeok-Sios)
    KiyeokSios,
    /// Represents final consonant `ᆫ` (U+11AB, Hangul Jongseong Nieun)
    Nieun,
    /// Represents final consonant `ᆬ` (U+11AC, Hangul Jongseong Nieun-Cieuc)
    NieunCieuc,
    /// Represents final consonant `ᆭ` (U+11AD, Hangul Jongseong Nieun-Hieuh)
    NieunHieuh,
    /// Represents final consonant `ᆮ` (U+11AE, Hangul Jongseong Tikeut)
    Tikeut,
    /// Represents final consonant `ᆯ` (U+11AF, Hangul Jongseong Rieul)
    Rieul,
    /// Represents final consonant `ᆰ` (U+11B0, Hangul Jongseong Rieul-Kiyeok)
    RieulKiyeok,
    /// Represents final consonant `ᆱ` (U+11B1, Hangul Jongseong Rieul-Mieum)
    RieulMieum,
    /// Represents final consonant `ᆲ` (U+11B2, Hangul Jongseong Rieul-Pieup)
    RieulPieup,
    /// Represents final consonant `ᆳ` (U+11B3, Hangul Jongseong Rieul-Sios)
    RieulSios,
    /// Represents final consonant `ᆴ` (U+11B4, Hangul Jongseong Rieul-Thieuth)
    RieulThieuth,
    /// Represents final consonant `ᆵ` (U+11B5, Hangul Jongseong Rieul-Phieuph)
    RieulPhieuph,
    /// Represents final consonant `ᆶ` (U+11B6, Hangul Jongseong Rieul-Hieuh)
    RieulHieuh,
    /// Represents final consonant `ᆷ` (U+11B7, Hangul Jongseong Mieum)
    Mieum,
    /// Represents final consonant `ᆸ` (U+11B8, Hangul Jongseong Pieup)
    Pieup,
    /// Represents final consonant `ᆹ` (U+11B9, Hangul Jongseong Pieup-Sios)
    PieupSios,
    /// Represents final consonant `ᆺ` (U+11BA, Hangul Jongseong Sios)
    Sios,
    /// Represents final consonant `ᆻ` (U+11BB, Hangul Jongseong SsangSios)
    SsangSios,
    /// Represents final consonant `ᆼ` (U+11BC, Hangul Jongseong Ieung)
    Ieung,
    /// Represents final consonant `ᆽ` (U+11BD, Hangul Jongseong Cieuc)
    Cieuc,
    /// Represents final consonant `ᆾ` (U+11BE, Hangul Jongseong Chieuch)
    Chieuch,
    /// Represents final consonant `ᆿ` (U+11BF, Hangul Jongseong Khieukh)
    Khieukh,
    /// Represents final consonant `ᇀ` (U+11C0, Hangul Jongseong Thieuth)
    Thieuth,
    /// Represents final consonant `ᇁ` (U+11C1, Hangul Jongseong Phieuph)
    Phieuph,
    /// Represents final consonant `ᇂ` (U+11C2, Hangul Jongseong Hieuh)
    Hieuh,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* final consonant `ᇃ` (U+11C3, Hangul Jongseong Kiyeok-Rieul)
    KiyeokRieul,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* final consonant `ᇄ` (U+11C4, Hangul Jongseong Kiyeok-Sios-Kiyeok)
    KiyeokSiosKiyeok,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* final consonant `ᇅ` (U+11C5, Hangul Jongseong Nieun-Kiyeok)
    NieunKiyeok,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* final consonant `ᇆ` (U+11C6, Hangul Jongseong Nieun-Tikeut)
    NieunTikeut,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* final consonant `ᇇ` (U+11C7, Hangul Jongseong Nieun-Sios)
    NieunSios,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* final consonant `ᇈ` (U+11C8, Hangul Jongseong Nieun-PanSios)
    NieunPanSios,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* final consonant `ᇉ` (U+11C9, Hangul Jongseong Nieun-Thieuth)
    NieunThieuth,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* final consonant `ᇊ` (U+11CA, Hangul Jongseong Tikeut-Kiyeok)
    TikeutKiyeok,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* final consonant `ᇋ` (U+11CB, Hangul Jongseong Tikeut-Rieul)
    TikeutRieul,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* final consonant `ᇌ` (U+11CC, Hangul Jongseong Rieul-Kiyeok-Sios)
    RieulKiyeokSios,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* final consonant `ᇍ` (U+11CD, Hangul Jongseong Rieul-Nieun)
    RieulNieun,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* final consonant `ᇎ` (U+11CE, Hangul Jongseong Rieul-Tikeut)
    RieulTikeut,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* final consonant `ᇏ` (U+11CF, Hangul Jongseong Rieul-Tikeut-Hieuh)
    RieulTikeutHieuh,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* final consonant `ᇐ` (U+11D0, Hangul Jongseong SsangRieul)
    SsangRieul,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* final consonant `ᇑ` (U+11D1, Hangul Jongseong Rieul-Mieum-Kiyeok)
    RieulMieumKiyeok,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* final consonant `ᇒ` (U+11D2, Hangul Jongseong Rieul-Mieum-Sios)
    RieulMieumSios,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* final consonant `ᇓ` (U+11D3, Hangul Jongseong Rieul-Pieup-Sios)
    RieulPieupSios,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* final consonant `ᇔ` (U+11D4, Hangul Jongseong Rieul-Pieup-Hieuh)
    RieulPieupHieuh,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* final consonant `ᇕ` (U+11D5, Hangul Jongseong Rieul-KapyeounPieup)
    RieulKapyeounPieup,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* final consonant `ᇖ` (U+11D6, Hangul Jongseong Rieul-SsangSios)
    RieulSsangSios,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* final consonant `ᇗ` (U+11D7, Hangul Jongseong Rieul-PanSios)
    RieulPanSios,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* final consonant `ᇘ` (U+11D8, Hangul Jongseong Rieul-Khieukh)
    RieulKhieukh,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* final consonant `ᇙ` (U+11D9, Hangul Jongseong Rieul-YeorinHieuh)
    RieulYeorinHieuh,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* final consonant `ᇚ` (U+11DA, Hangul Jongseong Mieum-Kiyeok)
    MieumKiyeok,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* final consonant `ᇛ` (U+11DB, Hangul Jongseong Mieum-Rieul)
    MieumRieul,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* final consonant `ᇜ` (U+11DC, Hangul Jongseong Mieum-Pieup)
    MieumPieup,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* final consonant `ᇝ` (U+11DD, Hangul Jongseong Mieum-Sios)
    MieumSios,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* final consonant `ᇞ` (U+11DE, Hangul Jongseong Mieum-SsangSios)
    MieumSsangSios,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* final consonant `ᇟ` (U+11DF, Hangul Jongseong Mieum-PanSios)
    MieumPanSios,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* final consonant `ᇠ` (U+11E0, Hangul Jongseong Mieum-Chieuch)
    MieumChieuch,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* final consonant `ᇡ` (U+11E1, Hangul Jongseong Mieum-Hieuh)
    MieumHieuh,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* final consonant `ᇢ` (U+11E2, Hangul Jongseong KapyeounMieum)
    KapyeounMieum,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* final consonant `ᇣ` (U+11E3, Hangul Jongseong Pieup-Rieul)
    PieupRieul,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* final consonant `ᇤ` (U+11E4, Hangul Jongseong Pieup-Phieuph)
    PieupPhieuph,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* final consonant `ᇥ` (U+11E5, Hangul Jongseong Pieup-Hieuh)
    PieupHieuh,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* final consonant `ᇦ` (U+11E6, Hangul Jongseong KapyeounPieup)
    KapyeounPieup,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* final consonant `ᇧ` (U+11E7, Hangul Jongseong Sios-Kiyeok)
    SiosKiyeok,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* final consonant `ᇨ` (U+11E8, Hangul Jongseong Sios-Tikeut)
    SiosTikeut,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* final consonant `ᇩ` (U+11E9, Hangul Jongseong Sios-Rieul)
    SiosRieul,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* final consonant `ᇪ` (U+11EA, Hangul Jongseong Sios-Pieup)
    SiosPieup,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* final consonant `ᇫ` (U+11EB, Hangul Jongseong PanSios)
    PanSios,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* final consonant `ᇬ` (U+11EC, Hangul Jongseong Ieung-Kiyeok)
    IeungKiyeok,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* final consonant `ᇭ` (U+11ED, Hangul Jongseong Ieung-SsangKiyeok)
    IeungSsangKiyeok,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* final consonant `ᇮ` (U+11EE, Hangul Jongseong SsangIeung)
    SsangIeung,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* final consonant `ᇯ` (U+11EF, Hangul Jongseong Ieung-Khieukh)
    IeungKhieukh,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* final consonant `ᇰ` (U+11F0, Hangul Jongseong YesIeung)
    YesIeung,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* final consonant `ᇱ` (U+11F1, Hangul Jongseong YesIeung-Sios)
    YesIeungSios,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* final consonant `ᇲ` (U+11F2, Hangul Jongseong YesIeung-PanSios)
    YesIeungPanSios,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* final consonant `ᇳ` (U+11F3, Hangul Jongseong Phieuph-Pieup)
    PhieuphPieup,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* final consonant `ᇴ` (U+11F4, Hangul Jongseong KapyeounPhieuph)
    KapyeounPhieuph,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* final consonant `ᇵ` (U+11F5, Hangul Jongseong Hieuh-Nieun)
    HieuhNieun,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* final consonant `ᇶ` (U+11F6, Hangul Jongseong Hieuh-Rieul)
    HieuhRieul,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* final consonant `ᇷ` (U+11F7, Hangul Jongseong Hieuh-Mieum)
    HieuhMieum,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* final consonant `ᇸ` (U+11F8, Hangul Jongseong Hieuh-Pieup)
    HieuhPieup,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* final consonant `ᇹ` (U+11F9, Hangul Jongseong YeorinHieuh)
    YeorinHieuh,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* final consonant `ᇺ` (U+11FA, Hangul Jongseong Kiyeok-Nieun)
    KiyeokNieun,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* final consonant `ᇻ` (U+11FB, Hangul Jongseong Kiyeok-Pieup)
    KiyeokPieup,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* final consonant `ᇼ` (U+11FC, Hangul Jongseong Kiyeok-Chieuch)
    KiyeokChieuch,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* final consonant `ᇽ` (U+11FD, Hangul Jongseong Kiyeok-Khieukh)
    KiyeokKhieukh,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* final consonant `ᇾ` (U+11FE, Hangul Jongseong Kiyeok-Hieuh)
    KiyeokHieuh,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* final consonant `ᇿ` (U+11FF, Hangul Jongseong SsangNieun)
    SsangNieun,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* final consonant `ퟋ` (U+D7CB, Hangul Jongseong Nieun-Rieul)
    NieunRieul = 0xD7CB,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* final consonant `ퟌ` (U+D7CC, Hangul Jongseong Nieun-Chieuch)
    NieunChieuch,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* final consonant `ퟍ` (U+D7CD, Hangul Jongseong SsangTikeut)
    SsangTikeut,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* final consonant `ퟎ` (U+D7CE, Hangul Jongseong SsangTikeut-Pieup)
    SsangTikeutPieup,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* final consonant `ퟏ` (U+D7CF, Hangul Jongseong Tikeut-Pieup)
    TikeutPieup,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* final consonant `ퟐ` (U+D7D0, Hangul Jongseong Tikeut-Sios)
    TikeutSios,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* final consonant `ퟑ` (U+D7D1, Hangul Jongseong Tikeut-Sios-Kiyeok)
    TikeutSiosKiyeok,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* final consonant `ퟒ` (U+D7D2, Hangul Jongseong Tikeut-Cieuc)
    TikeutCieuc,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* final consonant `ퟓ` (U+D7D3, Hangul Jongseong Tikeut-Chieuch)
    TikeutChieuch,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* final consonant `ퟔ` (U+D7D4, Hangul Jongseong Tikeut-Thieuth)
    TikeutThieuth,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* final consonant `ퟕ` (U+D7D5, Hangul Jongseong Rieul-SsangKiyeok)
    RieulSsangKiyeok,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* final consonant `ퟖ` (U+D7D6, Hangul Jongseong Rieul-Kiyeok-Hieuh)
    RieulKiyeokHieuh,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* final consonant `ퟗ` (U+D7D7, Hangul Jongseong SsangRieul-Khieukh)
    SsangRieulKhieukh,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* final consonant `ퟘ` (U+D7D8, Hangul Jongseong Rieul-Mieum-Hieuh)
    RieulMieumHieuh,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* final consonant `ퟙ` (U+D7D9, Hangul Jongseong Rieul-Pieup-Tikeut)
    RieulPieupTikeut,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* final consonant `ퟚ` (U+D7DA, Hangul Jongseong Rieul-Pieup-Phieuph)
    RieulPieupPhieuph,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* final consonant `ퟛ` (U+D7DB, Hangul Jongseong Rieul-YesIeung)
    RieulYesIeung,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* final consonant `ퟜ` (U+D7DC, Hangul Jongseong Rieul-YeorinHieuh-Hieuh)
    RieulYeorinHieuhHieuh,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* final consonant `ퟝ` (U+D7DD, Hangul Jongseong KapyeounRieul)
    KapyeounRieul,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* final consonant `ퟞ` (U+D7DE, Hangul Jongseong Mieum-Nieun)
    MieumNieun,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* final consonant `ퟟ` (U+D7DF, Hangul Jongseong Mieum-SsangNieun)
    MieumSsangNieun,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* final consonant `ퟠ` (U+D7E0, Hangul Jongseong SsangMieum)
    SsangMieum,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* final consonant `ퟡ` (U+D7E1, Hangul Jongseong Mieum-Pieup-Sios)
    MieumPieupSios,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* final consonant `ퟢ` (U+D7E2, Hangul Jongseong Mieum-Cieuc)
    MieumCieuc,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* final consonant `ퟣ` (U+D7E3, Hangul Jongseong Pieup-Tikeut)
    PieupTikeut,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* final consonant `ퟤ` (U+D7E4, Hangul Jongseong Pieup-Rieul-Phieuph)
    PieupRieulPhieuph,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* final consonant `ퟥ` (U+D7E5, Hangul Jongseong Pieup-Mieum)
    PieupMieum,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* final consonant `ퟦ` (U+D7E6, Hangul Jongseong SsangPieup)
    SsangPieup,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* final consonant `ퟧ` (U+D7E7, Hangul Jongseong Pieup-Sios-Tikeut)
    PieupSiosTikeut,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* final consonant `ퟨ` (U+D7E8, Hangul Jongseong Pieup-Cieuc)
    PieupCieuc,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* final consonant `ퟩ` (U+D7E9, Hangul Jongseong Pieup-Chieuch)
    PieupChieuch,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* final consonant `ퟪ` (U+D7EA, Hangul Jongseong Sios-Mieum)
    SiosMieum,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* final consonant `ퟫ` (U+D7EB, Hangul Jongseong Sios-KapyeounPieup)
    SiosKapyeounPieup,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* final consonant `ퟬ` (U+D7EC, Hangul Jongseong SsangSios-Kiyeok)
    SsangSiosKiyeok,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* final consonant `ퟭ` (U+D7ED, Hangul Jongseong SsangSios-Tikeut)
    SsangSiosTikeut,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* final consonant `ퟮ` (U+D7EE, Hangul Jongseong Sios-PanSios)
    SiosPanSios,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* final consonant `ퟯ` (U+D7EF, Hangul Jongseong Sios-Cieuc)
    SiosCieuc,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* final consonant `ퟰ` (U+D7F0, Hangul Jongseong Sios-Chieuch)
    SiosChieuch,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* final consonant `ퟱ` (U+D7F1, Hangul Jongseong Sios-Thieuth)
    SiosThieuth,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* final consonant `ퟲ` (U+D7F2, Hangul Jongseong Sios-Hieuh)
    SiosHieuh,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* final consonant `ퟳ` (U+D7F3, Hangul Jongseong PanSios-Pieup)
    PanSiosPieup,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* final consonant `ퟴ` (U+D7F4, Hangul Jongseong PanSios-KapyeounPieup)
    PanSiosKapyeounPieup,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* final consonant `ퟵ` (U+D7F5, Hangul Jongseong YesIeung-Mieum)
    YesIeungMieum,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* final consonant `ퟶ` (U+D7F6, Hangul Jongseong YesIeung-Hieuh)
    YesIeungHieuh,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* final consonant `ퟷ` (U+D7F7, Hangul Jongseong Cieuc-Pieup)
    CieucPieup,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* final consonant `ퟸ` (U+D7F8, Hangul Jongseong Cieuc-SsangPieup)
    CieucSsangPieup,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* final consonant `ퟹ` (U+D7F9, Hangul Jongseong SsangCieuc)
    SsangCieuc,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* final consonant `ퟺ` (U+D7FA, Hangul Jongseong Phieuph-Sios)
    PhieuphSios,
    #[cfg(feature = "archaic-korean")]
    /// Represents *old* final consonant `ퟻ` (U+D7FB, Hangul Jongseong Phieuph-Thieuth)
    PhieuphThieuth,
}
impl From<Jongseong> for char {
    fn from(value: Jongseong) -> Self {
        // guaranteed to not fail within BMP
        char::from_u32(value.into()).unwrap()
    }
}
impl TryFrom<char> for Jongseong {
    type Error = Error;

    /// Tries to convert a [`char`] into `Jongseong`.
    ///
    /// # Direct Conversion
    /// The [`char`] given will be tested against the following range(s):
    ///
    /// * Hangul Jamo / Final consonants (U+11A8--U+11C2)
    /// * Hangul Jamo / Old final consonants (U+11C3--U+11FF)
    /// * Hangul Jamo Extended-B / Old final consonants (U+D7CB--U+D7FB)
    ///
    /// The latter two ranges are considered valid if and only if `archaic-korean` feature is enabled.
    ///
    /// # Indirect Conversion
    /// For the following range(s), the [`char`] given will be tried converting into [`Jaeum`] first, then subsequently into `Jongseong`:
    ///
    /// * Hangul Jamo / Initial consonants (U+1100--U+1112)
    /// * Hangul Compatibility Jamo / Consonant letters (U+3131--U+314E)
    /// * Halfwidth and Fullwidth Forms / Halfwidth Hangul variants (U+FFA1--U+FFBE)
    /// * Hangul Jamo / Old initial consonants (U+1113--U+115E)
    /// * Hangul Compatibility Jamo / Old consonant letters (U+3165--U+3186)
    /// * Hangul Jamo Extended-A / Old initial consonants (U+A960--U+A97C)
    ///
    /// The latter three ranges are considered valid if and only if `archaic-korean` feature is enabled.
    ///
    /// # Errors
    /// * [`Error::NonJongseongTryFromChar`]: the [`char`] given is not a valid final consonant.
    /// * [`Error::NonJaeumTryFromChar`]: the [`char`] given is not a valid consonant.
    /// * [`Error::NonKoreanTryFromChar`]: the [`char`] given is not a valid Korean alphabet at all.
    fn try_from(value: char) -> Result<Self, Self::Error> {
        let code_point = value as u32;

        match code_point {
            0x11A8..=0x11C2 => return Ok(Jongseong::try_from(code_point).unwrap()),
            0x1161..=0x1175
            | 0x314F..=0x3163
            | 0xFFC2..=0xFFC7
            | 0xFFCA..=0xFFCF
            | 0xFFD2..=0xFFD7
            | 0xFFDA..=0xFFDC => return Err(Error::NonJongseongTryFromChar(value)),
            0x1100..=0x1112 | 0x3131..=0x314E | 0xFFA1..=0xFFBE => {
                return Ok(Jongseong::try_from(Jaeum::try_from(value)?)?);
            }
            _ => {}
        }

        #[cfg(feature = "archaic-korean")]
        match code_point {
            0x11C3..=0x11FF | 0xD7CB..=0xD7FB => {
                return Ok(Jongseong::try_from(code_point).unwrap())
            }
            0x1176..=0x11A7 | 0x3187..=0x318E | 0xD7B0..=0xD7C6 => {
                return Err(Error::NonJongseongTryFromChar(value))
            }
            0x1113..=0x115E | 0x3165..=0x3186 | 0xA960..=0xA97C => {
                return Ok(Jongseong::try_from(Jaeum::try_from(value)?)?);
            }
            _ => {}
        }

        Err(Error::NonKoreanTryFromChar(value))
    }
}
impl TryFrom<Choseong> for Jongseong {
    type Error = Error;

    /// Tries to convert a [`Choseong`] into `Jongseong`, by leveraging existing conversion between `Choseong` and [`Jaeum`], and between `Jaeum` and `Jongseong`.
    ///
    /// Note: whether all `Jongseong`-capable `Choseong` successfully converts to `Jongseong` or not has not yet been fully tested, thus consider this function **unstable**.
    ///
    /// # Errors
    /// * [`Error::NonJongseongTryFromJaeum`]: the **`Choseong`** (actually, a `Jaeum`-ized `Choseong`) given is not valid in final consonant position.
    /// * [`Error::NonUnicodeJaeumTryFromChoseong`]: side effect from the approach this function took; until the conversion is fully validated, this error shall serve as a logical assertion.
    fn try_from(value: Choseong) -> Result<Self, Self::Error> {
        Self::try_from(Jaeum::try_from(value)?)
    }
}
impl TryFrom<Jaeum> for Jongseong {
    type Error = Error;

    /// Tries to convert a [`Jaeum`] into `Jongseong`.
    ///
    /// # Errors
    /// * [`Error::NonJongseongTryFromJaeum`]: the [`Jaeum`] given is not a valid final consonant.
    fn try_from(value: Jaeum) -> Result<Self, Self::Error> {
        // TODO: consider switching to bst; but i'm not very sure of performance boost it'll yield.
        match value {
            Jaeum::Kiyeok => Ok(Jongseong::Kiyeok),
            Jaeum::SsangKiyeok => Ok(Jongseong::SsangKiyeok),
            Jaeum::KiyeokSios => Ok(Jongseong::KiyeokSios),
            Jaeum::Nieun => Ok(Jongseong::Nieun),
            Jaeum::NieunCieuc => Ok(Jongseong::NieunCieuc),
            Jaeum::NieunHieuh => Ok(Jongseong::NieunHieuh),
            Jaeum::Tikeut => Ok(Jongseong::Tikeut),
            #[cfg(feature = "archaic-korean")]
            Jaeum::SsangTikeut => Ok(Jongseong::SsangTikeut),
            Jaeum::Rieul => Ok(Jongseong::Rieul),
            Jaeum::RieulKiyeok => Ok(Jongseong::RieulKiyeok),
            Jaeum::RieulMieum => Ok(Jongseong::RieulMieum),
            Jaeum::RieulPieup => Ok(Jongseong::RieulPieup),
            Jaeum::RieulSios => Ok(Jongseong::RieulSios),
            Jaeum::RieulThieuth => Ok(Jongseong::RieulThieuth),
            Jaeum::RieulPhieuph => Ok(Jongseong::RieulPhieuph),
            Jaeum::RieulHieuh => Ok(Jongseong::RieulHieuh),
            Jaeum::Mieum => Ok(Jongseong::Mieum),
            Jaeum::Pieup => Ok(Jongseong::Pieup),
            #[cfg(feature = "archaic-korean")]
            Jaeum::SsangPieup => Ok(Jongseong::SsangPieup),
            Jaeum::PieupSios => Ok(Jongseong::PieupSios),
            Jaeum::Sios => Ok(Jongseong::Sios),
            Jaeum::SsangSios => Ok(Jongseong::SsangSios),
            Jaeum::Ieung => Ok(Jongseong::Ieung),
            Jaeum::Cieuc => Ok(Jongseong::Cieuc),
            #[cfg(feature = "archaic-korean")]
            Jaeum::SsangCieuc => Ok(Jongseong::SsangCieuc),
            Jaeum::Chieuch => Ok(Jongseong::Chieuch),
            Jaeum::Khieukh => Ok(Jongseong::Khieukh),
            Jaeum::Thieuth => Ok(Jongseong::Thieuth),
            Jaeum::Phieuph => Ok(Jongseong::Phieuph),
            Jaeum::Hieuh => Ok(Jongseong::Hieuh),
            #[cfg(feature = "archaic-korean")]
            Jaeum::SsangNieun => Ok(Jongseong::SsangNieun),
            #[cfg(feature = "archaic-korean")]
            Jaeum::NieunTikeut => Ok(Jongseong::NieunTikeut),
            #[cfg(feature = "archaic-korean")]
            Jaeum::NieunSios => Ok(Jongseong::NieunSios),
            #[cfg(feature = "archaic-korean")]
            Jaeum::NieunPanSios => Ok(Jongseong::NieunPanSios),
            #[cfg(feature = "archaic-korean")]
            Jaeum::RieulKiyeokSios => Ok(Jongseong::RieulKiyeokSios),
            #[cfg(feature = "archaic-korean")]
            Jaeum::RieulTikeut => Ok(Jongseong::RieulTikeut),
            #[cfg(feature = "archaic-korean")]
            Jaeum::RieulPieupSios => Ok(Jongseong::RieulPieupSios),
            #[cfg(feature = "archaic-korean")]
            Jaeum::RieulPanSios => Ok(Jongseong::RieulPanSios),
            #[cfg(feature = "archaic-korean")]
            Jaeum::RieulYeorinHieuh => Ok(Jongseong::RieulYeorinHieuh),
            #[cfg(feature = "archaic-korean")]
            Jaeum::MieumPieup => Ok(Jongseong::MieumPieup),
            #[cfg(feature = "archaic-korean")]
            Jaeum::MieumSios => Ok(Jongseong::MieumSios),
            #[cfg(feature = "archaic-korean")]
            Jaeum::MieumPanSios => Ok(Jongseong::MieumPanSios),
            #[cfg(feature = "archaic-korean")]
            Jaeum::KapyeounMieum => Ok(Jongseong::KapyeounMieum),
            #[cfg(feature = "archaic-korean")]
            Jaeum::PieupTikeut => Ok(Jongseong::PieupTikeut),
            #[cfg(feature = "archaic-korean")]
            Jaeum::PieupSiosTikeut => Ok(Jongseong::PieupSiosTikeut),
            #[cfg(feature = "archaic-korean")]
            Jaeum::PieupCieuc => Ok(Jongseong::PieupCieuc),
            #[cfg(feature = "archaic-korean")]
            Jaeum::KapyeounPieup => Ok(Jongseong::KapyeounPieup),
            #[cfg(feature = "archaic-korean")]
            Jaeum::SiosKiyeok => Ok(Jongseong::SiosKiyeok),
            #[cfg(feature = "archaic-korean")]
            Jaeum::SiosTikeut => Ok(Jongseong::SiosTikeut),
            #[cfg(feature = "archaic-korean")]
            Jaeum::SiosPieup => Ok(Jongseong::SiosPieup),
            #[cfg(feature = "archaic-korean")]
            Jaeum::SiosCieuc => Ok(Jongseong::SiosCieuc),
            #[cfg(feature = "archaic-korean")]
            Jaeum::PanSios => Ok(Jongseong::PanSios),
            #[cfg(feature = "archaic-korean")]
            Jaeum::SsangIeung => Ok(Jongseong::SsangIeung),
            #[cfg(feature = "archaic-korean")]
            Jaeum::YesIeung => Ok(Jongseong::YesIeung),
            #[cfg(feature = "archaic-korean")]
            Jaeum::YesIeungSios => Ok(Jongseong::YesIeungSios),
            #[cfg(feature = "archaic-korean")]
            Jaeum::YesIeungPanSios => Ok(Jongseong::YesIeungPanSios),
            #[cfg(feature = "archaic-korean")]
            Jaeum::KapyeounPhieuph => Ok(Jongseong::KapyeounPhieuph),
            #[cfg(feature = "archaic-korean")]
            Jaeum::YeorinHieuh => Ok(Jongseong::YeorinHieuh),
            _ => Err(Error::NonJongseongTryFromJaeum(value)),
        }
    }
}
impl Jongseong {
    // TODO: consider exporting this array even when `archaic-korean` feature is not selected.
    //       prob will require getting enum size at compile time kind of special magic thing.
    #[cfg(feature = "archaic-korean")]
    /// Lists `Jongseong` in correct dictionary order.
    pub const IN_ORDER: [Jongseong; 137] = [
        Jongseong::Kiyeok,
        Jongseong::SsangKiyeok,
        #[cfg(feature = "archaic-korean")]
        Jongseong::KiyeokNieun,
        #[cfg(feature = "archaic-korean")]
        Jongseong::KiyeokRieul,
        #[cfg(feature = "archaic-korean")]
        Jongseong::KiyeokPieup,
        Jongseong::KiyeokSios,
        #[cfg(feature = "archaic-korean")]
        Jongseong::KiyeokSiosKiyeok,
        #[cfg(feature = "archaic-korean")]
        Jongseong::KiyeokChieuch,
        #[cfg(feature = "archaic-korean")]
        Jongseong::KiyeokKhieukh,
        #[cfg(feature = "archaic-korean")]
        Jongseong::KiyeokHieuh,
        Jongseong::Nieun,
        #[cfg(feature = "archaic-korean")]
        Jongseong::NieunKiyeok,
        #[cfg(feature = "archaic-korean")]
        Jongseong::SsangNieun,
        #[cfg(feature = "archaic-korean")]
        Jongseong::NieunTikeut,
        #[cfg(feature = "archaic-korean")]
        Jongseong::NieunRieul,
        #[cfg(feature = "archaic-korean")]
        Jongseong::NieunSios,
        #[cfg(feature = "archaic-korean")]
        Jongseong::NieunPanSios,
        Jongseong::NieunCieuc,
        #[cfg(feature = "archaic-korean")]
        Jongseong::NieunChieuch,
        #[cfg(feature = "archaic-korean")]
        Jongseong::NieunThieuth,
        Jongseong::NieunHieuh,
        Jongseong::Tikeut,
        #[cfg(feature = "archaic-korean")]
        Jongseong::TikeutKiyeok,
        #[cfg(feature = "archaic-korean")]
        Jongseong::SsangTikeut,
        #[cfg(feature = "archaic-korean")]
        Jongseong::SsangTikeutPieup,
        #[cfg(feature = "archaic-korean")]
        Jongseong::TikeutRieul,
        #[cfg(feature = "archaic-korean")]
        Jongseong::TikeutPieup,
        #[cfg(feature = "archaic-korean")]
        Jongseong::TikeutSios,
        #[cfg(feature = "archaic-korean")]
        Jongseong::TikeutSiosKiyeok,
        #[cfg(feature = "archaic-korean")]
        Jongseong::TikeutCieuc,
        #[cfg(feature = "archaic-korean")]
        Jongseong::TikeutChieuch,
        #[cfg(feature = "archaic-korean")]
        Jongseong::TikeutThieuth,
        Jongseong::Rieul,
        Jongseong::RieulKiyeok,
        #[cfg(feature = "archaic-korean")]
        Jongseong::RieulSsangKiyeok,
        #[cfg(feature = "archaic-korean")]
        Jongseong::RieulKiyeokSios,
        #[cfg(feature = "archaic-korean")]
        Jongseong::RieulKiyeokHieuh,
        #[cfg(feature = "archaic-korean")]
        Jongseong::RieulNieun,
        #[cfg(feature = "archaic-korean")]
        Jongseong::RieulTikeut,
        #[cfg(feature = "archaic-korean")]
        Jongseong::RieulTikeutHieuh,
        #[cfg(feature = "archaic-korean")]
        Jongseong::SsangRieul,
        #[cfg(feature = "archaic-korean")]
        Jongseong::SsangRieulKhieukh,
        Jongseong::RieulMieum,
        #[cfg(feature = "archaic-korean")]
        Jongseong::RieulMieumKiyeok,
        #[cfg(feature = "archaic-korean")]
        Jongseong::RieulMieumSios,
        #[cfg(feature = "archaic-korean")]
        Jongseong::RieulMieumHieuh,
        Jongseong::RieulPieup,
        #[cfg(feature = "archaic-korean")]
        Jongseong::RieulPieupTikeut,
        #[cfg(feature = "archaic-korean")]
        Jongseong::RieulPieupSios,
        #[cfg(feature = "archaic-korean")]
        Jongseong::RieulPieupPhieuph,
        #[cfg(feature = "archaic-korean")]
        Jongseong::RieulPieupHieuh,
        #[cfg(feature = "archaic-korean")]
        Jongseong::RieulKapyeounPieup,
        Jongseong::RieulSios,
        #[cfg(feature = "archaic-korean")]
        Jongseong::RieulSsangSios,
        #[cfg(feature = "archaic-korean")]
        Jongseong::RieulPanSios,
        #[cfg(feature = "archaic-korean")]
        Jongseong::RieulYesIeung,
        #[cfg(feature = "archaic-korean")]
        Jongseong::RieulKhieukh,
        Jongseong::RieulThieuth,
        Jongseong::RieulPhieuph,
        Jongseong::RieulHieuh,
        #[cfg(feature = "archaic-korean")]
        Jongseong::RieulYeorinHieuh,
        #[cfg(feature = "archaic-korean")]
        Jongseong::RieulYeorinHieuhHieuh,
        #[cfg(feature = "archaic-korean")]
        Jongseong::KapyeounRieul,
        Jongseong::Mieum,
        #[cfg(feature = "archaic-korean")]
        Jongseong::MieumKiyeok,
        #[cfg(feature = "archaic-korean")]
        Jongseong::MieumNieun,
        #[cfg(feature = "archaic-korean")]
        Jongseong::MieumSsangNieun,
        #[cfg(feature = "archaic-korean")]
        Jongseong::MieumRieul,
        #[cfg(feature = "archaic-korean")]
        Jongseong::SsangMieum,
        #[cfg(feature = "archaic-korean")]
        Jongseong::MieumPieup,
        #[cfg(feature = "archaic-korean")]
        Jongseong::MieumPieupSios,
        #[cfg(feature = "archaic-korean")]
        Jongseong::MieumSios,
        #[cfg(feature = "archaic-korean")]
        Jongseong::MieumSsangSios,
        #[cfg(feature = "archaic-korean")]
        Jongseong::MieumPanSios,
        #[cfg(feature = "archaic-korean")]
        Jongseong::MieumCieuc,
        #[cfg(feature = "archaic-korean")]
        Jongseong::MieumChieuch,
        #[cfg(feature = "archaic-korean")]
        Jongseong::MieumHieuh,
        #[cfg(feature = "archaic-korean")]
        Jongseong::KapyeounMieum,
        Jongseong::Pieup,
        #[cfg(feature = "archaic-korean")]
        Jongseong::PieupTikeut,
        #[cfg(feature = "archaic-korean")]
        Jongseong::PieupRieul,
        #[cfg(feature = "archaic-korean")]
        Jongseong::PieupRieulPhieuph,
        #[cfg(feature = "archaic-korean")]
        Jongseong::PieupMieum,
        #[cfg(feature = "archaic-korean")]
        Jongseong::SsangPieup,
        Jongseong::PieupSios,
        #[cfg(feature = "archaic-korean")]
        Jongseong::PieupSiosTikeut,
        #[cfg(feature = "archaic-korean")]
        Jongseong::PieupCieuc,
        #[cfg(feature = "archaic-korean")]
        Jongseong::PieupChieuch,
        #[cfg(feature = "archaic-korean")]
        Jongseong::PieupPhieuph,
        #[cfg(feature = "archaic-korean")]
        Jongseong::PieupHieuh,
        #[cfg(feature = "archaic-korean")]
        Jongseong::KapyeounPieup,
        Jongseong::Sios,
        #[cfg(feature = "archaic-korean")]
        Jongseong::SiosKiyeok,
        #[cfg(feature = "archaic-korean")]
        Jongseong::SiosTikeut,
        #[cfg(feature = "archaic-korean")]
        Jongseong::SiosRieul,
        #[cfg(feature = "archaic-korean")]
        Jongseong::SiosMieum,
        #[cfg(feature = "archaic-korean")]
        Jongseong::SiosPieup,
        #[cfg(feature = "archaic-korean")]
        Jongseong::SiosKapyeounPieup,
        Jongseong::SsangSios,
        #[cfg(feature = "archaic-korean")]
        Jongseong::SsangSiosKiyeok,
        #[cfg(feature = "archaic-korean")]
        Jongseong::SsangSiosTikeut,
        #[cfg(feature = "archaic-korean")]
        Jongseong::SiosPanSios,
        #[cfg(feature = "archaic-korean")]
        Jongseong::SiosCieuc,
        #[cfg(feature = "archaic-korean")]
        Jongseong::SiosChieuch,
        #[cfg(feature = "archaic-korean")]
        Jongseong::SiosThieuth,
        #[cfg(feature = "archaic-korean")]
        Jongseong::SiosHieuh,
        #[cfg(feature = "archaic-korean")]
        Jongseong::PanSios,
        #[cfg(feature = "archaic-korean")]
        Jongseong::PanSiosPieup,
        #[cfg(feature = "archaic-korean")]
        Jongseong::PanSiosKapyeounPieup,
        Jongseong::Ieung,
        #[cfg(feature = "archaic-korean")]
        Jongseong::YesIeung,
        #[cfg(feature = "archaic-korean")]
        Jongseong::IeungKiyeok,
        #[cfg(feature = "archaic-korean")]
        Jongseong::IeungSsangKiyeok,
        #[cfg(feature = "archaic-korean")]
        Jongseong::YesIeungMieum,
        #[cfg(feature = "archaic-korean")]
        Jongseong::YesIeungSios,
        #[cfg(feature = "archaic-korean")]
        Jongseong::YesIeungPanSios,
        #[cfg(feature = "archaic-korean")]
        Jongseong::SsangIeung,
        #[cfg(feature = "archaic-korean")]
        Jongseong::IeungKhieukh,
        #[cfg(feature = "archaic-korean")]
        Jongseong::YesIeungHieuh,
        Jongseong::Cieuc,
        #[cfg(feature = "archaic-korean")]
        Jongseong::CieucPieup,
        #[cfg(feature = "archaic-korean")]
        Jongseong::CieucSsangPieup,
        #[cfg(feature = "archaic-korean")]
        Jongseong::SsangCieuc,
        Jongseong::Chieuch,
        Jongseong::Khieukh,
        Jongseong::Thieuth,
        Jongseong::Phieuph,
        #[cfg(feature = "archaic-korean")]
        Jongseong::PhieuphPieup,
        #[cfg(feature = "archaic-korean")]
        Jongseong::PhieuphSios,
        #[cfg(feature = "archaic-korean")]
        Jongseong::PhieuphThieuth,
        #[cfg(feature = "archaic-korean")]
        Jongseong::KapyeounPhieuph,
        Jongseong::Hieuh,
        #[cfg(feature = "archaic-korean")]
        Jongseong::HieuhNieun,
        #[cfg(feature = "archaic-korean")]
        Jongseong::HieuhRieul,
        #[cfg(feature = "archaic-korean")]
        Jongseong::HieuhMieum,
        #[cfg(feature = "archaic-korean")]
        Jongseong::HieuhPieup,
        #[cfg(feature = "archaic-korean")]
        Jongseong::YeorinHieuh,
    ];
}
