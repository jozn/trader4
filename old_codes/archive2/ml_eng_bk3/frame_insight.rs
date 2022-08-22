use super::*;
use crate::bar::Bar;
use serde::{Deserialize, Serialize};

// This csv is used for inlining in Trades outputs, should be common among all sig_engs
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct MLFrameTradeInsight {
    pub fid: i32, // frame_id
    pub bar: Bar, // frame_id
}

// This csv is used for inlining in Trades outputs
pub type MLFrameInsightCsv = (Bar,);

impl MLFrameTradeInsight {
    pub fn to_csv(&self) -> MLFrameInsightCsv {
        (self.bar.clone(),)
    }
}
