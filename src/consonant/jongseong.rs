use crate::{Error, Jaeum};
use num_enum::{IntoPrimitive, TryFromPrimitive};

/// Groups all the consonants (including clustered consonants) applicable to the 'final consonant'
/// (종성, Jongseong) position of a Korean syllable.
///
/// These consonants do reside by themselves as an individual Unicode characters, but not in this
/// particular order; for that, see [`Jaeum`].
#[derive(Clone, Copy, Debug, Eq, IntoPrimitive, Ord, PartialEq, PartialOrd, TryFromPrimitive)]
#[repr(u8)]
pub enum Jongseong {
    /// Represents 'ㄱ'.
    Kiyeok = 1,
    /// Represents 'ㄲ'.
    SsangKiyeok,
    /// Represents 'ㄳ'.
    KiyeokSios,
    /// Represents 'ㄴ'.
    Nieun,
    /// Represents 'ㄵ'.
    NieunCieuc,
    /// Represents 'ㄶ'.
    NieunHieuh,
    /// Represents 'ㄷ'.
    Tikeut,
    /// Represents 'ㄹ'.
    Rieul,
    /// Represents 'ㄺ'.
    RieulKiyeok,
    /// Represents 'ㄻ'.
    RieulMieum,
    /// Represents 'ㄼ'.
    RieulPieup,
    /// Represents 'ㄽ'.
    RieulSios,
    /// Represents 'ㄾ'.
    RieulThieuth,
    /// Represents 'ㄿ'.
    RieulPhieuph,
    /// Represents 'ㅀ'.
    RieulHieuh,
    /// Represents 'ㅁ'.
    Mieum,
    /// Represents 'ㅂ'.
    Pieup,
    /// Represents 'ㅄ'.
    PieupSios,
    /// Represents 'ㅅ'.
    Sios,
    /// Represents 'ㅆ'.
    SsangSios,
    /// Represents 'ㅇ'.
    Ieung,
    /// Represents 'ㅈ'.
    Cieuc,
    /// Represents 'ㅊ'.
    Chieuch,
    /// Represents 'ㅋ'.
    Khieukh,
    /// Represents 'ㅌ'.
    Thieuth,
    /// Represents 'ㅍ'.
    Phieuph,
    /// Represents 'ㅎ'.
    Hieuh,
}
impl From<Jongseong> for char {
    fn from(jongseong: Jongseong) -> Self {
        Jaeum::from(jongseong).into()
    }
}
impl TryFrom<char> for Jongseong {
    type Error = Error;

    fn try_from(character: char) -> Result<Self, Self::Error> {
        Self::try_from(Jaeum::try_from(character)?)
    }
}
impl TryFrom<Jaeum> for Jongseong {
    type Error = Error;

    fn try_from(jaeum: Jaeum) -> Result<Self, Self::Error> {
        match jaeum {
            Jaeum::Kiyeok => Ok(Self::Kiyeok),
            Jaeum::SsangKiyeok => Ok(Self::SsangKiyeok),
            Jaeum::KiyeokSios => Ok(Self::KiyeokSios),
            Jaeum::Nieun => Ok(Self::Nieun),
            Jaeum::NieunCieuc => Ok(Self::NieunCieuc),
            Jaeum::NieunHieuh => Ok(Self::NieunHieuh),
            Jaeum::Tikeut => Ok(Self::Tikeut),
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
            Jaeum::PieupSios => Ok(Self::PieupSios),
            Jaeum::Sios => Ok(Self::Sios),
            Jaeum::SsangSios => Ok(Self::SsangSios),
            Jaeum::Ieung => Ok(Self::Ieung),
            Jaeum::Cieuc => Ok(Self::Cieuc),
            Jaeum::Chieuch => Ok(Self::Chieuch),
            Jaeum::Khieukh => Ok(Self::Khieukh),
            Jaeum::Thieuth => Ok(Self::Thieuth),
            Jaeum::Phieuph => Ok(Self::Phieuph),
            Jaeum::Hieuh => Ok(Self::Hieuh),
            anything_else => Err(Error::NotApplicableToJongseong(anything_else)),
        }
    }
}
