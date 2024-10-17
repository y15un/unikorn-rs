use crate::{
    consonant::{Choseong, HalfwidthJaeum, Jaeum},
    Error,
};
use num_enum::{IntoPrimitive, TryFromPrimitive};
use std::{
    convert::TryFrom,
    fmt::{Display, Formatter, Result as FmtResult},
};

/// A set of consonants valid as final consonant (종성, Jongseong).
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
impl Display for Jongseong {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", char::from(*self))
    }
}
impl From<Jongseong> for char {
    fn from(value: Jongseong) -> Self {
        // guaranteed to not fail within BMP
        char::from_u32(value as u32).unwrap()
    }
}
impl TryFrom<char> for Jongseong {
    type Error = Error;

    /// Tries to convert a [`char`] into [`Jongseong`].
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
    /// ## Via [`Choseong`]
    /// For the following range(s), the [`char`] given will be tried converting into [`Choseong`] first, then subsequently into [`Jongseong`]:
    ///
    /// * Hangul Jamo / Initial consonants (U+1100--U+1112)
    /// * Hangul Jamo / Old initial consonants (U+1113--U+115E)
    /// * Hangul Jamo Extended-A / Old initial consonants (U+A960--U+A97C)
    ///
    /// The latter two ranges are considered valid if and only if `archaic-korean` feature is enabled.
    ///
    /// ## Via [`Jaeum`] or [`HalfwidthJaeum`]
    /// For the following range(s), the [`char`] given will be tried converting into [`Jaeum`] or [`HalfwidthJaeum`] accordingly first, then subsequently into [`Jongseong`]:
    ///
    /// * Hangul Compatibility Jamo / Consonant letters (U+3131--U+314E)
    /// * Halfwidth and Fullwidth Forms / Halfwidth Hangul variants (U+FFA1--U+FFBE)
    /// * Hangul Compatibility Jamo / Old consonant letters (U+3165--U+3186)
    ///
    /// The latter one range is considered valid if and only if `archaic-korean` feature is enabled.
    ///
    /// # Errors
    /// * [`Error::NonJongseongTryFromChar`]: the [`char`] given is not valid as final consonant.
    /// * [`Error::NonJaeumTryFromChar`]: the [`char`] given is not a valid consonant.
    /// * [`Error::NonKoreanTryFromChar`]: the [`char`] given is not a valid Korean letter.
    fn try_from(value: char) -> Result<Self, Self::Error> {
        let code_point = value as u32;
        let najong = Err(Error::NonJongseongTryFromChar(value));

        match code_point {
            // initial consonants
            0x1100..=0x1112 => {
                return Self::try_from(Choseong::try_from(code_point).unwrap()).or(najong)
            }
            // vowels
            0x1161..=0x1175
            | 0x314F..=0x3163
            | 0xFFC2..=0xFFC7
            | 0xFFCA..=0xFFCF
            | 0xFFD2..=0xFFD7
            | 0xFFDA..=0xFFDC => return Err(Error::NonJaeumTryFromChar(value)),
            // final consonants (self)
            0x11A8..=0x11C2 => return Ok(Self::try_from(code_point).unwrap()),
            // generic consonants
            0x3131..=0x314E => {
                return Self::try_from(Jaeum::try_from(code_point).unwrap()).or(najong)
            }
            // halfwidth consonants
            0xFFA1..=0xFFBE => {
                return Self::try_from(HalfwidthJaeum::try_from(code_point).unwrap()).or(najong)
            }
            _ => {}
        }

        #[cfg(feature = "archaic-korean")]
        match code_point {
            // old initial consonants
            0x1113..=0x115E | 0xA960..=0xA97C => {
                return Self::try_from(Choseong::try_from(code_point).unwrap()).or(najong)
            }
            // old vowels
            0x1176..=0x11A7 | 0x3187..=0x318E | 0xD7B0..=0xD7C6 => {
                return Err(Error::NonJaeumTryFromChar(value))
            }
            // old final consonants (self)
            0x11C3..=0x11FF | 0xD7CB..=0xD7FB => return Ok(Self::try_from(code_point).unwrap()),
            // old generic consonants
            0x3165..=0x3186 => {
                return Self::try_from(Jaeum::try_from(code_point).unwrap()).or(najong)
            }
            _ => {}
        }

        Err(Error::NonKoreanTryFromChar(value))
    }
}
impl TryFrom<Choseong> for Jongseong {
    type Error = Error;

