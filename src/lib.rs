//! This crate provides ways to manipulate modern Korean alphabets (현대한글, Hyeondae Hangeul).
//!
//! More specifically, you can:
//! * Decompose a Precomposed Korean (완성형, Wanseonghyeong) [`Syllable`] into individual
//!   'consonants and vowels' (자모, Jamo), and
//! * Do the reverse of above action, i.e., compose a set of individual consonants and vowels
//!   into a Precomposed Korean Syllable.
//!
//! ## Features
//! ### `archaic-korean`
//! The `archaic-korean` feature enables handling archaic Korean alphabets (옛한글, Yet Hangeul),
//! extending the behavior of this crate to include now-obsolete consosnants, consonant sequences,
//! vowels, and vowel sequences into the [`Choseong`],[`Jungseong`], [`Jongseong`], and [`Jaeum`]
//! enums, but in a limited way.
mod consonant;
mod error;
mod syllable;
mod vowel;

pub use crate::{
    consonant::{Choseong, Jaeum, Jongseong},
    error::Error,
    syllable::Syllable,
    vowel::{Jungseong, Moeum},
};

#[cfg(test)]
mod tests {
    use super::{Choseong, Error, Jaeum, Jongseong, Jungseong, Syllable};
    use std::convert::TryFrom;

    #[test]
    fn test_from_choseong_for_char() {
        assert_eq!(char::from(Choseong::Ieung), 'ㅇ');
        assert_eq!(char::from(Choseong::Cieuc), 'ㅈ');
    }

    #[test]
    fn test_tryfrom_char_for_choseong() {
        assert_eq!(
            Choseong::try_from('ㄵ'),
            Err(Error::NotApplicableToChoseong(Jaeum::NieunCieuc))
        );
        assert_eq!(
            Choseong::try_from('ㄺ'),
            Err(Error::NotApplicableToChoseong(Jaeum::RieulKiyeok))
        );

        assert_eq!(Choseong::try_from('ㅈ'), Ok(Choseong::Cieuc));
        assert_eq!(Choseong::try_from('ㅅ'), Ok(Choseong::Sios));
    }

    #[test]
    fn test_tryfrom_jaeum_for_choseong() {
        assert_eq!(
            Choseong::try_from(Jaeum::KiyeokSios),
            Err(Error::NotApplicableToChoseong(Jaeum::KiyeokSios))
        );
        assert_eq!(
            Choseong::try_from(Jaeum::RieulPhieuph),
            Err(Error::NotApplicableToChoseong(Jaeum::RieulPhieuph))
        );

        assert_eq!(
            Choseong::try_from(Jaeum::SsangPieup),
            Ok(Choseong::SsangPieup)
        );
        assert_eq!(
            Choseong::try_from(Jaeum::SsangTikeut),
            Ok(Choseong::SsangTikeut)
        );
    }

    #[test]
    fn test_from_choseong_for_jaeum() {
        assert_eq!(Jaeum::from(Choseong::Thieuth), Jaeum::Thieuth);
        assert_eq!(Jaeum::from(Choseong::Kiyeok), Jaeum::Kiyeok);
    }

    #[test]
    fn test_from_jaeum_for_char() {
        assert_eq!(char::from(Jaeum::KiyeokSios), 'ㄳ');
        assert_eq!(char::from(Jaeum::Hieuh), 'ㅎ');
    }

    #[test]
    fn test_from_jongseong_for_jaeum() {
        assert_eq!(Jaeum::from(Jongseong::PieupSios), Jaeum::PieupSios);
        assert_eq!(Jaeum::from(Jongseong::Hieuh), Jaeum::Hieuh);
    }

    #[test]
    fn test_tryfrom_char_for_jaeum() {
        assert_eq!(Jaeum::try_from('^'), Err(Error::NonJamo('^')));
        assert_eq!(Jaeum::try_from('v'), Err(Error::NonJamo('v')));
        assert_eq!(Jaeum::try_from('ㅗ'), Err(Error::NonJamo('ㅗ')));

        assert_eq!(Jaeum::try_from('ㄱ'), Ok(Jaeum::Kiyeok));
        assert_eq!(Jaeum::try_from('ㅄ'), Ok(Jaeum::PieupSios));
        assert_eq!(Jaeum::try_from('ㄸ'), Ok(Jaeum::SsangTikeut));
    }

    #[test]
    fn test_from_jongseong_for_char() {
        assert_eq!(char::from(Jongseong::KiyeokSios), 'ㄳ');
        assert_eq!(char::from(Jongseong::RieulHieuh), 'ㅀ');
        assert_eq!(char::from(Jongseong::Sios), 'ㅅ');
    }

    #[test]
    fn test_tryfrom_char_for_jongseong() {
        assert_eq!(Jongseong::try_from('ㅗ'), Err(Error::NonJamo('ㅗ')));
        assert_eq!(Jongseong::try_from('v'), Err(Error::NonJamo('v')));

        assert_eq!(Jongseong::try_from('ㅄ'), Ok(Jongseong::PieupSios));
        assert_eq!(Jongseong::try_from('ㅎ'), Ok(Jongseong::Hieuh));
    }

    #[test]
    fn test_tryfrom_jaeum_for_jongseong() {
        // These three are the only possible instances of error in this context.
        assert_eq!(
            Jongseong::try_from(Jaeum::SsangTikeut),
            Err(Error::NotApplicableToJongseong(Jaeum::SsangTikeut))
        );
        assert_eq!(
            Jongseong::try_from(Jaeum::SsangPieup),
            Err(Error::NotApplicableToJongseong(Jaeum::SsangPieup))
        );
        assert_eq!(
            Jongseong::try_from(Jaeum::SsangCieuc),
            Err(Error::NotApplicableToJongseong(Jaeum::SsangCieuc))
        );

        assert_eq!(
            Jongseong::try_from(Jaeum::PieupSios),
            Ok(Jongseong::PieupSios)
        );
        assert_eq!(Jongseong::try_from(Jaeum::Hieuh), Ok(Jongseong::Hieuh));
    }

