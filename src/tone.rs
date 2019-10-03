use crate::notation;

/// 音调：竖笛的两个八度
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
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
}

