use super::*;
use crate::bar::Bar;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct MLFrameInsight {
    pub fid: i32, // frame_id
    pub bar: Bar, // frame_id
}

pub type MLFrameInsightCsv = (
    Bar,
    // MLFrameInfo,
    // RPIRes,
    // RPCRes,
    // MACDOutput,
    // DMIOutput,
    // StochRes,
    // MATrendOut,
    // MATrendOut,
    // MACDOutput,
);

impl MLFrameInsight {
    pub fn to_csv(&self) -> MLFrameInsightCsv {
        // let pta = &self.info.bar_medium.primary.ta;
        // let bta = &self.info.bar_medium.primary.ta;
        (
            self.bar.clone(),
            // self.info.bar_medium.primary.clone(),
            // self.info.clone(),
            // pta.rpi.clone(),
            // pta.rpc.clone(),
            // pta.macd.clone(),
            // pta.dmi.clone(),
            // pta.stoch.clone(),
            // pta.trend.clone(),
            // bta.trend.clone(),
            // bta.macd.clone(),
        )
    }
}
