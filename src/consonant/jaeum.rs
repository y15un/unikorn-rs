use crate::{Choseong, Error, Jongseong};
use num_enum::{IntoPrimitive, TryFromPrimitive};

/// Groups all the Korean consonants (자음, Jaeum).
///
/// Specifically, those residing in Hangul Compatibility Jamo range (U+3131 'ㄱ' -- U+314E 'ㅎ').
///
/// Most of the time, all you need to do is calling [`Jaeum::try_from`] with [`char`]
/// (that contains a valid Korean syllable) as its argument:
/// ```
/// use unikorn::Jaeum;
/// use std::convert::TryFrom;
///
/// let jaeum = Jaeum::try_from('ㄳ').unwrap();
/// assert_eq!(jaeum, Jaeum::KiyeokSios);
/// ```
///
/// Also, you can convert [`Choseong`] and [`Jongseong`] from and into `Jaeum`:
/// ```
/// use unikorn::{Choseong, Error, Jaeum, Jongseong};
/// use std::convert::TryFrom;
///
/// let choseong = Choseong::Rieul; // ㄹ
/// assert_eq!(Jaeum::from(choseong), Jaeum::Rieul);
///
/// let jongseong = Jongseong::PieupSios; // ㅄ
/// assert_eq!(Jaeum::from(jongseong), Jaeum::PieupSios);
///
/// let jaeum = Jaeum::SsangSios; // ㅆ
/// assert_eq!(Choseong::try_from(jaeum), Ok(Choseong::SsangSios));
/// assert_eq!(Jongseong::try_from(jaeum), Ok(Jongseong::SsangSios));
///
/// let jaeum = Jaeum::SsangTikeut; // ㄸ
/// assert_eq!(Choseong::try_from(jaeum), Ok(Choseong::SsangTikeut));
/// assert_eq!(
///     Jongseong::try_from(jaeum),
///     Err(Error::NotApplicableToJongseong(Jaeum::SsangTikeut))
/// );
///
/// let jaeum = Jaeum::NieunCieuc; // ㄵ
/// assert_eq!(
///     Choseong::try_from(jaeum),
///     Err(Error::NotApplicableToChoseong(Jaeum::NieunCieuc))
/// );
/// assert_eq!(Jongseong::try_from(jaeum), Ok(Jongseong::NieunCieuc));
/// ```
#[derive(Clone, Copy, Debug, Eq, IntoPrimitive, Ord, PartialEq, PartialOrd, TryFromPrimitive)]
#[repr(u8)]
pub enum Jaeum {
    /// Represents 'ㄱ'.
    Kiyeok,
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
    /// Represents 'ㄸ'.
    SsangTikeut,
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
    /// Represents 'ㅃ'.
    SsangPieup,
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
impl From<Choseong> for Jaeum {
    fn from(choseong: Choseong) -> Self {
        match choseong {
            Choseong::Kiyeok => Self::Kiyeok,
            Choseong::SsangKiyeok => Self::SsangKiyeok,
            Choseong::Nieun => Self::Nieun,
            Choseong::Tikeut => Self::Tikeut,
            Choseong::SsangTikeut => Self::SsangTikeut,
            Choseong::Rieul => Self::Rieul,
            Choseong::Mieum => Self::Mieum,
            Choseong::Pieup => Self::Pieup,
            Choseong::SsangPieup => Self::SsangPieup,
            Choseong::Sios => Self::Sios,
            Choseong::SsangSios => Self::SsangSios,
            Choseong::Ieung => Self::Ieung,
            Choseong::Cieuc => Self::Cieuc,
            Choseong::SsangCieuc => Self::SsangCieuc,
            Choseong::Chieuch => Self::Chieuch,
            Choseong::Khieukh => Self::Khieukh,
            Choseong::Thieuth => Self::Thieuth,
            Choseong::Phieuph => Self::Phieuph,
            Choseong::Hieuh => Self::Hieuh,
        }
    }
}
impl From<Jaeum> for char {
    fn from(jaeum: Jaeum) -> Self {
        Self::from_u32(0x3131 + jaeum as u32).unwrap()
    }
}
impl From<Jongseong> for Jaeum {
    fn from(jongseong: Jongseong) -> Self {
        match jongseong {
            Jongseong::Kiyeok => Self::Kiyeok,
            Jongseong::SsangKiyeok => Self::SsangKiyeok,
            Jongseong::KiyeokSios => Self::KiyeokSios,
            Jongseong::Nieun => Self::Nieun,
            Jongseong::NieunCieuc => Self::NieunCieuc,
            Jongseong::NieunHieuh => Self::NieunHieuh,
            Jongseong::Tikeut => Self::Tikeut,
            Jongseong::Rieul => Self::Rieul,
            Jongseong::RieulKiyeok => Self::RieulKiyeok,
            Jongseong::RieulMieum => Self::RieulMieum,
            Jongseong::RieulPieup => Self::RieulPieup,
            Jongseong::RieulSios => Self::RieulSios,
            Jongseong::RieulThieuth => Self::RieulThieuth,
            Jongseong::RieulPhieuph => Self::RieulPhieuph,
            Jongseong::RieulHieuh => Self::RieulHieuh,
            Jongseong::Mieum => Self::Mieum,
            Jongseong::Pieup => Self::Pieup,
            Jongseong::PieupSios => Self::PieupSios,
            Jongseong::Sios => Self::Sios,
            Jongseong::SsangSios => Self::SsangSios,
            Jongseong::Ieung => Self::Ieung,
            Jongseong::Cieuc => Self::Cieuc,
            Jongseong::Chieuch => Self::Chieuch,
            Jongseong::Khieukh => Self::Khieukh,
            Jongseong::Thieuth => Self::Thieuth,
            Jongseong::Phieuph => Self::Phieuph,
            Jongseong::Hieuh => Self::Hieuh,
        }
    }
}
impl TryFrom<char> for Jaeum {
    type Error = Error;

    fn try_from(character: char) -> Result<Self, Self::Error> {
        if !(0x3131..=0x314E).contains(&(character as u32)) {
            return Err(Error::NonJamo(character));
        }

        Ok(Self::try_from((character as u32 - 0x3131) as u8).unwrap())
    }
}
