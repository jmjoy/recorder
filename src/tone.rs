use crate::notation::{self, NOTATIONS_MAP};
use crate::finger::{Fingering, Hole};
use num::FromPrimitive;
use num_derive::FromPrimitive;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::error::Error;
use crate::notation::parser::{Parser, ParseError, Token};

/// 音调：竖笛的两个八度
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, FromPrimitive)]
#[repr(usize)]
pub enum Tone {
    LLC = 1,
    LLSC,
    LLD,
    LLSD,
    LLE,
    LLF,
    LLSF,
    LLG,
    LLSG,
    LLA,
    LLSA,
    LLB,
    LC,
    LSC,
    LD,
    LSD,
    LE,
    LF,
    LSF,
    LG,
    LSG,
    LA,
    LSA,
    LB,
    C,
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
    HHSD,
    HHE,
    HHF,
    HHSF,
    HHG,
    HHSG,
    HHA,
    HHSA,
    HHB,
}

impl Tone {
    /// 英式竖笛指法
    pub fn to_finger(self) -> Option<Fingering> {
        match self {
            Tone::C => Some(Fingering::new(
                Hole::Close,
                Hole::Close,
                Hole::Close,
                Hole::Close,
                Hole::Close,
                Hole::Close,
                Hole::Close,
                Hole::Close,
            )),
            Tone::SC => Some(Fingering::new(
                Hole::Close,
                Hole::Close,
                Hole::Close,
                Hole::Close,
                Hole::Close,
                Hole::Close,
                Hole::Close,
                Hole::Half,
            )),
            Tone::D => Some(Fingering::new(
                Hole::Close,
                Hole::Close,
                Hole::Close,
                Hole::Close,
                Hole::Close,
                Hole::Close,
                Hole::Close,
                Hole::Open,
            )),
            Tone::SD => Some(Fingering::new(
                Hole::Close,
                Hole::Close,
                Hole::Close,
                Hole::Close,
                Hole::Close,
                Hole::Close,
                Hole::Half,
                Hole::Open,
            )),
            Tone::E => Some(Fingering::new(
                Hole::Close,
                Hole::Close,
                Hole::Close,
                Hole::Close,
                Hole::Close,
                Hole::Close,
                Hole::Open,
                Hole::Open,
            )),
            Tone::F => Some(Fingering::new(
                Hole::Close,
                Hole::Close,
                Hole::Close,
                Hole::Close,
                Hole::Close,
                Hole::Open,
                Hole::Close,
                Hole::Close,
            )),
            Tone::SF => Some(Fingering::new(
                Hole::Close,
                Hole::Close,
                Hole::Close,
                Hole::Close,
                Hole::Open,
                Hole::Close,
                Hole::Close,
                Hole::Open,
            )),
            Tone::G => Some(Fingering::new(
                Hole::Close,
                Hole::Close,
                Hole::Close,
                Hole::Close,
                Hole::Open,
                Hole::Open,
                Hole::Open,
                Hole::Open,
            )),
            Tone::SG => Some(Fingering::new(
                Hole::Close,
                Hole::Close,
                Hole::Close,
                Hole::Open,
                Hole::Close,
                Hole::Close,
                Hole::Half,
                Hole::Open,
            )),
            Tone::A => Some(Fingering::new(
                Hole::Close,
                Hole::Close,
                Hole::Close,
                Hole::Open,
                Hole::Open,
                Hole::Open,
                Hole::Open,
                Hole::Open,
            )),
            Tone::SA => Some(Fingering::new(
                Hole::Close,
                Hole::Close,
                Hole::Open,
                Hole::Close,
                Hole::Close,
                Hole::Open,
                Hole::Open,
                Hole::Open,
            )),
            Tone::B => Some(Fingering::new(
                Hole::Close,
                Hole::Close,
                Hole::Open,
                Hole::Open,
                Hole::Open,
                Hole::Open,
                Hole::Open,
                Hole::Open,
            )),
            Tone::HC => Some(Fingering::new(
                Hole::Close,
                Hole::Open,
                Hole::Close,
                Hole::Open,
                Hole::Open,
                Hole::Open,
                Hole::Open,
                Hole::Open,
            )),
            Tone::HSC => Some(Fingering::new(
                Hole::Open,
                Hole::Close,
                Hole::Close,
                Hole::Open,
                Hole::Open,
                Hole::Open,
                Hole::Open,
                Hole::Open,
            )),
            Tone::HD => Some(Fingering::new(
                Hole::Open,
                Hole::Open,
                Hole::Close,
                Hole::Open,
                Hole::Open,
                Hole::Open,
                Hole::Open,
                Hole::Open,
            )),
            Tone::HSD => Some(Fingering::new(
                Hole::Open,
                Hole::Open,
                Hole::Close,
                Hole::Close,
                Hole::Close,
                Hole::Close,
                Hole::Close,
                Hole::Open,
            )),
            Tone::HE => Some(Fingering::new(
                Hole::Half,
                Hole::Close,
                Hole::Close,
                Hole::Close,
                Hole::Close,
                Hole::Close,
                Hole::Open,
                Hole::Open,
            )),
            Tone::HF => Some(Fingering::new(
                Hole::Half,
                Hole::Close,
                Hole::Close,
                Hole::Close,
                Hole::Close,
                Hole::Open,
                Hole::Close,
                Hole::Open,
            )),
            Tone::HSF => Some(Fingering::new(
                Hole::Half,
                Hole::Close,
                Hole::Close,
                Hole::Close,
                Hole::Open,
                Hole::Close,
                Hole::Open,
                Hole::Open,
            )),
            Tone::HG => Some(Fingering::new(
                Hole::Half,
                Hole::Close,
                Hole::Close,
                Hole::Close,
                Hole::Open,
                Hole::Open,
                Hole::Open,
                Hole::Open,
            )),
            Tone::HSG => Some(Fingering::new(
                Hole::Half,
                Hole::Close,
                Hole::Close,
                Hole::Open,
                Hole::Close,
                Hole::Open,
                Hole::Open,
                Hole::Open,
            )),
            Tone::HA => Some(Fingering::new(
                Hole::Half,
                Hole::Close,
                Hole::Close,
                Hole::Open,
                Hole::Open,
                Hole::Open,
                Hole::Open,
                Hole::Open,
            )),
            Tone::HSA => Some(Fingering::new(
                Hole::Half,
                Hole::Close,
                Hole::Close,
                Hole::Open,
                Hole::Close,
                Hole::Close,
                Hole::Close,
                Hole::Open,
            )),
            Tone::HB => Some(Fingering::new(
                Hole::Half,
                Hole::Close,
                Hole::Close,
                Hole::Open,
                Hole::Close,
                Hole::Close,
                Hole::Open,
                Hole::Open,
            )),
            Tone::HHC => Some(Fingering::new(
                Hole::Half,
                Hole::Close,
                Hole::Open,
                Hole::Open,
                Hole::Close,
                Hole::Close,
                Hole::Open,
                Hole::Open,
            )),
            Tone::HHSC => Some(Fingering::new(
                Hole::Half,
                Hole::Close,
                Hole::Half,
                Hole::Close,
                Hole::Close,
                Hole::Open,
                Hole::Close,
                Hole::Close,
            )),
            Tone::HHD => Some(Fingering::new(
                Hole::Half,
                Hole::Close,
                Hole::Open,
                Hole::Close,
                Hole::Close,
                Hole::Open,
                Hole::Close,
                Hole::Half,
            )),
            _ => None,
        }
    }
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
            let index = index + FingerTone::C as usize + Tone::C as usize - finger_tone as usize - notation::TONE_C_START;
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
                t => s.to_owned(),
            };
            new_line.push(s);
        }
        lines.push(new_line);
    }

    Ok(lines.iter().map(|line| {
        line.join("")
    }).collect::<Vec<_>>().join("\n"))
}

