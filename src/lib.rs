//! This crate provides ways to manipulate modern Korean alphabets (현대한글, Hyeondae Hangeul).
//!
//! More specifically, you can:
//! * Decompose a Precomposed Korean [`Syllable`] into individual 'consonants and vowels' (자모,
//!   Jamo), and
//! * Do the reverse of above action, i.e., compose a set of individual consonants and vowels
//!   into a Precomposed Korean Syllable.
use num_enum::{IntoPrimitive, TryFromPrimitive};
use std::{
    convert::TryFrom,
    error::Error as StdError,
    fmt::{Display, Formatter, Result as FmtResult},
};

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

/// Contains all the possible error conditions that can arise within this crate.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Error {
    /// Denotes that a [`char`] outside the Hangul Compatibility Jamo range (U+3131 'ㄱ' -- U+3163
    /// 'ㅣ') was tried converting into either a [`Jaeum`] or [`Moeum`].
    NonJamo(char),
    /// Denotes that a [`char`] outside the Precomposed Korean Syllables range (U+AC00 '가' --
    /// U+D7A3 '힣') was tried converting into a [`Syllable`].
    NonKorean(char),
    /// Denotes that a consonant (자음, [`Jaeum`]) cannot be placed in the initial consonant (초성,
    /// [`Choseong`]) position.
    NotApplicableToChoseong(Jaeum),
    /// Denotes that a consonant (자음, [`Jaeum`]) cannot be placed in the final consonant (종성,
    /// [`Jongseong`]) position.
    NotApplicableToJongseong(Jaeum),
}
impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            Self::NonJamo(coi) => write!(f, "'{}' is not a Hangul Compatibility Jamo", coi),
            Self::NonKorean(coi) => write!(f, "'{}' is not a Precomposed Korean Sylable", coi),
            Self::NotApplicableToChoseong(jaeum) => {
                write!(f, "{:?} cannot be used as an initial consonant", jaeum)
            }
            Self::NotApplicableToJongseong(jaeum) => {
                write!(f, "{:?} cannot be used as a final consonant", jaeum)
            }
        }
    }
}
impl StdError for Error {}

/// Groups all the Korean consonants (자음, Jaeum).
///
/// Specifically, those residing in Hangul Compatibility Jamo range (U+3131 'ㄱ' -- U+314E 'ㅎ').
///
/// Most of the time, all you need to do is calling [`Jaeum::try_from`] with [`char`]
/// (that contains a valid Korean syllable) as its argument:
/// ```
/// use crate::Jaeum;
/// use std::convert::TryFrom;
///
/// let jaeum = Jaeum::try_from('ㄳ').unwrap();
/// assert_eq!(jaeum, Jaeum::KiyeokSios);
/// ```
///
/// Also, you can convert [`Choseong`] and [`Jongseong`] from and into `Jaeum`:
/// ```
/// use crate::{Choseong, Error, Jaeum, Jongseong};
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

/// Groups all the vowels applicable to the 'medial vowel' (중성, Jungseong) position of a Korean
/// syllable.
#[derive(Clone, Copy, Debug, Eq, IntoPrimitive, Ord, PartialEq, PartialOrd, TryFromPrimitive)]
#[repr(u8)]
pub enum Jungseong {
    /// Represents 'ㅏ'.
    A,
    /// Represents 'ㅐ'.
    Ae,
    /// Represents 'ㅑ'.
    Ya,
    /// Represents 'ㅒ'.
    Yae,
    /// Represents 'ㅓ'.
    Eo,
    /// Represents 'ㅔ'.
    E,
    /// Represents 'ㅕ'.
    Yeo,
    /// Represents 'ㅖ'.
    Ye,
    /// Represents 'ㅗ'.
    O,
    /// Represents 'ㅘ'.
    Wa,
    /// Represents 'ㅙ'.
    Wae,
    /// Represents 'ㅚ'.
    Oe,
    /// Represents 'ㅛ'.
    Yo,
    /// Represents 'ㅜ'.
    U,
    /// Represents 'ㅝ'.
    Weo,
    /// Represents 'ㅞ'.
    We,
    /// Represents 'ㅟ'.
    Wi,
    /// Represents 'ㅠ'.
    Yu,
    /// Represents 'ㅡ'.
    Eu,
    /// Represents 'ㅢ'.
    Yi,
    /// Represents 'ㅣ'.
    I,
}
impl From<Jungseong> for char {
    fn from(jungseong: Jungseong) -> Self {
        Self::from_u32(0x314F + jungseong as u32).unwrap()
    }
}
impl TryFrom<char> for Jungseong {
    type Error = Error;

