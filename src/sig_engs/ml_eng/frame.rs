use super::*;
use crate::bar::*;
use crate::collector::row_data::BTickData;
use crate::configs::assets::Pair;
// use crate::cortex_old::types::{ActionSignal, SignalMem};
use crate::cortex::FlagsRow;
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
    let insight = MLFrameTradeInsight {
        fid: 53,
        bar: p.clone(),
    };
    MLFrame {
        fid: p.seq,
        info: f,
        insight,
        score: TScore::new(mbr),
        signals: vec![],
    }
}

#[derive(Debug, Serialize, Clone, Default)]
pub struct MLFrame {
    pub fid: i32, // frame_id
    pub info: MLFrameInfo,
    pub insight: MLFrameTradeInsight,
    #[serde(skip)]
    pub score: TScore,
    pub signals: Vec<FlagsRow>,
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
    // #[serde(skip)]
    pub bar_major: PrimaryHolder,
    // #[serde(skip)]
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

pub type FrameCsvV2 = (Bar,);

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

    pub fn to_csv_v2(&self) -> FrameCsvV2 {
        let pta = &self.info.bar_medium.primary.ta;
        let bta = &self.info.bar_medium.primary.ta;
        (self.info.bar_medium.primary.clone(),)
    }
    ///////////////// For Json Outputs //////////////////
    pub fn get_frames_markers(&self) -> Vec<MarkerJson> {
        let mut arr = vec![];
        // let time =
        for f in &self.signals {
            // todo: mathc static &str
            let m = if f.type_key == EARLY_LONG {
                MarkerJson {
                    time: f.time_sec,
                    marker_key: format!("el_{}", f.medium_bar_id),
                    position: "belowBar".to_string(),
                    color: "#ae4bd5".to_string(),
                    shape: "circle".to_string(),
                    text: format!(""),
                }
            } else if f.type_key == FINAL_LONG {
                MarkerJson {
                    time: f.time_sec,
                    marker_key: format!("ll_{}", f.medium_bar_id),
                    position: "belowBar".to_string(),
                    color: "#2196F3".to_string(),
                    shape: "arrowUp".to_string(),
                    text: format!(""),
                }
            } else {
                panic!("unknown signal marker to json")
            };
            arr.push(m);
        }
        arr
    }
}
