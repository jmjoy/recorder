use std::fmt::{Display, Formatter, Result as FmtResult};

/// 洞
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Hole {
    /// 全开
    Open,
    /// 半按
    Half,
    /// 全按
    Close,
}

impl Display for Hole {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            Hole::Open => "○",
            Hole::Half => "◐",
            Hole::Close => "●",
        }.fmt(f)
    }
}

/// 指法
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Fingering {
    /// 左手拇指
    left_0: Hole,
    /// 左手食指
    left_1: Hole,
    /// 左手中指
    left_2: Hole,
    /// 左手无名指
    left_3: Hole,
    /// 右手食指
    right_4: Hole,
    /// 右手中指
    right_5: Hole,
    /// 右手无名指
    right_6: Hole,
    /// 右手小指
    right_7: Hole,
}

impl Fingering {
    /// 创建指法
    pub fn new( left_0: Hole, left_1: Hole, left_2: Hole, left_3: Hole, right_4: Hole, right_5: Hole, right_6: Hole, right_7: Hole) -> Self {
        Self {
            left_0,
            left_1,
            left_2,
            left_3,
            right_4,
            right_5,
            right_6,
            right_7,
        }
    }
}

impl Display for Fingering {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        format!(r#"{}
-
{}
{}
{}
-
{}
{}
{}
{}
"#, self.left_0, self.left_1, self.left_2, self.left_3, self.right_4, self.right_5, self.right_6, self.right_7).fmt(f)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::tone::Tone;

    #[test]
    fn test_fingering() {
        assert_eq!(Tone::C.to_finger().map(|f| f.to_string()), Some("●\n-\n●\n●\n●\n-\n●\n●\n●\n●\n".to_owned()));
        assert_eq!(Tone::SC.to_finger().map(|f| f.to_string()), Some("●\n-\n●\n●\n●\n-\n●\n●\n●\n◐\n".to_owned()));
        assert_eq!(Tone::D.to_finger().map(|f| f.to_string()), Some("●\n-\n●\n●\n●\n-\n●\n●\n●\n○\n".to_owned()));
        assert_eq!(Tone::SD.to_finger().map(|f| f.to_string()), Some("●\n-\n●\n●\n●\n-\n●\n●\n◐\n○\n".to_owned()));
        assert_eq!(Tone::E.to_finger().map(|f| f.to_string()), Some("●\n-\n●\n●\n●\n-\n●\n●\n○\n○\n".to_owned()));
        assert_eq!(Tone::F.to_finger().map(|f| f.to_string()), Some("●\n-\n●\n●\n●\n-\n●\n○\n●\n●\n".to_owned()));
        assert_eq!(Tone::SF.to_finger().map(|f| f.to_string()), Some("●\n-\n●\n●\n●\n-\n○\n●\n●\n○\n".to_owned()));
        assert_eq!(Tone::G.to_finger().map(|f| f.to_string()), Some("●\n-\n●\n●\n●\n-\n○\n○\n○\n○\n".to_owned()));
        assert_eq!(Tone::SG.to_finger().map(|f| f.to_string()), Some("●\n-\n●\n●\n○\n-\n●\n●\n◐\n○\n".to_owned()));
        assert_eq!(Tone::A.to_finger().map(|f| f.to_string()), Some("●\n-\n●\n●\n○\n-\n○\n○\n○\n○\n".to_owned()));
        assert_eq!(Tone::B.to_finger().map(|f| f.to_string()), Some("●\n-\n●\n○\n○\n-\n○\n○\n○\n○\n".to_owned()));
        assert_eq!(Tone::HC.to_finger().map(|f| f.to_string()), Some("●\n-\n○\n●\n○\n-\n○\n○\n○\n○\n".to_owned()));
        assert_eq!(Tone::HSC.to_finger().map(|f| f.to_string()), Some("○\n-\n●\n●\n○\n-\n○\n○\n○\n○\n".to_owned()));
        assert_eq!(Tone::HD.to_finger().map(|f| f.to_string()), Some("○\n-\n○\n●\n○\n-\n○\n○\n○\n○\n".to_owned()));
        assert_eq!(Tone::HSD.to_finger().map(|f| f.to_string()), Some("○\n-\n○\n●\n●\n-\n●\n●\n●\n○\n".to_owned()));
        assert_eq!(Tone::HE.to_finger().map(|f| f.to_string()), Some("◐\n-\n●\n●\n●\n-\n●\n●\n○\n○\n".to_owned()));
        assert_eq!(Tone::HF.to_finger().map(|f| f.to_string()), Some("◐\n-\n●\n●\n●\n-\n●\n○\n●\n○\n".to_owned()));
        assert_eq!(Tone::HSF.to_finger().map(|f| f.to_string()), Some("◐\n-\n●\n●\n●\n-\n○\n●\n○\n○\n".to_owned()));
        assert_eq!(Tone::HG.to_finger().map(|f| f.to_string()), Some("◐\n-\n●\n●\n●\n-\n○\n○\n○\n○\n".to_owned()));
        assert_eq!(Tone::HSG.to_finger().map(|f| f.to_string()), Some("◐\n-\n●\n●\n○\n-\n●\n○\n○\n○\n".to_owned()));
        assert_eq!(Tone::HA.to_finger().map(|f| f.to_string()), Some("◐\n-\n●\n●\n○\n-\n○\n○\n○\n○\n".to_owned()));
        assert_eq!(Tone::HSA.to_finger().map(|f| f.to_string()), Some("◐\n-\n●\n●\n○\n-\n●\n●\n●\n○\n".to_owned()));
        assert_eq!(Tone::HB.to_finger().map(|f| f.to_string()), Some("◐\n-\n●\n●\n○\n-\n●\n●\n○\n○\n".to_owned()));
        assert_eq!(Tone::HB.to_finger().map(|f| f.to_string()), Some("◐\n-\n●\n●\n○\n-\n●\n●\n○\n○\n".to_owned()));
        assert_eq!(Tone::HHC.to_finger().map(|f| f.to_string()), Some("◐\n-\n●\n○\n○\n-\n●\n●\n○\n○\n".to_owned()));
        assert_eq!(Tone::HHSC.to_finger().map(|f| f.to_string()), Some("◐\n-\n●\n◐\n●\n-\n●\n○\n●\n●\n".to_owned()));
        assert_eq!(Tone::HHD.to_finger().map(|f| f.to_string()), Some("◐\n-\n●\n○\n●\n-\n●\n○\n●\n◐\n".to_owned()));
    }
}
