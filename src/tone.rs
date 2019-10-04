use crate::notation::{self, NOTATIONS_MAP};
use num::FromPrimitive;
use num_derive::FromPrimitive;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::error::Error;
use crate::notation::parser::{Parser, ParseError, Token};

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

impl FingerTone {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "#a" => Some(FingerTone::SA),
            "a" => Some(FingerTone::A),
            "#g" => Some(FingerTone::SG),
            "g" => Some(FingerTone::G),
            "#f" => Some(FingerTone::SF),
            "f" => Some(FingerTone::F),
            "e" => Some(FingerTone::E),
            "#d" => Some(FingerTone::SD),
            "d" => Some(FingerTone::D),
            "#c" => Some(FingerTone::SC),
            "c" => Some(FingerTone::C),
            "b" => Some(FingerTone::B),
            _ => None
        }
    }
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

/// 转换错误
#[derive(Debug, PartialEq)]
pub enum ConvertError {
    Parse(ParseError),
    NotFound(String),
}

impl Display for ConvertError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            ConvertError::Parse(e) => e.fmt(f),
            ConvertError::NotFound(s) => s.fmt(f),
        }
    }
}

impl From<ParseError> for ConvertError {
    fn from(e: ParseError) -> ConvertError {
        Self::Parse(e)
    }
}

impl Error for ConvertError {}

/// 转换简谱
pub fn convert_tone(content: &str, from: FingerTone, to: FingerTone) -> Result<String, ConvertError> {
    let parser = Parser::from_str(content)?;
    let mut lines = Vec::new();
    for line in parser.lines() {
        let mut new_line = Vec::new();
        for token in line {
            let s = format!("{}", token);
            let s = match token {
                Token::Notation(n) => {
                    let tone = Tone::notation_to_tone(&s, from)
                        .ok_or_else(|| ConvertError::NotFound("转换失败：出现未知音符".to_owned()))?;
                    tone.to_notation(to).to_owned()
                },
                t => s,
            };
            new_line.push(s);
        }
        lines.push(new_line);
    }

    Ok(lines.iter().map(|line| {
        line.join("")
    }).collect::<Vec<_>>().join("\n"))
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

