use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CsvOut_DEP {
    pub time: String,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
    pub hlc3: f64,
    // TA
    pub ema: f64,
    pub hull: f64,
    pub roc: i64,
    pub mom: i64,
    pub cci: i64,
}
