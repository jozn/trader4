use super::*;
use crate::bar::*;
use crate::collector::row_data::BTickData;
use crate::configs::assets::Pair;
use crate::cortex::types::{ActionSignal, SignalMem};
use crate::json_output::MarkerJson;
use crate::ta::*;
use serde::{Deserialize, Serialize};

pub fn new_frame(mbr: &MultiBarRes) -> MLFrame {
    let p = &mbr.medium.primary;
    let pta = &mbr.medium.primary.ta;
    let b = &mbr.medium.big;
    let bta = &mbr.medium.big.ta;

    let mut f = MLFrameInfo {
        med_low: pta.dc.low,
        med_high: pta.dc.high,
        med_mid: pta.dc.middle,
        big_low: bta.dc.low,
        big_high: bta.dc.high,
        big_mid: bta.dc.middle,
        med_dc_hl_pip: (pta.dc.high - pta.dc.low) * 10_000.,
        big_dc_hl_pip: (bta.dc.high - bta.dc.low) * 10_000.,
        bar_major: mbr.major.clone(),
        bar_medium: mbr.medium.clone(),
        bars_small: mbr.smalls.clone(),
        bar_small_tip_: Default::default(),
    };
    MLFrame {
        fid: p.seq,
        info: f,
        signal_mem: None,
        signal_action: None,
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct MLFrame {
    pub fid: i32, // frame_id
    pub info: MLFrameInfo,
    #[serde(skip)]
    pub signal_mem: Option<SignalMem>,
    #[serde(skip)]
    pub signal_action: Option<ActionSignal>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct MLFrameInfo {
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
    #[serde(skip)]
    pub bar_major: PrimaryHolder,
    #[serde(skip)]
    pub bar_medium: PrimaryHolder,
    #[serde(skip)]
    pub bars_small: Vec<PrimaryHolder>,
    #[serde(skip)]
    pub bar_small_tip_: PrimaryHolder,
}

pub type FrameCsv = (
    Bar,
    MLFrameInfo,
    RPIRes,
    RPCRes,
    MACDOutput,
    DMIOutput,
    StochRes,
    MATrendOut,
    MATrendOut,
    MACDOutput,
);

impl MLFrame {
    pub fn to_csv(&self) -> FrameCsv {
        let pta = &self.info.bar_medium.primary.ta;
        let bta = &self.info.bar_medium.primary.ta;
        (
            self.info.bar_medium.primary.clone(),
            self.info.clone(),
            pta.rpi.clone(),
            pta.rpc.clone(),
            pta.macd.clone(),
            pta.dmi.clone(),
            pta.stoch.clone(),
            pta.trend.clone(),
            bta.trend.clone(),
            bta.macd.clone(),
        )
    }
    ///////////////// For Json Outputs //////////////////
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
