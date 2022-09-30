use super::*;
use crate::offline::Position;
pub use serde::{Deserialize, Serialize};

// For CSV ML Fedd
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct MLFrameFeedV1 {
    pub won: i64,
    // pub med_ma: f64,
    pub med_mom: f64,
    pub med_mom_mom: f64,
    // pub big_ma: f64,
    pub big_mom: f64,
    pub big_mom_mom: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct MLFrameFeedV2 {
    pub won: i64,
    pub mom1: f64,
    pub mom2: f64,
    pub mom1_count: i32,
    pub mom1_sum: f64,
    // pub med_ma: f64,
    // pub med_mom: f64,
    // pub med_mom_mom: f64,
    // pub big_ma: f64,
    // pub big_mom: f64,
    // pub big_mom_mom: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct FrameCsvMLFeedV1 {
    // pub med_high: f64,
    // #[serde(skip)]
}

pub type FrameCsvMLFeedV1Out = (FrameCsvMLFeedV1);

impl MLFrame {
    pub fn to_ml_feed_csv_v3(&self, po: &Position) -> MLFrameFeedV2 {
        let m = &self.info.bar_medium.big.ta;
        let b = &self.info.bar_major.big.ta;

        let mmam = &m.ma_mom;
        let bmam = &b.ma_mom;
        let ld = &b.line_dir;
        let m = MLFrameFeedV2 {
            won: po.won,
            // med_ma: mmam.ma ,
            // med_mom: mmam.mom,
            // med_mom_mom: mmam.mom_mom,
            // big_ma: bmam.ma,
            // big_mom: bmam.mom,
            // big_mom_mom: bmam.mom_mom,
            mom1: ld.mom1,
            mom2: ld.mom2,
            mom1_count: ld.mom1_count,
            mom1_sum: ld.mom1_sum,
        };
        m
    }

    pub fn to_ml_feed_csv_v2(&self, po: &Position) -> MLFrameFeedV2 {
        let m = &self.info.bar_medium.big.ta;
        let b = &self.info.bar_major.big.ta;

        let mmam = &m.ma_mom;
        let bmam = &b.ma_mom;
        let ld = &b.line_dir;
        let m = MLFrameFeedV2 {
            won: po.won,
            // med_ma: mmam.ma ,
            // med_mom: mmam.mom,
            // med_mom_mom: mmam.mom_mom,
            // big_ma: bmam.ma,
            // big_mom: bmam.mom,
            // big_mom_mom: bmam.mom_mom,
            mom1: ld.mom1,
            mom2: ld.mom2,
            mom1_count: ld.mom1_count,
            mom1_sum: ld.mom1_sum,
        };
        m
    }

    pub fn to_ml_feed_csv_v1(&self, po: &Position) -> MLFrameFeedV1 {
        let m = &self.info.bar_medium.big.ta;
        let b = &self.info.bar_major.big.ta;

        let mmam = &m.ma_mom;
        let bmam = &b.ma_mom;
        let m = MLFrameFeedV1 {
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
