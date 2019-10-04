use crate::notation::{self, NOTATIONS_MAP};
use num::FromPrimitive;
use num_derive::FromPrimitive;

/// 音调：竖笛的两个八度
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, FromPrimitive)]
#[repr(usize)]
pub enum Tone {
    C = 1,
    SC,
    D,
    SD,
    E,
    F,
    SF,
    G,
    SG,
    A,
    SA,
    B,
    HC,
    HSC,
    HD,
    HSD,
    HE,
    HF,
    HSF,
    HG,
    HSG,
    HA,
    HSA,
    HB,
    HHC,
    HHSC,
    HHD,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[repr(usize)]
/// 音调指法，按顺序递增
pub enum FingerTone {
    SA = 1,
    A,
    SG,
    G,
    SF,
    F,
    E,
    SD,
    D,
    SC,
    C,
    B,
}

impl Tone {
    /// 返回数字符号
    pub fn to_notation(self, finger_tone: FingerTone) -> &'static str {
        let index = self as usize + finger_tone as usize + notation::TONE_C_START - FingerTone::C as usize - Tone::C as usize;
        notation::NOTATIONS[index]
    }

    /// 数字符号返回Tone
    pub fn notation_to_tone(notation: &str, finger_tone: FingerTone) -> Option<Tone> {
        NOTATIONS_MAP.get(notation).and_then(|index| {
            let index = index + 1 + FingerTone::C as usize - finger_tone as usize - notation::TONE_C_START;
            FromPrimitive::from_usize(index)
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_to_notation() {
        assert_eq!(Tone::C.to_notation(FingerTone::C), "1");
        assert_eq!(Tone::D.to_notation(FingerTone::C), "2");
        assert_eq!(Tone::C.to_notation(FingerTone::D), "(#6)");
        assert_eq!(Tone::SC.to_notation(FingerTone::SC), "1");
        assert_eq!(Tone::HHD.to_notation(FingerTone::C), "[[2]]");
        assert_eq!(Tone::C.to_notation(FingerTone::B), "#1");
        assert_eq!(Tone::HD.to_notation(FingerTone::G), "5");
        assert_eq!(Tone::HSD.to_notation(FingerTone::SG), "5");
        assert_eq!(Tone::HHD.to_notation(FingerTone::B), "[[#2]]");
    }

    #[test]
    fn test_notation_to_tone() {
        assert_eq!(Tone::notation_to_tone("1", FingerTone::C), Some(Tone::C));
        assert_eq!(Tone::notation_to_tone("1", FingerTone::D), Some(Tone::D));
        assert_eq!(Tone::notation_to_tone("[[#1]]", FingerTone::C), Some(Tone::HHSC));
        assert_eq!(Tone::notation_to_tone("#1", FingerTone::B), Some(Tone::C));
        assert_eq!(Tone::notation_to_tone("[[#2]]", FingerTone::B), Some(Tone::HHD));
        assert_eq!(Tone::notation_to_tone("[1]", FingerTone::G), Some(Tone::HG));
        assert_eq!(Tone::notation_to_tone("(7)", FingerTone::C), None);
        assert_eq!(Tone::notation_to_tone("(#7)", FingerTone::C), None);
        assert_eq!(Tone::notation_to_tone("[[3]]", FingerTone::B), None);
    }
}

