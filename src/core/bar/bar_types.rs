use super::*;
use prost::Message;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BarConfig {
    pub primary_ticks: u64,
    pub big_ticks: u64, // big must be multiple of primary
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct PrimaryHolder {
    pub primary: Bar,
    pub big: Bar,
    pub finish_primary: bool,
    pub finish_big: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct JsonOHLC {
    pub date: i64,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
}