    /// Tries to convert a [`Choseong`] into [`Jongseong`].
    ///
    /// # Errors
    /// * [`Error::NonJongseongTryFromChoseong`]: the [`Choseong`] given is not valid as final consonant.
    fn try_from(value: Choseong) -> Result<Self, Self::Error> {
        // TODO: consider switching to bst; but i'm not very sure of performance boost it'll yield.
        match value {
            Choseong::Kiyeok => Ok(Self::Kiyeok),
            Choseong::SsangKiyeok => Ok(Self::SsangKiyeok),
            Choseong::Nieun => Ok(Self::Nieun),
            Choseong::Tikeut => Ok(Self::Tikeut),
            #[cfg(feature = "archaic-korean")]
            Choseong::SsangTikeut => Ok(Self::SsangTikeut),
            Choseong::Rieul => Ok(Self::Rieul),
            Choseong::Mieum => Ok(Self::Mieum),
            Choseong::Pieup => Ok(Self::Pieup),
            #[cfg(feature = "archaic-korean")]
            Choseong::SsangPieup => Ok(Self::SsangPieup),
            Choseong::Sios => Ok(Self::Sios),
            Choseong::SsangSios => Ok(Self::SsangSios),
            Choseong::Ieung => Ok(Self::Ieung),
            Choseong::Cieuc => Ok(Self::Cieuc),
            #[cfg(feature = "archaic-korean")]
            Choseong::SsangCieuc => Ok(Self::SsangCieuc),
            Choseong::Chieuch => Ok(Self::Chieuch),
            Choseong::Khieukh => Ok(Self::Khieukh),
            Choseong::Thieuth => Ok(Self::Thieuth),
            Choseong::Phieuph => Ok(Self::Phieuph),
            Choseong::Hieuh => Ok(Self::Hieuh),
            #[cfg(feature = "archaic-korean")]
            Choseong::NieunKiyeok => Ok(Self::NieunKiyeok),
            #[cfg(feature = "archaic-korean")]
            Choseong::SsangNieun => Ok(Self::SsangNieun),
            #[cfg(feature = "archaic-korean")]
            Choseong::NieunTikeut => Ok(Self::NieunTikeut),
            #[cfg(feature = "archaic-korean")]
            Choseong::TikeutKiyeok => Ok(Self::TikeutKiyeok),
            #[cfg(feature = "archaic-korean")]
            Choseong::RieulNieun => Ok(Self::RieulNieun),
            #[cfg(feature = "archaic-korean")]
            Choseong::SsangRieul => Ok(Self::SsangRieul),
            #[cfg(feature = "archaic-korean")]
            Choseong::RieulHieuh => Ok(Self::RieulHieuh),
            #[cfg(feature = "archaic-korean")]
            Choseong::KapyeounRieul => Ok(Self::KapyeounRieul),
            #[cfg(feature = "archaic-korean")]
            Choseong::MieumPieup => Ok(Self::MieumPieup),
            #[cfg(feature = "archaic-korean")]
            Choseong::KapyeounMieum => Ok(Self::KapyeounMieum),
            #[cfg(feature = "archaic-korean")]
            Choseong::PieupTikeut => Ok(Self::PieupTikeut),
            #[cfg(feature = "archaic-korean")]
            Choseong::PieupSios => Ok(Self::PieupSios),
            #[cfg(feature = "archaic-korean")]
            Choseong::PieupSiosTikeut => Ok(Self::PieupSiosTikeut),
            #[cfg(feature = "archaic-korean")]
            Choseong::PieupCieuc => Ok(Self::PieupCieuc),
            #[cfg(feature = "archaic-korean")]
            Choseong::PieupChieuch => Ok(Self::PieupChieuch),
            #[cfg(feature = "archaic-korean")]
            Choseong::PieupPhieuph => Ok(Self::PieupPhieuph),
            #[cfg(feature = "archaic-korean")]
            Choseong::KapyeounPieup => Ok(Self::KapyeounPieup),
            #[cfg(feature = "archaic-korean")]
            Choseong::SiosKiyeok => Ok(Self::SiosKiyeok),
            #[cfg(feature = "archaic-korean")]
            Choseong::SiosTikeut => Ok(Self::SiosTikeut),
            #[cfg(feature = "archaic-korean")]
            Choseong::SiosRieul => Ok(Self::SiosRieul),
            #[cfg(feature = "archaic-korean")]
            Choseong::SiosMieum => Ok(Self::SiosMieum),
            #[cfg(feature = "archaic-korean")]
            Choseong::SiosPieup => Ok(Self::SiosPieup),
            #[cfg(feature = "archaic-korean")]
            Choseong::SiosCieuc => Ok(Self::SiosCieuc),
            #[cfg(feature = "archaic-korean")]
            Choseong::SiosChieuch => Ok(Self::SiosChieuch),
            #[cfg(feature = "archaic-korean")]
            Choseong::SiosThieuth => Ok(Self::SiosThieuth),
            #[cfg(feature = "archaic-korean")]
            Choseong::SiosHieuh => Ok(Self::SiosHieuh),
            #[cfg(feature = "archaic-korean")]
            Choseong::PanSios => Ok(Self::PanSios),
            #[cfg(feature = "archaic-korean")]
            Choseong::IeungKiyeok => Ok(Self::IeungKiyeok),
            #[cfg(feature = "archaic-korean")]
            Choseong::SsangIeung => Ok(Self::SsangIeung),
            #[cfg(feature = "archaic-korean")]
            Choseong::YesIeung => Ok(Self::YesIeung),
            #[cfg(feature = "archaic-korean")]
            Choseong::PhieuphPieup => Ok(Self::PhieuphPieup),
            #[cfg(feature = "archaic-korean")]
            Choseong::KapyeounPhieuph => Ok(Self::KapyeounPhieuph),
            #[cfg(feature = "archaic-korean")]
            Choseong::YeorinHieuh => Ok(Self::YeorinHieuh),
            #[cfg(feature = "archaic-korean")]
            Choseong::NieunSios => Ok(Self::NieunSios),
            #[cfg(feature = "archaic-korean")]
            Choseong::NieunCieuc => Ok(Self::NieunCieuc),
            #[cfg(feature = "archaic-korean")]
            Choseong::NieunHieuh => Ok(Self::NieunHieuh),
            #[cfg(feature = "archaic-korean")]
            Choseong::TikeutRieul => Ok(Self::TikeutRieul),
            #[cfg(feature = "archaic-korean")]
            Choseong::TikeutPieup => Ok(Self::TikeutPieup),
            #[cfg(feature = "archaic-korean")]
            Choseong::TikeutSios => Ok(Self::TikeutSios),
            #[cfg(feature = "archaic-korean")]
            Choseong::TikeutCieuc => Ok(Self::TikeutCieuc),
            #[cfg(feature = "archaic-korean")]
            Choseong::RieulKiyeok => Ok(Self::RieulKiyeok),
            #[cfg(feature = "archaic-korean")]
            Choseong::RieulSsangKiyeok => Ok(Self::RieulSsangKiyeok),
            #[cfg(feature = "archaic-korean")]
            Choseong::RieulTikeut => Ok(Self::RieulTikeut),
            #[cfg(feature = "archaic-korean")]
            Choseong::RieulMieum => Ok(Self::RieulMieum),
            #[cfg(feature = "archaic-korean")]
            Choseong::RieulPieup => Ok(Self::RieulPieup),
            #[cfg(feature = "archaic-korean")]
            Choseong::RieulKapyeounPieup => Ok(Self::RieulKapyeounPieup),
            #[cfg(feature = "archaic-korean")]
            Choseong::RieulSios => Ok(Self::RieulSios),
            #[cfg(feature = "archaic-korean")]
            Choseong::RieulKhieukh => Ok(Self::RieulKhieukh),
            #[cfg(feature = "archaic-korean")]
            Choseong::MieumKiyeok => Ok(Self::MieumKiyeok),
            #[cfg(feature = "archaic-korean")]
            Choseong::MieumSios => Ok(Self::MieumSios),
            #[cfg(feature = "archaic-korean")]
            Choseong::PieupHieuh => Ok(Self::PieupHieuh),
            _ => Err(Error::NonJongseongTryFromChoseong(value)),
        }
    }
}
impl TryFrom<HalfwidthJaeum> for Jongseong {
    type Error = Error;

