use crate::{Error, Jaeum};
use num_enum::{IntoPrimitive, TryFromPrimitive};

/// Groups all the consonants applicable to the 'initial consonant' (초성, Choseong) position of a
/// Korean syllable.
///
/// These consonants do reside by themselves as an individual Unicode characters, but not in this
/// particular order; for that, see [`Jaeum`].
#[derive(Clone, Copy, Debug, Eq, IntoPrimitive, Ord, PartialEq, PartialOrd, TryFromPrimitive)]
#[repr(u8)]
pub enum Choseong {
    /// Represents 'ㄱ'.
    Kiyeok,
    /// Represents 'ㄲ'.
    SsangKiyeok,
    /// Represents 'ㄴ'.
    Nieun,
    /// Represents 'ㄷ'.
    Tikeut,
    /// Represents 'ㄸ'.
    SsangTikeut,
    /// Represents 'ㄹ'.
    Rieul,
    /// Represents 'ㅁ'.
    Mieum,
    /// Represents 'ㅂ'.
    Pieup,
    /// Represents 'ㅃ'.
    SsangPieup,
    /// Represents 'ㅅ'.
    Sios,
    /// Represents 'ㅆ'.
    SsangSios,
    /// Represents 'ㅇ'.
    Ieung,
    /// Represents 'ㅈ'.
    Cieuc,
    /// Represents 'ㅉ'.
    SsangCieuc,
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
impl From<Choseong> for char {
    fn from(choseong: Choseong) -> Self {
        Jaeum::from(choseong).into()
    }
}
impl TryFrom<char> for Choseong {
    type Error = Error;

    fn try_from(character: char) -> Result<Self, Self::Error> {
        Self::try_from(Jaeum::try_from(character)?)
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
