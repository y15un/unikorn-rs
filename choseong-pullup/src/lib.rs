use std::convert::TryFrom;
use unicode_korean_multitool::{Choseong, Jongseong, Syllable};

// how to interpret (jongseong_a, choseong_b, jongseong_c, extended)
// => when the current syllable has `jongseong_a` and the next syllable has 'choseong_b',
//    replace the current syllable's jongseong with `jongseong_c`
//    and replace the next syllable's choseong with `Choseong::Ieung`.
// => when `extended` is true, it's part of the extended ruleset, which violates
//    phonetic equivalence.
const RULESET: [(Option<Jongseong>, Choseong, Jongseong, bool); 28] = [
    (None, Choseong::Kiyeok, Jongseong::Kiyeok, false),
    (None, Choseong::SsangKiyeok, Jongseong::SsangKiyeok, false),
    (None, Choseong::Nieun, Jongseong::Nieun, false),
    (None, Choseong::Tikeut, Jongseong::Tikeut, false),
    (None, Choseong::Rieul, Jongseong::Rieul, false),
    (None, Choseong::Mieum, Jongseong::Mieum, false),
    (None, Choseong::Pieup, Jongseong::Pieup, false),
    (None, Choseong::Sios, Jongseong::Sios, false),
    (None, Choseong::SsangSios, Jongseong::SsangSios, false),
    (None, Choseong::Cieuc, Jongseong::Cieuc, false),
    (None, Choseong::Chieuch, Jongseong::Chieuch, false),
    (None, Choseong::Khieukh, Jongseong::Khieukh, false),
    (None, Choseong::Thieuth, Jongseong::Thieuth, false),
    (None, Choseong::Phieuph, Jongseong::Phieuph, false),
    (None, Choseong::Hieuh, Jongseong::Hieuh, true),
    (
        Some(Jongseong::Kiyeok),
        Choseong::Kiyeok,
        Jongseong::SsangKiyeok,
        true,
    ),
    (
        Some(Jongseong::Kiyeok),
        Choseong::Sios,
        Jongseong::KiyeokSios,
        false,
    ),
    (
        Some(Jongseong::Nieun),
        Choseong::Cieuc,
        Jongseong::NieunCieuc,
        false,
    ),
    (
        Some(Jongseong::Nieun),
        Choseong::Hieuh,
        Jongseong::NieunHieuh,
        true,
    ),
    (
        Some(Jongseong::Rieul),
        Choseong::Kiyeok,
        Jongseong::RieulKiyeok,
        false,
    ),
    (
        Some(Jongseong::Rieul),
        Choseong::Mieum,
        Jongseong::RieulMieum,
        false,
    ),
    (
        Some(Jongseong::Rieul),
        Choseong::Pieup,
        Jongseong::RieulPieup,
        false,
    ),
    (
        Some(Jongseong::Rieul),
        Choseong::Sios,
        Jongseong::RieulSios,
        false,
    ),
    (
        Some(Jongseong::Rieul),
        Choseong::Thieuth,
        Jongseong::RieulThieuth,
        false,
    ),
    (
        Some(Jongseong::Rieul),
        Choseong::Phieuph,
        Jongseong::RieulPhieuph,
        false,
    ),
    (
        Some(Jongseong::Rieul),
        Choseong::Hieuh,
        Jongseong::RieulHieuh,
        true,
    ),
    (
        Some(Jongseong::Pieup),
        Choseong::Sios,
        Jongseong::PieupSios,
        false,
    ),
    (
        Some(Jongseong::Sios),
        Choseong::Sios,
        Jongseong::SsangSios,
        true,
    ),
];

pub fn pullup_choseong(source: &str) -> String {
    pullup_choseong_config(source, false)
}

pub fn pullup_choseong_config(source: &str, extended_flag: bool) -> String {
    let mut destination = String::with_capacity(source.len());

    let mut characters = source.chars().peekable();
    let mut choseong_pulled = false;

    while let Some(current) = characters.next() {
        if !Syllable::is_one_of_us(current) {
            destination.push(current);

            continue;
        }
        let mut current_syllable = Syllable::try_from(current).unwrap();
        if choseong_pulled {
            current_syllable.choseong = Choseong::Ieung;
            choseong_pulled = false;
        }

        if let Some(&next) = characters.peek() {
            if !Syllable::is_one_of_us(next) {
                destination.push(char::from(current_syllable));

                continue;
            }
            let next_syllable = Syllable::try_from(next).unwrap();

            for &(
                current_jongseong_match,
                next_choseong_match,
                current_jongseong_to_be,
                is_extended,
            ) in RULESET.iter()
            {
                if current_jongseong_match == current_syllable.jongseong
                    && next_choseong_match == next_syllable.choseong
                    && (is_extended <= extended_flag)
                {
                    current_syllable.jongseong = Some(current_jongseong_to_be);
                    choseong_pulled = true;

                    break;
                }
            }
        }

        destination.push(char::from(current_syllable));
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
