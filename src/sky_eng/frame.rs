use super::*;
// use crate::base::*;
use crate::bar::*;
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
    pub tscore: TScore,

    #[serde(skip)]
    pub bar_major: PrimaryHolder,
    #[serde(skip)]
    pub bar_medium: PrimaryHolder,
    #[serde(skip)]
    pub bars_small: Vec<PrimaryHolder>,

    // signals
    pub sign_buy: bool,
    pub sign_sell: bool,
    pub buy2: bool,
    pub sell2: bool,

    #[serde(skip)]
    pub buys: Vec<i64>,
    #[serde(skip)]
    pub sells: Vec<i64>,
}

pub fn new_frame(ph_medium: &PrimaryHolder, ph_major: &PrimaryHolder) -> SFrame {
    let p = &ph_medium.primary;
    let pta = &ph_medium.primary.ta;
    let b = &ph_medium.big;
    let bta = &ph_medium.big.ta;

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
        bar_major: ph_major.clone(),
        bar_medium: ph_medium.clone(),
        ..Default::default()
    };
    f.score = Score::new(&f);
    f.tscore = TScore::new(&f);

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
    MACDOutput,
    // SFrame_Dep,
    // MATrendOut,
);

impl SFrame {
    pub fn to_csv(&self) -> FrameCsv {
        let pta = &self.bar_medium.primary.ta;
        let bta = &self.bar_medium.big.ta;
        (
            self.bar_medium.primary.clone(),
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
            self.bar_medium.big.clone(),
            bta.trend.clone(),
            bta.macd.clone(),
        )
    }

    pub fn to_json(&self) -> FrameJsonOut {
        let pta = &self.bar_medium.primary.ta;
        let bta = &self.bar_medium.big.ta;
        FrameJsonOut {
            ohlc: self.bar_medium.primary.to_json_out(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct FrameJsonOut {
    pub ohlc: JsonOHLC,
}
