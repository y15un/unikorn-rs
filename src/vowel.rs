use crate::Error;
use num_enum::{IntoPrimitive, TryFromPrimitive};

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
