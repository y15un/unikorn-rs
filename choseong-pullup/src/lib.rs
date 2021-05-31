use std::convert::TryFrom;
use unicode_korean_multitool::{Choseong, Jongseong, Syllable};

// how to interpret (jongseong_a, choseong_b, jongseong_c, extended)
// => when the current syllable has `jongseong_a` and the next syllable has 'choseong_b',
//    replace the current syllable's jongseong with `jongseong_c`
//    and replace the next syllable's choseong with `Choseong:Ieung`.
// => when `extended` is true, it's part of the extended ruleset, which violates
//    pronunciation equivalence.
const RULESET: [(Jongseong, Choseong, Jongseong, bool); 28] = [
    (Jongseong::Empty, Choseong::Kiyeok, Jongseong::Kiyeok, false),
    (
        Jongseong::Empty,
        Choseong::SsangKiyeok,
        Jongseong::SsangKiyeok,
        false,
    ),
    (Jongseong::Empty, Choseong::Nieun, Jongseong::Nieun, false),
    (Jongseong::Empty, Choseong::Tikeut, Jongseong::Tikeut, false),
    (Jongseong::Empty, Choseong::Rieul, Jongseong::Rieul, false),
    (Jongseong::Empty, Choseong::Mieum, Jongseong::Mieum, false),
    (Jongseong::Empty, Choseong::Pieup, Jongseong::Pieup, false),
    (Jongseong::Empty, Choseong::Sios, Jongseong::Sios, false),
    (
        Jongseong::Empty,
        Choseong::SsangSios,
        Jongseong::SsangSios,
        false,
    ),
    (Jongseong::Empty, Choseong::Cieuc, Jongseong::Cieuc, false),
    (
        Jongseong::Empty,
        Choseong::Chieuch,
        Jongseong::Chieuch,
        false,
    ),
    (
        Jongseong::Empty,
        Choseong::Khieukh,
        Jongseong::Khieukh,
        false,
    ),
    (
        Jongseong::Empty,
        Choseong::Thieuth,
        Jongseong::Thieuth,
        false,
    ),
    (
        Jongseong::Empty,
        Choseong::Phieuph,
        Jongseong::Phieuph,
        false,
    ),
    (Jongseong::Empty, Choseong::Hieuh, Jongseong::Hieuh, true),
    (
        Jongseong::Kiyeok,
        Choseong::Kiyeok,
        Jongseong::SsangKiyeok,
        true,
    ),
    (
        Jongseong::Kiyeok,
        Choseong::Sios,
        Jongseong::KiyeokSios,
        false,
    ),
    (
        Jongseong::Nieun,
        Choseong::Cieuc,
        Jongseong::NieunCieuc,
        false,
    ),
    (
        Jongseong::Nieun,
        Choseong::Hieuh,
        Jongseong::NieunHieuh,
        true,
    ),
    (
        Jongseong::Rieul,
        Choseong::Kiyeok,
        Jongseong::RieulKiyeok,
        false,
    ),
    (
        Jongseong::Rieul,
        Choseong::Mieum,
        Jongseong::RieulMieum,
        false,
    ),
    (
        Jongseong::Rieul,
        Choseong::Pieup,
        Jongseong::RieulPieup,
        false,
    ),
    (
        Jongseong::Rieul,
        Choseong::Sios,
        Jongseong::RieulSios,
        false,
    ),
    (
        Jongseong::Rieul,
        Choseong::Thieuth,
        Jongseong::RieulThieuth,
        false,
    ),
    (
        Jongseong::Rieul,
        Choseong::Phieuph,
        Jongseong::RieulPhieuph,
        false,
    ),
    (
        Jongseong::Rieul,
        Choseong::Hieuh,
        Jongseong::RieulHieuh,
        true,
    ),
    (
        Jongseong::Pieup,
        Choseong::Sios,
        Jongseong::PieupSios,
        false,
    ),
    (Jongseong::Sios, Choseong::Sios, Jongseong::SsangSios, true),
];

pub fn pullup_choseong(source: &str) -> String {
    pullup_choseong_config(source, false)
}

pub fn pullup_choseong_config(source: &str, extended_rulset: bool) -> String {
    let mut destination = String::with_capacity(source.len());

    let mut buffer: [u8; 4] = [0, 0, 0, 0];
    let mut characters = source.chars().peekable();
    let mut choseong_pulled = false;

    while let Some(current) = characters.next() {
        if !Syllable::is_one_of_us(current) {
            destination.push_str(current.encode_utf8(&mut buffer));

            continue;
        }
        let mut current_syllable = Syllable::try_from(current).unwrap();
        if choseong_pulled {
            current_syllable.choseong = Choseong::Ieung;
            choseong_pulled = false;
        }

        if let Some(&next) = characters.peek() {
            if !Syllable::is_one_of_us(next) {
                destination.push_str(char::from(current_syllable).encode_utf8(&mut buffer));

                continue;
            }
            let next_syllable = Syllable::try_from(next).unwrap();

            for &(
                current_jongseong_match,
                next_choseong_match,
                current_jongseong_to_be,
                extended,
            ) in RULESET.iter()
            {
                if current_jongseong_match == current_syllable.jongseong
                    && next_choseong_match == next_syllable.choseong
                    && (extended <= extended_rulset)
                {
                    current_syllable.jongseong = current_jongseong_to_be;
                    choseong_pulled = true;

                    break;
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
    fn test_pullup_choseong() {
        assert_eq!(
            super::pullup_choseong("초성 올려 쓰기"),
            "촛엉 올려 쓱이".to_owned()
        );
        assert_eq!(
            super::pullup_choseong("이불 밖은 위험해!"),
            "입울 밖은 위험해!".to_owned()
        );
        assert_eq!(
            super::pullup_choseong("버터치킨 최고야!"),
            "벝엋잌인 쵝오야!".to_owned()
        );
        assert_eq!(
            super::pullup_choseong("이 얼마나 무시무시한 생각이니"),
            "이 얾안아 뭇임웃이한 생각인이".to_owned()
        );

        assert_eq!(
            super::pullup_choseong_config("이불 밖은 위험해!", true),
            "입울 밖은 윟엄해!".to_owned()
        );
        assert_eq!(
            super::pullup_choseong_config("이 얼마나 무시무시한 생각이니", true),
            "이 얾안아 뭇임웃잏안 생각인이".to_owned()
        );
    }
}
