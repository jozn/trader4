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

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct MLFrameFeedV3 {
    pub won: i64,

    // Major time (b_)

    // TrendDirection
    pub b_plus: f64,
    pub b_minus: f64,
    pub b_diff: f64,

    pub b_os_stoch_main: f64,

    // LineDire
    pub b_ma: f64,
    pub b_mom1: f64,
    pub b_mom2: f64,
    pub b_mom1_count: i32,
    pub b_mom1_sum: f64,

    // Medium (m_)

    // TrendDirection
    pub m_plus: f64,
    pub m_minus: f64,
    pub m_diff: f64,

    pub m_os_stoch_main: f64,

    // LineDire
    pub m_ma: f64,
    pub m_mom1: f64,
    pub m_mom2: f64,
    pub m_mom1_count: i32,
    pub m_mom1_sum: f64,
}

impl MLFrame {
    pub fn to_ml_feed_csv_v3(&self, po: &Position) -> MLFrameFeedV3 {
        let won = if po.won > 0 { 1 } else { 0 };
        let medium = &self.info.bar_medium.big.ta;
        let major = &self.info.bar_major.big.ta;

        let btd = &major.td;
        let mtd = &medium.td;

        let brt = &major.rel_price;
        let mrt = &medium.rel_price;

        let bld = &major.line_dir;
        let mld = &medium.line_dir;

        let m = MLFrameFeedV3 {
            won: won,

            b_plus: btd.plus,
            b_minus: btd.minus,
            b_diff: btd.diff,

            b_os_stoch_main: brt.os_stoch_main,

            b_ma: bld.ma,
            b_mom1: bld.mom1,
            b_mom2: bld.mom2,
            b_mom1_count: bld.mom1_count,
            b_mom1_sum: bld.mom1_sum,

            m_plus: mtd.plus,
            m_minus: mtd.minus,
            m_diff: mtd.diff,

            m_os_stoch_main: mrt.os_stoch_smooth,

            m_ma: mld.ma,
            m_mom1: mld.mom1,
            m_mom2: mld.mom2,
            m_mom1_count: mld.mom1_count,
            m_mom1_sum: mld.mom1_sum,
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
