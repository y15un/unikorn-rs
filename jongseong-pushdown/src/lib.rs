use std::convert::TryFrom;
use unicode_korean_multitool::{Choseong, Jongseong, Jungseong, Syllable};

// how to interpret (jongseong_a, jongseong_b, choseong_c, extended)
// => when the current syllable has `jongseong_a` and the next syllable has `Choseong::Ieung`,
//    replace the current syllable's jongseong with `jongseong_b`
//    and replace the next syllable's choseong with `choseong_c`.
// => when `extended` is true, it's part of the extended ruleset, which violates
//    pronunciation equivalence.
//
// additional extended ruleset:
//     if the current syllable has either `Jongseong::Tikeut` or `Jongseong::Thikeuth`,
//     and the next syllable has any of the following vowels as jungseong:
//     * Jungseong::Ya,
//     * Jungseong::Yae,
//     * Jungseong::Yeo,
//     * Jungseong::Ye,
//     * Jungseong::Yo,
//     * Jungseong::Yu,
//     * Jungseong::I,
//     then only apply jongseong pushdown if and only if extended rulset is active.
const RULESET: [(Jongseong, Jongseong, Choseong, bool); 26] = [
    (Jongseong::Kiyeok, Jongseong::Empty, Choseong::Kiyeok, false),
    (
        Jongseong::SsangKiyeok,
        Jongseong::Empty,
        Choseong::SsangKiyeok,
        false,
    ),
    (
        Jongseong::KiyeokSios,
        Jongseong::Kiyeok,
        Choseong::Sios,
        false,
    ),
    (Jongseong::Nieun, Jongseong::Empty, Choseong::Nieun, false),
    (
        Jongseong::NieunCieuc,
        Jongseong::Nieun,
        Choseong::Cieuc,
        false,
    ),
    (
        Jongseong::NieunHieuh,
        Jongseong::Nieun,
        Choseong::Hieuh,
        true,
    ),
    (Jongseong::Tikeut, Jongseong::Empty, Choseong::Tikeut, false),
    (Jongseong::Rieul, Jongseong::Empty, Choseong::Rieul, false),
    (
        Jongseong::RieulKiyeok,
        Jongseong::Rieul,
        Choseong::Kiyeok,
        false,
    ),
    (
        Jongseong::RieulMieum,
        Jongseong::Rieul,
        Choseong::Mieum,
        false,
    ),
    (
        Jongseong::RieulPieup,
        Jongseong::Rieul,
        Choseong::Pieup,
        false,
    ),
    (
        Jongseong::RieulSios,
        Jongseong::Rieul,
        Choseong::Sios,
        false,
    ),
    (
        Jongseong::RieulThieuth,
        Jongseong::Rieul,
        Choseong::Thieuth,
        false,
    ),
    (
        Jongseong::RieulPhieuph,
        Jongseong::Rieul,
        Choseong::Phieuph,
        false,
    ),
    (
        Jongseong::RieulHieuh,
        Jongseong::Rieul,
        Choseong::Hieuh,
        true,
    ),
    (Jongseong::Mieum, Jongseong::Empty, Choseong::Mieum, false),
    (Jongseong::Pieup, Jongseong::Empty, Choseong::Pieup, false),
    (
        Jongseong::PieupSios,
        Jongseong::Pieup,
        Choseong::Sios,
        false,
    ),
    (Jongseong::Sios, Jongseong::Empty, Choseong::Sios, false),
    (
        Jongseong::SsangSios,
        Jongseong::Empty,
        Choseong::SsangSios,
        false,
    ),
    (Jongseong::Cieuc, Jongseong::Empty, Choseong::Cieuc, false),
    (
        Jongseong::Chieuch,
        Jongseong::Empty,
        Choseong::Chieuch,
        false,
    ),
    (
        Jongseong::Khieukh,
        Jongseong::Empty,
        Choseong::Khieukh,
        false,
    ),
    (
        Jongseong::Thieuth,
        Jongseong::Empty,
        Choseong::Thieuth,
        false,
    ),
    (
        Jongseong::Phieuph,
        Jongseong::Empty,
        Choseong::Phieuph,
        false,
    ),
    (Jongseong::Hieuh, Jongseong::Empty, Choseong::Hieuh, true),
];

