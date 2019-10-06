use std::fmt::{Display, Formatter, Result as FmtResult};

/// 洞
pub enum Hole {
    /// 全开
    Open,
    /// 半按
    Half,
    /// 全按
    Close,
}

/// 指法
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
        unimplemented!()
    }
}

