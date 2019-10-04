use std::collections::HashMap;
use lazy_static::lazy_static;
use std::fmt::{Display, Formatter, Result as FmtResult};

/// 音符列表
pub const NOTATIONS: [&'static str; 46] = [
    "((5))", "((#5))", "((6))", "((#6))", "((7))",
    "(1)", "(#1)", "(2)", "(#2)", "(3)", "(4)", "(#4)", "(5)", "(#5)", "(6)", "(#6)", "(7)",
    "1", "#1", "2", "#2", "3", "4", "#4", "5", "#5", "6", "#6", "7",
    "[1]", "[#1]", "[2]", "[#2]", "[3]", "[4]", "[#4]", "[5]", "[#5]", "[6]", "[#6]", "[7]",
    "[[1]]", "[[#1]]", "[[2]]", "[[#2]]", "[[3]]",
];

lazy_static! {
    pub static ref NOTATIONS_MAP: HashMap<&'static str, usize> = {
        let mut map = HashMap::new();
        NOTATIONS.iter().enumerate().for_each(|(index, &item)| {
            map.insert(item, index);
        });
        map
    };
}

/// C调一号位坐标
pub const TONE_C_START: usize = 17;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NotationType {
    LLow,
    Low,
    Normal,
    High,
    HHigh,
}

/// 音符单元
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Notation {
    r#type: NotationType,
    number: u8,
    is_sharp: bool,
}

impl Display for Notation {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        let sharp = if self.is_sharp { "#" } else { "" };
        match self.r#type {
            NotationType::LLow => format!("(({}{}))", sharp, self.number),
            NotationType::Low => format!("({}{})", sharp, self.number),
            NotationType::Normal => format!("{}{}", sharp, self.number),
            NotationType::High => format!("[{}{}]", sharp, self.number),
            NotationType::HHigh => format!("[[{}{}]]", sharp, self.number),
        }.fmt(f)
    }
}

pub mod parser {
    use std::fmt::{Display, Formatter, Result as FmtResult};
    use crate::tone::Tone;
    use std::error::Error;
    use super::{Notation, NotationType};
    use std::collections::LinkedList;

    type ParseResult<T> = Result<T, ParseError>;

    /// 解析错误
    #[derive(Debug, PartialEq)]
    pub struct ParseError(String);

    impl Display for ParseError {
        fn fmt(&self, f: &mut Formatter) -> FmtResult {
            self.0.fmt(f)
        }
    }

    impl Error for ParseError {}

    #[derive(Debug, PartialEq)]
    pub enum Token {
        Notation(Notation),
        Raw(String),
        Whitespace,
    }

    /// 简单数字谱解析器
    #[derive(Debug, PartialEq)]
    pub struct Parser {
        inner: Vec<Vec<Token>>,
    }

    impl Parser {
        pub fn from_str(s: &str) -> ParseResult<Self> {
            let mut lines = Vec::new();

            for line in s.lines() {
                let mut vec = Vec::new();
                let line = line.trim();
                if !line.is_empty() {
                    // 注释
                    if line.starts_with("//") {
                        vec.push(Token::Raw(line.to_owned()));
                    } else {
                        Self::parse_token_item(line, &mut vec)?;
                    }
                }
                lines.push(vec);
            }

            Ok(Self {
                inner: lines,
            })
        }