/// 竖笛数字简谱可视化
pub fn visualize_tone(content: &str, finger_tone: FingerTone) -> Result<String, ConvertError> {
    let parser = Parser::from_str(content)?;
    let mut lines = Vec::new();

    for line in parser.lines() {
        if line.is_empty() {
            continue;
        }

        let mut fingers_list = vec![Vec::new(); 11];

        for token in &line {
            let finger: String;
            let s = token.to_string();
            let fingers = match token {
                Token::Notation(n) => {
                    let tone = Tone::notation_to_tone(&s, finger_tone)
                        .ok_or_else(|| ConvertError::NotFound(format!("出现未知音符：{}", &s)))?;
                    finger = tone.to_finger()
                        .ok_or_else(|| ConvertError::NotFound(format!("这个音调竖笛吹不了的音符：{}", &s)))?
                        .to_string()
                        .trim()
                        .to_string();
                    let finger = finger.split('\n');
                    finger.collect::<Vec<_>>()
                },
                Token::Whitespace => vec!["     "; 11],
                _ => vec![&*s; 11],
            };

            if !fingers.is_empty() {
                for (index, item) in fingers.iter().enumerate() {
                    fingers_list[index].push(item.to_string());
                }
            }
        }

        for fingers in fingers_list {
            lines.push(fingers);
        }

        lines.push(line.iter().map(|s| s.to_string()).collect::<Vec<_>>());

        lines.push(Vec::new());
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
        assert_eq!(Tone::notation_to_tone("(5)", FingerTone::C), Some(Tone::LG));
        assert_eq!(Tone::notation_to_tone("(7)", FingerTone::C), Some(Tone::LB));
        assert_eq!(Tone::notation_to_tone("(#7)", FingerTone::C), None);
        assert_eq!(Tone::notation_to_tone("[[3]]", FingerTone::B), Some(Tone::HHSD));
    }
}

