use super::*;
use crate::base::*;
use crate::candle::{Kline, KlineTA, Tick, TimeSerVec};
use crate::collector::row_data::BTickData;
use crate::helper;
use crate::ta::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct SFrame {
    pub fid: i32, // frame_id

    // Donchain Channel
    pub med_low: f64,
    pub med_high: f64,
    #[serde(skip)]
    pub med_mid: f64,
    pub big_low: f64,
    pub big_high: f64,
    #[serde(skip)]
    pub big_mid: f64,

    pub med_dc_hl_pip: f64,
    pub big_dc_hl_pip: f64,

    pub trend: f64,
    pub buy1: bool,
    pub sell1: bool,

    #[serde(skip)]
    pub score: Score,

    #[serde(skip)]
    pub bar: PrimaryHolder,
}

pub fn new_frame(ph: &PrimaryHolder) -> SFrame {
    let p = &ph.primary;
    let pta = &ph.primary.ta;
    let b = &ph.big;
    let bta = &ph.big.ta;

    let mut f = SFrame {
        fid: p.seq,
        med_low: pta.dc.low,
        med_high: pta.dc.high,
        med_mid: pta.dc.middle,
        big_low: bta.dc.low,
        big_high: bta.dc.high,
        big_mid: bta.dc.middle,
        med_dc_hl_pip: (pta.dc.high - pta.dc.low) * 10_000.,
        big_dc_hl_pip: (bta.dc.high - bta.dc.low) * 10_000.,
        score: Default::default(),
        bar: ph.clone(),
        ..Default::default()
    };
    f.score = Score::new(&f);

    if bta.trend.is_bullish() {
        f.trend = 1.;
    } else {
        f.trend = -1.;
    }
    // set sell buy signals
    if bta.trend.is_bullish() && pta.rpi.buy_low {
        f.buy1 = true;
    }
    if bta.trend.is_bearish() && pta.rpi.buy_high {
        f.sell1 = true;
    }

    f
}

pub type FrameCsv = (
    Bar,
    SFrame,
    RPIRes,
    Score,
    RPCRes,
    MACDOutput,
    DMIOutput,
    StochRes,
    MATrendOut,
    // For big
    Bar,
    MATrendOut,
    // SFrame_Dep,
    // MATrendOut,
);

impl SFrame {
    pub fn to_csv(&self) -> FrameCsv {
        let pta = &self.bar.primary.ta;
        let bta = &self.bar.big.ta;
        (
            self.bar.primary.clone(),
            self.clone(),
            pta.rpi.clone(),
            self.score.clone(),
            // self.bar.big.clone(),
            pta.rpc.clone(),
            pta.macd.clone(),
            pta.dmi.clone(),
            pta.stoch.clone(),
            pta.trend.clone(),
            // big time frame
            self.bar.big.clone(),
            bta.trend.clone(),
        )
    }

    pub fn to_json(&self) -> FrameJsonOut {
        let pta = &self.bar.primary.ta;
        let bta = &self.bar.big.ta;
        FrameJsonOut {
            ohlc: self.bar.primary.to_json_out(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct FrameJsonOut {
    pub ohlc: JsonOHLC,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct JsonOHLC {
    pub date: i64,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
}