        fn parse_token_item(line: &str, vec: &mut Vec<Token>) -> ParseResult<()> {
            let mut line: LinkedList<_> = line.chars().collect();

            let mut is_sharp = false;
            let mut is_low = false;
            let mut is_double_low = false;
            let mut is_high = false;
            let mut is_double_high = false;

            loop {
                match line.pop_front() {
                    Some(c) => match c {
                        ' ' => {
                            vec.push(Token::Whitespace);
                        }
                        '#' => {
                            is_sharp = true;
                        }
                        '(' => match line.pop_front() {
                            Some('(') => {
                                is_double_low = true;
                            }
                            Some(c) => {
                                is_low = true;
                                line.push_front(c);
                            }
                            None => break,
                        }
                        '[' => match line.pop_front() {
                            Some('[') => {
                                is_double_high = true;
                            }
                            Some(c) => {
                                is_high = true;
                                line.push_front(c);
                            }
                            None => break,
                        }
                        c => {
                            let x = match c.to_digit(8) {
                                Some(x) if x >= 1 && x <= 7 => x,
                                _ => return Err(ParseError(format!("未知字符: {:?}", c))),
                            };

                            if is_low {
                                if line.pop_front() != Some(')') {
                                    return Err(ParseError(format!("不以)结尾")));
                                }
                                vec.push(Token::Notation(Notation {
                                    r#type: NotationType::Low,
                                    number: x as u8,
                                    is_sharp,
                                }));
                            } else if is_high  {
                                if line.pop_front() != Some(']') {
                                    return Err(ParseError(format!("不以]结尾")));
                                }
                                vec.push(Token::Notation(Notation {
                                    r#type: NotationType::High,
                                    number: x as u8,
                                    is_sharp,
                                }));
                            } else if is_double_low {
                                if line.pop_front() != Some(')') || line.pop_front() != Some(')') {
                                    return Err(ParseError(format!("不以))结尾")));
                                }
                                vec.push(Token::Notation(Notation {
                                    r#type: NotationType::LLow,
                                    number: x as u8,
                                    is_sharp,
                                }));
                            } else if is_double_high {
                                if line.pop_front() != Some(']') || line.pop_front() != Some(']') {
                                    return Err(ParseError(format!("不以]]结尾")));
                                }
                                vec.push(Token::Notation(Notation {
                                    r#type: NotationType::HHigh,
                                    number: x as u8,
                                    is_sharp,
                                }));
                            } else {
                                vec.push(Token::Notation(Notation {
                                    r#type: NotationType::Normal,
                                    number: x as u8,
                                    is_sharp,
                                }));
                            }

                            if let Some(c) = line.pop_front() {
                                if c != ' ' {
                                    return Err(ParseError(format!("要以空格隔开")));
                                }
                                line.push_front(c);
                            }

                            is_sharp = false;
                            is_low = false;
                            is_double_low = false;
                            is_high = false;
                            is_double_high = false;
                        }
                    },
                    None => break,
                }
            }

            Ok(())
        }

    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn test_parse_from_str() {
            assert_eq!(Parser::from_str("1"), Ok(Parser { inner: vec![vec![Token::Notation(Notation {
                r#type: NotationType::Normal,
                number: 1,
                is_sharp: false,
            })]]}));

            assert_eq!(Parser::from_str("1 #2 3\n4  5 "), Ok(Parser { inner: vec![
                vec![
                    Token::Notation(Notation { r#type: NotationType::Normal, number: 1, is_sharp: false }),
                    Token::Whitespace,
                    Token::Notation(Notation { r#type: NotationType::Normal, number: 2, is_sharp: true }),
                    Token::Whitespace,
                    Token::Notation(Notation { r#type: NotationType::Normal, number: 3, is_sharp: false }),
                ],
                vec![
                    Token::Notation(Notation { r#type: NotationType::Normal, number: 4, is_sharp: false }),
                    Token::Whitespace,
                    Token::Whitespace,
                    Token::Notation(Notation { r#type: NotationType::Normal, number: 5, is_sharp: false }),
                ]
            ]}));

            assert_eq!(Parser::from_str("(1) [#2] ((#7)) \n[[#4]] #5"), Ok(Parser { inner: vec![
                vec![
                    Token::Notation(Notation { r#type: NotationType::Low, number: 1, is_sharp: false }),
                    Token::Whitespace,
                    Token::Notation(Notation { r#type: NotationType::High, number: 2, is_sharp: true }),
                    Token::Whitespace,
                    Token::Notation(Notation { r#type: NotationType::LLow, number: 7, is_sharp: true }),
                ],
                vec![
                    Token::Notation(Notation { r#type: NotationType::HHigh, number: 4, is_sharp: true }),
                    Token::Whitespace,
                    Token::Notation(Notation { r#type: NotationType::Normal, number: 5, is_sharp: true }),
                ]
            ]}));

            assert_eq!(Parser::from_str("(1) [2] ((7)) \n[[4]] 5"), Ok(Parser { inner: vec![
                vec![
                    Token::Notation(Notation { r#type: NotationType::Low, number: 1, is_sharp: false }),
                    Token::Whitespace,
                    Token::Notation(Notation { r#type: NotationType::High, number: 2, is_sharp: false }),
                    Token::Whitespace,
                    Token::Notation(Notation { r#type: NotationType::LLow, number: 7, is_sharp: false }),
                ],
                vec![
                    Token::Notation(Notation { r#type: NotationType::HHigh, number: 4, is_sharp: false }),
                    Token::Whitespace,
                    Token::Notation(Notation { r#type: NotationType::Normal, number: 5, is_sharp: false }),
                ]
            ]}));

            assert!(Parser::from_str("8").is_err());
            assert!(Parser::from_str("*").is_err());
            assert!(Parser::from_str(")").is_err());
            assert!(Parser::from_str("[1").is_err());
            assert!(Parser::from_str("[[1").is_err());
            assert!(Parser::from_str("(1").is_err());
            assert!(Parser::from_str("((1").is_err());
        }
    }

}