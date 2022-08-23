use super::*;
use crate::bar::{Bar, MultiBars};
use crate::json_output::*;
use crate::ta::*;
pub use serde::{Deserialize, Serialize};

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

// pub type FrameCsvV2 = (Bar,MLFrameInfo, FrameCsv);
pub type FrameCsvV2 = (Bar, MLFrameInfo);

impl MLFrame {
    pub fn to_csv_v2_(&self) -> FrameCsv {
        self.to_csv_old_not_used()
    }
    pub fn to_csv_v2(&self) -> FrameCsvV2 {
        let pta = &self.info.bar_medium.primary.ta;
        let bta = &self.info.bar_medium.primary.ta;
        (
            self.info.bar_medium.primary.clone(),
            self.info.clone(),
            // self.to_csv_old_not_used(),
        )
    }

    pub fn to_csv_old_not_used(&self) -> FrameCsv {
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

impl JsonMaker for MLEng {
    fn get_bars(&self) -> MultiBars {
        self.mutli_bars.clone()
    }

    fn get_markers(&self, start: i64, end: i64) -> Vec<MarkerJson> {
        let mut out = vec![];
        for fm in &self.frames {
            let bar = &fm.info.bar_medium.primary;
            if !(bar.open_time >= start && bar.open_time <= end) {
                continue;
            }
            let marks = fm.get_frames_markers();
            for m in marks {
                out.push(m);
            }
        }
        // println!("markers {:?}",out);
        out
    }

    // Set custom json data
    fn set_json_data(&self, jo: &mut SkyJsonOut) {
        for fm in &self.frames {
            let bar = &fm.info.bar_medium.primary;
            // todo: fix this
            // if !(bar.open_time >= start && bar.open_time <= end) {
            //     continue;
            // }
            let time = bar.open_time / 1000;
            let score = &fm.score;
            // Add scores
            //  let score = &fm.tscore;
            jo.score_bull.push(RowJson {
                time,
                value: score.bull as f64,
            });
            jo.score_bear.push(RowJson {
                time,
                value: score.bear as f64,
            });
            jo.score_diff.push(RowJson {
                time,
                value: score.diff as f64,
            });
        }
    }
}
