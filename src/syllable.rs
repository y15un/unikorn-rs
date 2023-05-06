use crate::{Choseong, Error, Jongseong, Jungseong};

/// Represents a Korean syllable.
///
/// Specifically, those residing in Precomposed Hangul Syllables range (U+AC00 '가' -- U+D7A3 '힣').
///
/// Most of the time, all you need to do is calling [`Syllable::try_from`] with [`char`]
/// (that contains a valid Korean syllable) as its argument:
/// ```
/// use unikorn::{Choseong, Jungseong, Jongseong, Syllable};
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
impl From<(Jungseong, Option<Jongseong>)> for Syllable {
    fn from((jungseong, jongseong): (Jungseong, Option<Jongseong>)) -> Self {
        Self {
            choseong: Choseong::Ieung,
            jungseong,
            jongseong,
        }
    }
}
impl From<Jungseong> for Syllable {
    fn from(jungseong: Jungseong) -> Self {
        Self {
            choseong: Choseong::Ieung,
            jungseong,
            jongseong: None,
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
        // and thus, `char::from_u32()` never fails.
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
