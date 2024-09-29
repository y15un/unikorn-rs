//! A library for computationally manipulating modern Korean alphabets (현대한글, Hyeondae Hangeul).
//!
//! TODO: complete documentation
//!
//! # Features
//! ## `archaic-korean`
//! `archaic-korean` feature enables handling archaic Korean alphabets (옛한글, Yet Hangeul), extending the behavior of this crate to include now-obsolete consosnants, consonant sequences, vowels, and vowel sequences into the [`Choseong`](crate::consonant::Choseong), [`Jungseong`](crate::vowel::Jungseong), [`Jongseong`](crate::consonant::Jongseong), and [`Jaeum`](crate::consonant::Jaeum), but in a limited way.
pub mod consonant;
mod error;
mod syllable;
pub mod vowel;

#[doc(inline)]
pub use crate::{error::Error, syllable::Syllable};