    fn try_from(character: char) -> Result<Self, Self::Error> {
        if !(0x314F..=0x3163).contains(&(character as u32)) {
            return Err(Error::NonJamo(character));
        }

        Ok(Self::try_from((character as u32 - 0x314F) as u8).unwrap())
    }
}

/// Groups all the Korean vowels (모음, Moeum).
///
/// Specifically, those residing in 'Hangul Compatibility Jamo' range (U+314F 'ㅏ' -- U+3163 'ㅣ').
///
/// Since [`Jungseong`] already contains the entire vowel set, this is mere a type alias to it.
pub type Moeum = Jungseong;

/// Represents a Korean syllable.
///
/// Specifically, those residing in Precomposed Hangul Syllables range (U+AC00 '가' -- U+D7A3 '힣').
///
/// Most of the time, all you need to do is calling [`Syllable::try_from`] with [`char`]
/// (that contains a valid Korean syllable) as its argument:
/// ```
/// use crate::{Choseong, Jungseong, Jongseong};
/// use std::convert::TryFrom;
///
/// let syllable = Syllable::try_from('잌').unwrap();
/// assert_eq!(syllable.choseong, Choseong::Ieung);
/// assert_eq!(syllable.jungseong, Jungseong::I);
/// assert_eq!(syllable.jongseong, Some(Jongseong::Khieukh));
///
/// let syllable = Syllable::try_from('뭐').unwrap();
/// assert_eq!(syllable.choseong, Choseong::Mieum);
/// assert_eq!(syllable.jungseong, Jungseong::Weo);
/// assert_eq!(syllable.jongseong, None);
/// ```
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Syllable {
    pub choseong: Choseong,
    pub jungseong: Jungseong,
    pub jongseong: Option<Jongseong>,
}
impl From<(Choseong, Jungseong)> for Syllable {
    fn from((choseong, jungseong): (Choseong, Jungseong)) -> Self {
        Self {
            choseong,
            jungseong,
            jongseong: None,
        }
    }
}
impl From<(Choseong, Jungseong, Option<Jongseong>)> for Syllable {
    fn from((choseong, jungseong, jongseong): (Choseong, Jungseong, Option<Jongseong>)) -> Self {
        Self {
            choseong,
            jungseong,
            jongseong,
        }
    }
}
impl From<Syllable> for (Choseong, Jungseong, Option<Jongseong>) {
    fn from(syllable: Syllable) -> Self {
        (syllable.choseong, syllable.jungseong, syllable.jongseong)
    }
}
impl From<Syllable> for char {
    fn from(syllable: Syllable) -> Self {
        // all precomposed korean syllables are within BMP, so in this context, it is safe to
        // assume:
        //     Unicode Scalar Value == Unicode Code Point
        // and thus, `char::from_u32()` never fails
        Self::from_u32(
            0xAC00
                + (syllable.choseong as u32 * 21 * 28)
                + (syllable.jungseong as u32 * 28)
                + if let Some(jongseong) = syllable.jongseong {
                    jongseong as u32
                } else {
                    0
                },
        )
        .unwrap()
    }
}
impl TryFrom<char> for Syllable {
    type Error = Error;

    fn try_from(character: char) -> Result<Self, Self::Error> {
        if !Self::is_one_of_us(character) {
            return Err(Error::NonKorean(character));
        }

        // all precomposed korean syllables are within BMP, so in this context, it is safe to
        // assume:
        //     Unicode Scalar Value == Unicode Code Point
        let mut unified_syllable = character as u32 - 0xAC00;

        let jongseong = unified_syllable % 28;
        unified_syllable -= jongseong;
        let jungseong = (unified_syllable / 28) % 21;
        unified_syllable -= jungseong * 28;
        let choseong = unified_syllable / (21 * 28);

        Ok(Self {
            choseong: Choseong::try_from(choseong as u8).unwrap(),
            jungseong: Jungseong::try_from(jungseong as u8).unwrap(),
            jongseong: Jongseong::try_from(jongseong as u8).ok(),
        })
    }
}
impl Syllable {
    /// Determines if a given [`char`] is one of the 11,172 valid modern Korean syllables.
    pub fn is_one_of_us(character: char) -> bool {
        // all precomposed korean syllables are within BMP, so in this context, it is safe to
        // assume:
        //     Unicode Scalar Value == Unicode Code Point
        let character = character as u32;

        (0xAC00..=0xD7A3).contains(&character)
    }
}

