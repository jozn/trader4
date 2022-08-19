use super::*;
// use crate::base::*;
use crate::bar::*;
use crate::collector::row_data::BTickData;
use crate::cortex::types::{ActionSignal, SignalMem};
use crate::helper;
use crate::json_output::MarkerJson;
use crate::ta::*;
use crate::types::SignalMemDep;
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
    #[serde(skip)]
    pub bar_small_tip: PrimaryHolder,

    #[serde(skip)]
    pub signal_mem: Option<SignalMem>,
    #[serde(skip)]
    pub signal_action: Option<ActionSignal>,
    #[serde(skip)]
    pub signal_store: Option<SignalMemDep>,
    // signals
    // pub sign_buy_dep: bool,
    // pub sign_sell_dep: bool,
    // pub buy2_dep: bool,
    // pub sell2_dep: bool,
    //
    // #[serde(skip)]
    // pub buys_dep: Vec<i64>,
    // #[serde(skip)]
    // pub sells: Vec<i64>,
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
        // f.buy1 = true;
    }
    if bta.trend.is_bearish() && pta.rpi.buy_high {
        // f.sell1 = true;
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

    pub fn get_early_mark(&self) -> Option<MarkerJson> {
        match &self.signal_mem {
            None => None,
            Some(sm) => {
                //todo short
                Some(MarkerJson {
                    time: sm.ps_time_sec,
                    position: "belowBar".to_string(),
                    color: "#ae4bd5".to_string(),
                    shape: "circle".to_string(),
                    text: format!(""),
                })
            }
        }
    }

    pub fn get_long_final_mark(&self) -> Option<MarkerJson> {
        match &self.signal_action {
            None => None,
            Some(sm) => {
                //todo short
                Some(MarkerJson {
                    time: sm.time_sec,
                    position: "belowBar".to_string(),
                    color: "#2196F3".to_string(),
                    // color: "#14a255".to_string(),
                    shape: "arrowUp".to_string(),
                    text: format!(""),
                })
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct FrameJsonOut {
    pub ohlc: JsonOHLC,
}