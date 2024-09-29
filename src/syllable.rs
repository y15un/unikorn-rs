use crate::{
    consonant::{Choseong, Jongseong},
    vowel::Jungseong,
};

/// A modern Korean syllable.
pub struct Syllable {
    pub initial_consonant: Choseong,
    pub median_vowel: Jungseong,
    pub final_consonant: Option<Jongseong>,
}