pub fn pushdown_jongseong(source: &str) -> String {
    pushdown_jongseong_config(source, false)
}

pub fn pushdown_jongseong_config(source: &str, extended_flag: bool) -> String {
    let mut destination = String::with_capacity(source.len());

    let mut buffer: [u8; 4] = [0, 0, 0, 0];
    let mut characters = source.chars().peekable();
    let mut new_choseong = None;

    while let Some(current) = characters.next() {
        if !Syllable::is_one_of_us(current) {
            destination.push_str(current.encode_utf8(&mut buffer));

            continue;
        }
        let mut current_syllable = Syllable::try_from(current).unwrap();
        if new_choseong.is_some() {
            current_syllable.choseong = new_choseong.take().unwrap();
        }

        if let Some(&next) = characters.peek() {
            if !Syllable::is_one_of_us(next) {
                destination.push_str(char::from(current_syllable).encode_utf8(&mut buffer));

                continue;
            }
            let next_syllable = Syllable::try_from(next).unwrap();

            // additional extended ruleset check
            if !([Jongseong::Tikeut, Jongseong::Thieuth].contains(&current_syllable.jongseong)
                && [
                    Jungseong::Ya,
                    Jungseong::Yae,
                    Jungseong::Yeo,
                    Jungseong::Ye,
                    Jungseong::Yo,
                    Jungseong::Yu,
                    Jungseong::I,
                ]
                .contains(&next_syllable.jungseong))
                || extended_flag
            {
                for &(
                    current_jongseong_match,
                    current_jongseong_to_be,
                    next_choseong_to_be,
                    is_extended,
                ) in RULESET.iter()
                {
                    if current_jongseong_match == current_syllable.jongseong
                        && Choseong::Ieung == next_syllable.choseong
                        && (is_extended <= extended_flag)
                    {
                        current_syllable.jongseong = current_jongseong_to_be;
                        new_choseong = Some(next_choseong_to_be);

                        break;
                    }
                }
            }
        }

        destination.push_str(char::from(current_syllable).encode_utf8(&mut buffer));
    }

    destination
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_pushdown_jongseong() {
        assert_eq!(
            super::pushdown_jongseong("종성 내려 쓰기"),
            "종성 내려 쓰기".to_owned()
        );
        assert_eq!(
            super::pushdown_jongseong("입울 밖은 위험해!"),
            "이불 바끈 위험해!".to_owned()
        );
        assert_eq!(
            super::pushdown_jongseong("입울 밖은 윟엄해!"),
            "이불 바끈 윟엄해!".to_owned()
        );
        assert_eq!(
            super::pushdown_jongseong("벝엋잌인 쵝오야!"),
            "버터치킨 최고야!".to_owned()
        );
        assert_eq!(
            super::pushdown_jongseong("이 얾안아 뭇임웃이한 생각인이"),
            "이 얼마나 무시무시한 생가기니".to_owned()
        );
        assert_eq!(
            super::pushdown_jongseong("이 얾안아 뭇임웃잏안 생각인이"),
            "이 얼마나 무시무싷안 생가기니".to_owned()
        );
        assert_eq!(
            super::pushdown_jongseong("해돋이 돋아 다같이 같아"),
            "해돋이 도다 다같이 가타".to_owned()
        );

        assert_eq!(
            super::pushdown_jongseong_config("입울 밖은 윟엄해!", true),
            "이불 바끈 위험해!".to_owned()
        );
        assert_eq!(
            super::pushdown_jongseong_config("이 얾안아 뭇임웃잏안 생각인이", true),
            "이 얼마나 무시무시한 생가기니".to_owned()
        );
        assert_eq!(
            super::pushdown_jongseong_config("해돋이 돋아 다같이 같아", true),
            "해도디 도다 다가티 가타".to_owned()
        );
    }
}
