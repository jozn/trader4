use super::*;
use crate::base::*;
use crate::candle::{Kline, KlineTA, Tick, TimeSerVec};
use crate::helper;
use crate::ta::*;
use serde::{Deserialize, Serialize};

pub type FrameCsv = (StochRes, FrameMem, Kline, VelRes, DCStrength, VelRes2);

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct FrameMem {
    pub fid: u64, // frame_id
    pub finished: bool,
    pub duration: String,

    // Donchain Channel
    pub med_low: f64,
    pub med_high: f64,
    pub med_mid: f64,
    pub big_low: f64,
    pub big_high: f64,
    pub big_mid: f64,

    pub spreed_min: f64,
    pub spreed_max: f64,
    pub med_dc_hl_pip: f64,
    pub big_dc_hl_pip: f64,

    // TA
    pub ma1: f64,
    pub ma2: f64,
    #[serde(skip)]
    pub vel: VelRes,
    pub trd1: f64,
    pub trd2: f64,
    pub atr_p: f64,
    pub rsi: f64,
    #[serde(skip)]
    pub rsi_sth: StochRes, // rsi_stoch
    #[serde(skip)]
    pub vel2: VelRes2,

    // pub ticks_ohlc: [f64; 4], // open, high, low, close of frame ticks
    #[serde(skip)]
    pub ohlc: Kline,
    #[serde(skip)]
    pub dc_strength: DCStrength,
}

impl FrameMem {
    pub fn to_csv(&self) -> FrameCsv {
        (
            self.rsi_sth.clone(),
            self.clone(),
            self.ohlc.clone(),
            self.vel.clone(),
            self.dc_strength.clone(),
            self.vel2.clone(),
        )
    }
}

pub fn new_frame(k_med: &KlineTA, k_big: &KlineTA) -> FrameMem {
    let med_k = &k_med.kline;
    let big_k = &k_big.kline;

    let med_ta = &k_med.ta1.ta2;
    let big_ta = &k_big.ta1.ta2;

    let frame = FrameMem {
        fid: med_k.kid,
        finished: false,
        duration: "".to_string(),
        med_low: 0.0,
        med_high: 0.0,
        med_mid: 0.0,
        big_low: 0.0,
        big_high: 0.0,
        big_mid: 0.0,
        spreed_min: 0.0,
        spreed_max: 0.0,
        med_dc_hl_pip: 0.0,
        big_dc_hl_pip: 0.0,
        ma1: 0.0,
        ma2: 0.0,
        vel: Default::default(),
        trd1: 0.0,
        trd2: 0.0,
        atr_p: 0.0,
        rsi: 0.0,
        rsi_sth: Default::default(),
        vel2: Default::default(),
        ohlc: med_k.clone(),
        dc_strength: Default::default(),
        // ..Default::default()
    };

    frame
}
