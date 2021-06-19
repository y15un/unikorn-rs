use std::convert::TryFrom;
use unicode_korean_multitool::Syllable;

pub fn flip_chojongseong_horizontally(source: &str) -> String {
    let mut destination = String::with_capacity(source.len());

    let mut chojongseong = Vec::new();
    let mut jungseong = Vec::new();
    for character in source.chars() {
        if Syllable::is_one_of_us(character) {
            let syllable = Syllable::try_from(character).unwrap();
            chojongseong.push((syllable.choseong, syllable.jongseong));
            jungseong.push(syllable.jungseong);
        }
    }

    let mut flipped = chojongseong.into_iter().rev().zip(jungseong.into_iter());
    for character in source.chars() {
        if Syllable::is_one_of_us(character) {
            let ((choseong, jongseong), jungseong) = flipped.next().unwrap();
            destination.push(char::from(Syllable {
                choseong,
                jungseong,
                jongseong,
            }));
        } else {
            destination.push(character);
        }
    }

    destination
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_flip_chojongseong_horizontally() {
        assert_eq!(
            super::flip_chojongseong_horizontally("사람이 사람답게 살아야"),
            "아아실 가답람세 아람샤".to_owned()
        );
        assert_eq!(
            super::flip_chojongseong_horizontally("아무말 대잔치"),
            "차준다 맬마이".to_owned()
        );
        assert_eq!(
            super::flip_chojongseong_horizontally("집사가말여! 잘못해서 않해서!!!! 어!!!!!"),
            "이사하않셔! 하못잴어 말개서!!!! 접!!!!!".to_owned()
        );
        assert_eq!(
            super::flip_chojongseong_horizontally("두뇌 3000% 가동중"),
            "중됭 3000% 가노두".to_owned()
        );
    }
}