    #[test]
    fn test_from_jungseong_for_char() {
        assert_eq!(char::from(Jungseong::Yu), 'ㅠ');
        assert_eq!(char::from(Jungseong::O), 'ㅗ');
    }

    #[test]
    fn test_tryfrom_char_for_jungseong() {
        assert_eq!(Jungseong::try_from('1'), Err(Error::NonJamo('1')));
        assert_eq!(Jungseong::try_from('ㅎ'), Err(Error::NonJamo('ㅎ')));

        assert_eq!(Jungseong::try_from('ㅠ'), Ok(Jungseong::Yu));
    }

    #[test]
    fn test_from_i_m_tuple_for_syllable() {
        assert_eq!(
            Syllable::from((Choseong::Sios, Jungseong::O)),
            Syllable {
                choseong: Choseong::Sios,
                jungseong: Jungseong::O,
                jongseong: None
            }
        );
    }

    #[test]
    fn test_from_i_m_f_tuple_for_syllable() {
        assert_eq!(
            Syllable::from((Choseong::Khieukh, Jungseong::O, Some(Jongseong::Rieul))),
            Syllable {
                choseong: Choseong::Khieukh,
                jungseong: Jungseong::O,
                jongseong: Some(Jongseong::Rieul),
            }
        );
        assert_eq!(
            Syllable::from((Choseong::Rieul, Jungseong::A, None)),
            Syllable {
                choseong: Choseong::Rieul,
                jungseong: Jungseong::A,
                jongseong: None,
            }
        )
    }

    #[test]
    fn test_from_m_f_tuple_for_syllable() {
        assert_eq!(
            Syllable::from((Jungseong::Yo, None)),
            Syllable {
                choseong: Choseong::Ieung,
                jungseong: Jungseong::Yo,
                jongseong: None
            }
        );
        assert_eq!(
            Syllable::from((Jungseong::Ya, Some(Jongseong::Kiyeok))),
            Syllable {
                choseong: Choseong::Ieung,
                jungseong: Jungseong::Ya,
                jongseong: Some(Jongseong::Kiyeok),
            }
        )
    }

    #[test]
    fn test_from_jungseong_for_syllable() {
        assert_eq!(
            Syllable::from(Jungseong::U),
            Syllable {
                choseong: Choseong::Ieung,
                jungseong: Jungseong::U,
                jongseong: None,
            }
        );
        assert_eq!(
            Syllable::from(Jungseong::Yu),
            Syllable {
                choseong: Choseong::Ieung,
                jungseong: Jungseong::Yu,
                jongseong: None,
            }
        );
    }

    #[test]
    fn test_from_syllable_for_i_m_f_tuple() {
        let tuple: (Choseong, Jungseong, Option<Jongseong>) = Syllable {
            choseong: Choseong::Tikeut,
            jungseong: Jungseong::A,
            jongseong: Some(Jongseong::Kiyeok),
        }
        .into();
        assert_eq!(
            tuple,
            (Choseong::Tikeut, Jungseong::A, Some(Jongseong::Kiyeok))
        );

        let (choseong, jungseong, jongseong) = Syllable {
            choseong: Choseong::Phieuph,
            jungseong: Jungseong::E,
            jongseong: None,
        }
        .into();
        assert_eq!(choseong, Choseong::Phieuph);
        assert_eq!(jungseong, Jungseong::E);
        assert_eq!(jongseong, None);
    }

    #[test]
    fn test_from_syllable_for_char() {
        assert_eq!(
            char::from(Syllable {
                choseong: Choseong::Kiyeok,
                jungseong: Jungseong::Ae,
                jongseong: Some(Jongseong::Ieung),
            }),
            '갱'
        );
        assert_eq!(
            char::from(Syllable {
                choseong: Choseong::Ieung,
                jungseong: Jungseong::Eo,
                jongseong: Some(Jongseong::Rieul),
            }),
            '얼'
        );

        assert_eq!(
            char::from(Syllable {
                choseong: Choseong::Cieuc,
                jungseong: Jungseong::Wi,
                jongseong: None,
            }),
            '쥐'
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
            Syllable::try_from('먐'),
            Ok(Syllable {
                choseong: Choseong::Mieum,
                jungseong: Jungseong::Ya,
                jongseong: Some(Jongseong::Mieum),
            })
        );
        assert_eq!(
            Syllable::try_from('미'),
            Ok(Syllable {
                choseong: Choseong::Mieum,
                jungseong: Jungseong::I,
                jongseong: None,
            })
        );
    }

    #[test]
    fn test_syllable_is_one_of_us() {
        // TODO: update this test once the `archaic-korean` feature is added to the crate.
        assert_eq!(Syllable::is_one_of_us('꯹'), false); // U+ABF9
        assert_eq!(Syllable::is_one_of_us('가'), true); // U+AC00
        assert_eq!(Syllable::is_one_of_us('문'), true); // U+BB38
        assert_eq!(Syllable::is_one_of_us('힣'), true); // U+D7A3
        assert_eq!(Syllable::is_one_of_us('ힰ'), false); // U+D7B0 is technically a Korean alphabet,
                                                        // but an *archaic* Korean alphabet rather
                                                        // than a modern one. Thus it is considered
                                                        // NOT a valid Korean alphabet in the
                                                        // context of this library.
    }
}