    /// Tries to convert a [`HalfwidthJaeum`] into [`Jongseong`].
    ///
    /// # Errors
    /// * [`Error::NonJongseongTryFromHalfwidthJaeum`]: the [`HalfwidthJaeum`] given is not valid as final consonant.
    fn try_from(value: HalfwidthJaeum) -> Result<Self, Self::Error> {
        // TODO: consider switching to bst; but i'm not very sure of performance boost it'll yield.
        match value {
            HalfwidthJaeum::Kiyeok => Ok(Self::Kiyeok),
            HalfwidthJaeum::SsangKiyeok => Ok(Self::SsangKiyeok),
            HalfwidthJaeum::KiyeokSios => Ok(Self::KiyeokSios),
            HalfwidthJaeum::Nieun => Ok(Self::Nieun),
            HalfwidthJaeum::NieunCieuc => Ok(Self::NieunCieuc),
            HalfwidthJaeum::NieunHieuh => Ok(Self::NieunHieuh),
            HalfwidthJaeum::Tikeut => Ok(Self::Tikeut),
            #[cfg(feature = "archaic-korean")]
            HalfwidthJaeum::SsangTikeut => Ok(Self::SsangTikeut),
            HalfwidthJaeum::Rieul => Ok(Self::Rieul),
            HalfwidthJaeum::RieulKiyeok => Ok(Self::RieulKiyeok),
            HalfwidthJaeum::RieulMieum => Ok(Self::RieulMieum),
            HalfwidthJaeum::RieulPieup => Ok(Self::RieulPieup),
            HalfwidthJaeum::RieulSios => Ok(Self::RieulSios),
            HalfwidthJaeum::RieulThieuth => Ok(Self::RieulThieuth),
            HalfwidthJaeum::RieulPhieuph => Ok(Self::RieulPhieuph),
            HalfwidthJaeum::RieulHieuh => Ok(Self::RieulHieuh),
            HalfwidthJaeum::Mieum => Ok(Self::Mieum),
            HalfwidthJaeum::Pieup => Ok(Self::Pieup),
            #[cfg(feature = "archaic-korean")]
            HalfwidthJaeum::SsangPieup => Ok(Self::SsangPieup),
            HalfwidthJaeum::PieupSios => Ok(Self::PieupSios),
            HalfwidthJaeum::Sios => Ok(Self::Sios),
            HalfwidthJaeum::SsangSios => Ok(Self::SsangSios),
            HalfwidthJaeum::Ieung => Ok(Self::Ieung),
            HalfwidthJaeum::Cieuc => Ok(Self::Cieuc),
            #[cfg(feature = "archaic-korean")]
            HalfwidthJaeum::SsangCieuc => Ok(Self::SsangCieuc),
            HalfwidthJaeum::Chieuch => Ok(Self::Chieuch),
            HalfwidthJaeum::Khieukh => Ok(Self::Khieukh),
            HalfwidthJaeum::Thieuth => Ok(Self::Thieuth),
            HalfwidthJaeum::Phieuph => Ok(Self::Phieuph),
            HalfwidthJaeum::Hieuh => Ok(Self::Hieuh),
            #[cfg(not(feature = "archaic-korean"))]
            _ => Err(Error::NonJongseongTryFromHalfwidthJaeum(value)),
        }
    }
}
impl TryFrom<Jaeum> for Jongseong {
    type Error = Error;

