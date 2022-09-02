use super::*;
use crate::bar::Bar;
use crate::offline::Position;
use serde::{Deserialize, Serialize};

// This csv is used for inlining in Trades outputs, should be common among all sig_engs
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct MLFrameTradeInsight {
    pub fid: i32, // frame_id
    pub bar: Bar, // frame_id
}

// This csv is used for inlining in Trades outputs
pub type MLFrameInsightCsv = (Bar,);
// pub type MLFrameFeedCsv = (Bar,i64);
pub type MLFrameFeedCsv = MLFrameFeedCsv2;

impl MLFrameTradeInsight {
    pub fn to_csv(&self) -> MLFrameInsightCsv {
        (self.bar.clone(),)
    }
    pub fn to_ml_feed_csv(&self, po: &Position) -> MLFrameFeedCsv {
        let ml = MLFrameFeedCsv2 {
            fid: po.won,
            bar: self.bar.clone(),
        };
        ml
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct MLFrameFeedCsv2 {
    pub fid: i64, // frame_id
    pub bar: Bar, // frame_id
}
