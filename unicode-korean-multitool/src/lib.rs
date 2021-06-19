//! This crate provides ways to manipulate modern Korean alphabet (현대한글, Hyeondae Hangeul).
//!
//! More specifically, you can:
//! * Decompose a Precomposed Korean [`Syllable`] into individual
//!   consonants and vowels (자모, Jamo), and
//! * Do the reverse of above action, i.e., compose a set of individual consonants and vowels into
//!   a Precomposed Korean Syllable.
use num_enum::{IntoPrimitive, TryFromPrimitive};
use std::{
    convert::TryFrom,
    error::Error as StdError,
    fmt::{Display, Formatter, Result as FmtResult},
};

/// Groups all the consonants applicable to the initial consonant (초성, Choseong) position of a
/// Korean syllable.
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

/// Contains all the possible error conditions that can arise within this crate.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Error {
    /// Denotes that a [`char`] outside the Precomposed Korean Syllables range (U+AC00 '가' --
    /// U+D7A3 '힣') was tried converting into a [`Syllable`].
    NonKorean(char),
}
impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            Self::NonKorean(coi) => write!(f, "'{}' is not a Precomposed Korean Sylable", coi),
        }
    }
}
impl StdError for Error {}

/// Groups all the consonants (including clustered consonants) applicable to the final consonants
/// (종성, Jongseong) position of a Korean syllable.
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

/// Groups all the vowels applicable to the medial vowel (중성, Jungseong) position of a Korean
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

/// Represents a Korean syllable.
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
impl From<Syllable> for char {
    fn from(syllable: Syllable) -> char {
        // all unified korean syllables are within BMP, so in this context, it is safe to assume:
        //     Unicode Scalar Value == Unicode Code Point
        // and thus, `char::from_u32()` never fails
        char::from_u32(
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

        // all unified korean syllables are within BMP, so in this context, it is safe to assume:
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
        // all unified korean syllables are within BMP, so in this context, it is safe to assume:
        //     Unicode Scalar Value == Unicode Code Point
        let character = character as u32;

        (0xAC00..=0xD7A3).contains(&character)
    }
}

#[cfg(test)]
mod tests {
    use super::{Choseong, Error, Jongseong, Jungseong, Syllable};
    use std::convert::TryFrom;

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