#[cfg(test)]
mod tests {
    use super::{Choseong, Error, Jaeum, Jongseong, Jungseong, Moeum, Syllable};
    use std::convert::TryFrom;

    #[test]
    fn test_from_jamo_for_char() {
        let choseong = Choseong::Pieup;
        assert_eq!(char::from(choseong), 'ㅂ');
        let jaeum = Jaeum::from(choseong);
        assert_eq!(jaeum, Jaeum::Pieup);
        assert_eq!(char::from(jaeum), 'ㅂ');

        let jungseong = Jungseong::We;
        assert_eq!(char::from(jungseong), 'ㅞ');
        assert_eq!(jungseong, Moeum::We);

        let jongseong = Jongseong::RieulKiyeok;
        assert_eq!(char::from(jongseong), 'ㄺ');
        let jaeum = Jaeum::from(jongseong);
        assert_eq!(jaeum, Jaeum::RieulKiyeok);
        assert_eq!(char::from(jaeum), 'ㄺ');
    }

    #[test]
    fn test_tryfrom_char_for_jamo() {
        assert_eq!(Choseong::try_from('@'), Err(Error::NonJamo('@')));
        assert_eq!(Jungseong::try_from('E'), Err(Error::NonJamo('E')));
        assert_eq!(Jongseong::try_from('𝄞'), Err(Error::NonJamo('𝄞')));

        assert_eq!(Choseong::try_from('ㄸ'), Ok(Choseong::SsangTikeut));
        assert_eq!(Jaeum::try_from('ㄸ'), Ok(Jaeum::SsangTikeut));
        assert_eq!(Jungseong::try_from('ㅖ'), Ok(Jungseong::Ye));
        assert_eq!(Moeum::try_from('ㅖ'), Ok(Moeum::Ye));
        assert_eq!(Jongseong::try_from('ㅎ'), Ok(Jongseong::Hieuh));
        assert_eq!(Jaeum::try_from('ㅎ'), Ok(Jaeum::Hieuh));
    }

    #[test]
    fn test_from_syllable_for_char() {
        assert_eq!(
            char::from(Syllable {
                choseong: Choseong::Ieung,
                jungseong: Jungseong::I,
                jongseong: Some(Jongseong::Rieul),
            }),
            '일'
        );
        assert_eq!(
            char::from(Syllable {
                choseong: Choseong::Sios,
                jungseong: Jungseong::Eo,
                jongseong: Some(Jongseong::Nieun),
            }),
            '선'
        );

        assert_eq!(
            char::from(Syllable {
                choseong: Choseong::Kiyeok,
                jungseong: Jungseong::Ae,
                jongseong: None,
            }),
            '개'
        );
    }

    #[test]
    fn test_tryfrom_char_for_syllable() {
        assert_eq!(Syllable::try_from('@'), Err(Error::NonKorean('@')));
        assert_eq!(Syllable::try_from('E'), Err(Error::NonKorean('E')));
        assert_eq!(Syllable::try_from('𝄞'), Err(Error::NonKorean('𝄞')));

        assert_eq!(
            Syllable::try_from('고'),
            Ok(Syllable {
                choseong: Choseong::Kiyeok,
                jungseong: Jungseong::O,
                jongseong: None,
            })
        );
        assert_eq!(
            Syllable::try_from('양'),
            Ok(Syllable {
                choseong: Choseong::Ieung,
                jungseong: Jungseong::Ya,
                jongseong: Some(Jongseong::Ieung),
            })
        );
        assert_eq!(
            Syllable::try_from('이'),
            Ok(Syllable {
                choseong: Choseong::Ieung,
                jungseong: Jungseong::I,
                jongseong: None,
            })
        );
    }
}