    /// Tries to convert a [`Jaeum`] into [`Jongseong`].
    ///
    /// # Errors
    /// * [`Error::NonJongseongTryFromJaeum`]: the [`Jaeum`] given is not valid as final consonant.
    fn try_from(value: Jaeum) -> Result<Self, Self::Error> {
        // TODO: consider switching to bst; but i'm not very sure of performance boost it'll yield.
        match value {
            Jaeum::Kiyeok => Ok(Self::Kiyeok),
            Jaeum::SsangKiyeok => Ok(Self::SsangKiyeok),
            Jaeum::KiyeokSios => Ok(Self::KiyeokSios),
            Jaeum::Nieun => Ok(Self::Nieun),
            Jaeum::NieunCieuc => Ok(Self::NieunCieuc),
            Jaeum::NieunHieuh => Ok(Self::NieunHieuh),
            Jaeum::Tikeut => Ok(Self::Tikeut),
            #[cfg(feature = "archaic-korean")]
            Jaeum::SsangTikeut => Ok(Self::SsangTikeut),
            Jaeum::Rieul => Ok(Self::Rieul),
            Jaeum::RieulKiyeok => Ok(Self::RieulKiyeok),
            Jaeum::RieulMieum => Ok(Self::RieulMieum),
            Jaeum::RieulPieup => Ok(Self::RieulPieup),
            Jaeum::RieulSios => Ok(Self::RieulSios),
            Jaeum::RieulThieuth => Ok(Self::RieulThieuth),
            Jaeum::RieulPhieuph => Ok(Self::RieulPhieuph),
            Jaeum::RieulHieuh => Ok(Self::RieulHieuh),
            Jaeum::Mieum => Ok(Self::Mieum),
            Jaeum::Pieup => Ok(Self::Pieup),
            #[cfg(feature = "archaic-korean")]
            Jaeum::SsangPieup => Ok(Self::SsangPieup),
            Jaeum::PieupSios => Ok(Self::PieupSios),
            Jaeum::Sios => Ok(Self::Sios),
            Jaeum::SsangSios => Ok(Self::SsangSios),
            Jaeum::Ieung => Ok(Self::Ieung),
            Jaeum::Cieuc => Ok(Self::Cieuc),
            #[cfg(feature = "archaic-korean")]
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
            Jaeum::NieunPanSios => Ok(Self::NieunPanSios),
            #[cfg(feature = "archaic-korean")]
            Jaeum::RieulKiyeokSios => Ok(Self::RieulKiyeokSios),
            #[cfg(feature = "archaic-korean")]
            Jaeum::RieulTikeut => Ok(Self::RieulTikeut),
            #[cfg(feature = "archaic-korean")]
            Jaeum::RieulPieupSios => Ok(Self::RieulPieupSios),
            #[cfg(feature = "archaic-korean")]
            Jaeum::RieulPanSios => Ok(Self::RieulPanSios),
            #[cfg(feature = "archaic-korean")]
            Jaeum::RieulYeorinHieuh => Ok(Self::RieulYeorinHieuh),
            #[cfg(feature = "archaic-korean")]
            Jaeum::MieumPieup => Ok(Self::MieumPieup),
            #[cfg(feature = "archaic-korean")]
            Jaeum::MieumSios => Ok(Self::MieumSios),
            #[cfg(feature = "archaic-korean")]
            Jaeum::MieumPanSios => Ok(Self::MieumPanSios),
            #[cfg(feature = "archaic-korean")]
            Jaeum::KapyeounMieum => Ok(Self::KapyeounMieum),
            #[cfg(feature = "archaic-korean")]
            Jaeum::PieupTikeut => Ok(Self::PieupTikeut),
            #[cfg(feature = "archaic-korean")]
            Jaeum::PieupSiosTikeut => Ok(Self::PieupSiosTikeut),
            #[cfg(feature = "archaic-korean")]
            Jaeum::PieupCieuc => Ok(Self::PieupCieuc),
            #[cfg(feature = "archaic-korean")]
            Jaeum::KapyeounPieup => Ok(Self::KapyeounPieup),
            #[cfg(feature = "archaic-korean")]
            Jaeum::SiosKiyeok => Ok(Self::SiosKiyeok),
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
            Jaeum::YesIeungSios => Ok(Self::YesIeungSios),
            #[cfg(feature = "archaic-korean")]
            Jaeum::YesIeungPanSios => Ok(Self::YesIeungPanSios),
            #[cfg(feature = "archaic-korean")]
            Jaeum::KapyeounPhieuph => Ok(Self::KapyeounPhieuph),
            #[cfg(feature = "archaic-korean")]
            Jaeum::YeorinHieuh => Ok(Self::YeorinHieuh),
            _ => Err(Error::NonJongseongTryFromJaeum(value)),
        }
    }
}
impl Jongseong {
    // This list is only exported with `archaic-korean` feature, because without it the [`Jongseong`] should be in order by itself.
    #[cfg(feature = "archaic-korean")]
    /// Lists [`Jongseong`] in correct dictionary order.
    pub const IN_ORDER: [Self; 137] = [
        Self::Kiyeok,
        Self::SsangKiyeok,
        #[cfg(feature = "archaic-korean")]
        Self::KiyeokNieun,
        #[cfg(feature = "archaic-korean")]
        Self::KiyeokRieul,
        #[cfg(feature = "archaic-korean")]
        Self::KiyeokPieup,
        Self::KiyeokSios,
        #[cfg(feature = "archaic-korean")]
        Self::KiyeokSiosKiyeok,
        #[cfg(feature = "archaic-korean")]
        Self::KiyeokChieuch,
        #[cfg(feature = "archaic-korean")]
        Self::KiyeokKhieukh,
        #[cfg(feature = "archaic-korean")]
        Self::KiyeokHieuh,
        Self::Nieun,
        #[cfg(feature = "archaic-korean")]
        Self::NieunKiyeok,
        #[cfg(feature = "archaic-korean")]
        Self::SsangNieun,
        #[cfg(feature = "archaic-korean")]
        Self::NieunTikeut,
        #[cfg(feature = "archaic-korean")]
        Self::NieunRieul,
        #[cfg(feature = "archaic-korean")]
        Self::NieunSios,
        #[cfg(feature = "archaic-korean")]
        Self::NieunPanSios,
        Self::NieunCieuc,
        #[cfg(feature = "archaic-korean")]
        Self::NieunChieuch,
        #[cfg(feature = "archaic-korean")]
        Self::NieunThieuth,
        Self::NieunHieuh,
        Self::Tikeut,
        #[cfg(feature = "archaic-korean")]
        Self::TikeutKiyeok,
        #[cfg(feature = "archaic-korean")]
        Self::SsangTikeut,
        #[cfg(feature = "archaic-korean")]
        Self::SsangTikeutPieup,
        #[cfg(feature = "archaic-korean")]
        Self::TikeutRieul,
        #[cfg(feature = "archaic-korean")]
        Self::TikeutPieup,
        #[cfg(feature = "archaic-korean")]
        Self::TikeutSios,
        #[cfg(feature = "archaic-korean")]
        Self::TikeutSiosKiyeok,
        #[cfg(feature = "archaic-korean")]
        Self::TikeutCieuc,
        #[cfg(feature = "archaic-korean")]
        Self::TikeutChieuch,
        #[cfg(feature = "archaic-korean")]
        Self::TikeutThieuth,
        Self::Rieul,
        Self::RieulKiyeok,
        #[cfg(feature = "archaic-korean")]
        Self::RieulSsangKiyeok,
        #[cfg(feature = "archaic-korean")]
        Self::RieulKiyeokSios,
        #[cfg(feature = "archaic-korean")]
        Self::RieulKiyeokHieuh,
        #[cfg(feature = "archaic-korean")]
        Self::RieulNieun,
        #[cfg(feature = "archaic-korean")]
        Self::RieulTikeut,
        #[cfg(feature = "archaic-korean")]
        Self::RieulTikeutHieuh,
        #[cfg(feature = "archaic-korean")]
        Self::SsangRieul,
        #[cfg(feature = "archaic-korean")]
        Self::SsangRieulKhieukh,
        Self::RieulMieum,
        #[cfg(feature = "archaic-korean")]
        Self::RieulMieumKiyeok,
        #[cfg(feature = "archaic-korean")]
        Self::RieulMieumSios,
        #[cfg(feature = "archaic-korean")]
        Self::RieulMieumHieuh,
        Self::RieulPieup,
        #[cfg(feature = "archaic-korean")]
        Self::RieulPieupTikeut,
        #[cfg(feature = "archaic-korean")]
        Self::RieulPieupSios,
        #[cfg(feature = "archaic-korean")]
        Self::RieulPieupPhieuph,
        #[cfg(feature = "archaic-korean")]
        Self::RieulPieupHieuh,
        #[cfg(feature = "archaic-korean")]
        Self::RieulKapyeounPieup,
        Self::RieulSios,
        #[cfg(feature = "archaic-korean")]
        Self::RieulSsangSios,
        #[cfg(feature = "archaic-korean")]
        Self::RieulPanSios,
        #[cfg(feature = "archaic-korean")]
        Self::RieulYesIeung,
        #[cfg(feature = "archaic-korean")]
        Self::RieulKhieukh,
        Self::RieulThieuth,
        Self::RieulPhieuph,
        Self::RieulHieuh,
        #[cfg(feature = "archaic-korean")]
        Self::RieulYeorinHieuh,
        #[cfg(feature = "archaic-korean")]
        Self::RieulYeorinHieuhHieuh,
        #[cfg(feature = "archaic-korean")]
        Self::KapyeounRieul,
        Self::Mieum,
        #[cfg(feature = "archaic-korean")]
        Self::MieumKiyeok,
        #[cfg(feature = "archaic-korean")]
        Self::MieumNieun,
        #[cfg(feature = "archaic-korean")]
        Self::MieumSsangNieun,
        #[cfg(feature = "archaic-korean")]
        Self::MieumRieul,
        #[cfg(feature = "archaic-korean")]
        Self::SsangMieum,
        #[cfg(feature = "archaic-korean")]
        Self::MieumPieup,
        #[cfg(feature = "archaic-korean")]
        Self::MieumPieupSios,
        #[cfg(feature = "archaic-korean")]
        Self::MieumSios,
        #[cfg(feature = "archaic-korean")]
        Self::MieumSsangSios,
        #[cfg(feature = "archaic-korean")]
        Self::MieumPanSios,
        #[cfg(feature = "archaic-korean")]
        Self::MieumCieuc,
        #[cfg(feature = "archaic-korean")]
        Self::MieumChieuch,
        #[cfg(feature = "archaic-korean")]
        Self::MieumHieuh,
        #[cfg(feature = "archaic-korean")]
        Self::KapyeounMieum,
        Self::Pieup,
        #[cfg(feature = "archaic-korean")]
        Self::PieupTikeut,
        #[cfg(feature = "archaic-korean")]
        Self::PieupRieul,
        #[cfg(feature = "archaic-korean")]
        Self::PieupRieulPhieuph,
        #[cfg(feature = "archaic-korean")]
        Self::PieupMieum,
        #[cfg(feature = "archaic-korean")]
        Self::SsangPieup,
        Self::PieupSios,
        #[cfg(feature = "archaic-korean")]
        Self::PieupSiosTikeut,
        #[cfg(feature = "archaic-korean")]
        Self::PieupCieuc,
        #[cfg(feature = "archaic-korean")]
        Self::PieupChieuch,
        #[cfg(feature = "archaic-korean")]
        Self::PieupPhieuph,
        #[cfg(feature = "archaic-korean")]
        Self::PieupHieuh,
        #[cfg(feature = "archaic-korean")]
        Self::KapyeounPieup,
        Self::Sios,
        #[cfg(feature = "archaic-korean")]
        Self::SiosKiyeok,
        #[cfg(feature = "archaic-korean")]
        Self::SiosTikeut,
        #[cfg(feature = "archaic-korean")]
        Self::SiosRieul,
        #[cfg(feature = "archaic-korean")]
        Self::SiosMieum,
        #[cfg(feature = "archaic-korean")]
        Self::SiosPieup,
        #[cfg(feature = "archaic-korean")]
        Self::SiosKapyeounPieup,
        Self::SsangSios,
        #[cfg(feature = "archaic-korean")]
        Self::SsangSiosKiyeok,
        #[cfg(feature = "archaic-korean")]
        Self::SsangSiosTikeut,
        #[cfg(feature = "archaic-korean")]
        Self::SiosPanSios,
        #[cfg(feature = "archaic-korean")]
        Self::SiosCieuc,
        #[cfg(feature = "archaic-korean")]
        Self::SiosChieuch,
        #[cfg(feature = "archaic-korean")]
        Self::SiosThieuth,
        #[cfg(feature = "archaic-korean")]
        Self::SiosHieuh,
        #[cfg(feature = "archaic-korean")]
        Self::PanSios,
        #[cfg(feature = "archaic-korean")]
        Self::PanSiosPieup,
        #[cfg(feature = "archaic-korean")]
        Self::PanSiosKapyeounPieup,
        Self::Ieung,
        #[cfg(feature = "archaic-korean")]
        Self::YesIeung,
        #[cfg(feature = "archaic-korean")]
        Self::IeungKiyeok,
        #[cfg(feature = "archaic-korean")]
        Self::IeungSsangKiyeok,
        #[cfg(feature = "archaic-korean")]
        Self::YesIeungMieum,
        #[cfg(feature = "archaic-korean")]
        Self::YesIeungSios,
        #[cfg(feature = "archaic-korean")]
        Self::YesIeungPanSios,
        #[cfg(feature = "archaic-korean")]
        Self::SsangIeung,
        #[cfg(feature = "archaic-korean")]
        Self::IeungKhieukh,
        #[cfg(feature = "archaic-korean")]
        Self::YesIeungHieuh,
        Self::Cieuc,
        #[cfg(feature = "archaic-korean")]
        Self::CieucPieup,
        #[cfg(feature = "archaic-korean")]
        Self::CieucSsangPieup,
        #[cfg(feature = "archaic-korean")]
        Self::SsangCieuc,
        Self::Chieuch,
        Self::Khieukh,
        Self::Thieuth,
        Self::Phieuph,
        #[cfg(feature = "archaic-korean")]
        Self::PhieuphPieup,
        #[cfg(feature = "archaic-korean")]
        Self::PhieuphSios,
        #[cfg(feature = "archaic-korean")]
        Self::PhieuphThieuth,
        #[cfg(feature = "archaic-korean")]
        Self::KapyeounPhieuph,
        Self::Hieuh,
        #[cfg(feature = "archaic-korean")]
        Self::HieuhNieun,
        #[cfg(feature = "archaic-korean")]
        Self::HieuhRieul,
        #[cfg(feature = "archaic-korean")]
        Self::HieuhMieum,
        #[cfg(feature = "archaic-korean")]
        Self::HieuhPieup,
        #[cfg(feature = "archaic-korean")]
        Self::YeorinHieuh,
    ];
}
