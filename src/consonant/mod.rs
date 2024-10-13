//! A collection of types representing consonants (자음, Jaeum) of modern Korean alphabets.
//!
//! # Backgrounds
//! A typical Korean syllable is composed of the three components: initial consonant (초성, Choseong), median vowel (중성, Jungseong), and final consonant (종성, Jongseong).
//! For example, a Korean syllable `멍` (U+BA4D, Hangul Syllable Meong) has the following in it:
//! * Initial consonant: `ᄆ` (U+1106, Hangul Choseong Mieum)
//! * Median vowel: `ᅥ` (U+1165, Hangul Jungseong Eo)
//! * Final consonant: `ᆼ` (U+11BC, Hangul Jongseong Ieung)
//!
//! While intial consonant and median vowel are mandatory, final consonant is optional if that particular syllable does not have any *closing* phonetic sound.
//! For example, a Korean syllable `개` (U+AC1C, Hangul Syllable Gae) does not have final consonant, thus has the following only:
//! * Initial consonant: `ᄀ` (U+1100, Hangul Choseong Kiyeok)
//! * Median vowel: `ᅢ` (U+1162, Hangul Jungseong Ae)
//! * (Final consonant: None)
//!
//! Note that the Unicode character name and the actual (i.e., practical) phonetic sound might have some discrepancies sometimes.
//! Let's visit our previous example of `개` again: considering that the initial consonant Kiyeok, it would be considered *natural/intuitive* to be romanized as Kae, but in reality the National Institute of Korean Language defined its romanized version to be Gae. So it is advised to be aware of such possibilities.
//!
//! Also note that in Unicode there are two distict representations of consonants: consonants that are shaped and intended for initial and final consonants, and consonants that are shaped for *display* purpose. These two have their own dedicated code points, so it could cause some confusion if care is not taken. This library supports both types, by having former representation as either [`Choseong`] or [`Jongseong`] types, and the latter representation as [`Jaeum`] types; in addition to that, interoperability between these two representations are also supported.
//!
//! # Consonant Sequences
//! In modern Korean, two consonants can be combined together to form a consonant sequence (겹자음, Gyeobjaeum). Examples of such would be:
//! * `ᄁ` (U+1101, Hangul Choseong Ssangkiyeok) by combining two `ᄀ`s together.
//! * `ᆪ` (U+11AA, Hangul Jongseong Kiyeok-Sios) by combining `ᆨ` (U+11A8, Hangul Jongseong Kiyeok) and `ᆺ` (U+11BA, Hangul Jongseong Sios) together.
//!
//! Of course, there are only handful of these consonant sequences that are explicitly defined and are in use in modern Korean. However, the *original design principle* (훈민정음, Hunminjeongeum; design finalized in 1443 CE, publicly announced in 1446 CE) of the Korean alphabets allowed any arbitrary consonants to form a consonant sequence.
//!
//! This library supports such mechanism with `archaic-korean` feature, but with known limitation. While *the* principle allows any arbitrary combinations, due to the policy in adding new characters to Unicode, only consonant sequences explicitly observed of their existence are available in Unicode and are supported.

mod choseong;
mod jaeum;
mod jongseong;

#[doc(inline)]
pub use crate::consonant::{choseong::Choseong, jaeum::Jaeum, jongseong::Jongseong};
