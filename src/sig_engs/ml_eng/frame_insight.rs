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
// pub type MLFrameFeedCsv = MLFrameFeedCsv2;

impl MLFrameTradeInsight {
    pub fn to_csv(&self) -> MLFrameInsightCsv {
        (self.bar.clone(),)
    }
    // pub fn to_ml_feed_csv(&self, po: &Position) -> MLFrameFeedCsv2 {
    //     let ml = MLFrameFeedCsv2 {
    //         fid: po.won,
    //         bar: self.bar.clone(),
    //     };
    // // let m = b
    //     ml
    // }
}

impl MLFrame {
    pub fn to_ml_feed_csv(&self, po: &Position) -> MLFrameFeed {
        let m = &self.info.bar_medium.big.ta;
        let b = &self.info.bar_major.big.ta;

        let mmam = &m.ma_mom;
        let bmam = &b.ma_mom;
        let m = MLFrameFeed {
            won: po.won,
            // med_ma: mmam.ma ,
            med_mom: mmam.mom,
            med_mom_mom: mmam.mom_mom,
            // big_ma: bmam.ma,
            big_mom: bmam.mom,
            big_mom_mom: bmam.mom_mom,
        };
        m
    }
}

// For CSV ML Fedd
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct MLFrameFeed {
    pub won: i64,
    // pub med_ma: f64,
    pub med_mom: f64,
    pub med_mom_mom: f64,
    // pub big_ma: f64,
    pub big_mom: f64,
    pub big_mom_mom: f64,
}
