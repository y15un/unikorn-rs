//! A collection of types representing vowel (모음, Moeum) of modern Korean alphabets.
//!
//! # Backgrounds
//! Please refer to [`consonant`] module for details.
//!
//! There is only one type defined in this module, [`Jungseong`]. [`Moeum`] is mere an alias to [`Jungseong`].
//!
//! # Vowel Sequences
//! Please refer to [`consonant`] module for details.
//!
//! [`consonant`]: crate::consonant
mod jungseong;
// mod moeum;

#[doc(inline)]
pub use crate::vowel::jungseong::Jungseong;
