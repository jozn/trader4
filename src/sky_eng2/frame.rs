use super::*;
use crate::base::*;
use crate::candle::{Kline, KlineTA, Tick, TimeSerVec};
use crate::collector::row_data::BTickData;
use crate::helper;
use crate::ta::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
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

    pub spreed_min: f64,
    pub spreed_max: f64,
    pub med_dc_hl_pip: f64,
    pub big_dc_hl_pip: f64,

    #[serde(skip)]
    pub score: Score,

    #[serde(skip)]
    pub bar: PrimaryHolder,
}

pub fn new_frame(ph: &PrimaryHolder) -> SFrame {
    let p = &ph.primary;
    let b = &ph.big;

    let mut f = SFrame {
        fid: p.seq,
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
        score: Default::default(),
        bar: ph.clone(),
    };
    f.score = Score::new(&f);

    f
}

pub type FrameCsv = (
    // SCandle,
    Score,
    // MACDOutput,
    // DMIOutput,
    // StochRes,
    // MATrendOut,
    // SFrame_Dep,
    // MATrendOut,
);

impl SFrame {
    pub fn to_csv(&self) -> FrameCsv {
        (
            // self.ohlc.clone(),
            self.score.clone(),
            // self.macd.clone(),
            // self.dmi.clone(),
            // self.stoch.clone(),
            // self.trend.clone(),
            // "BIG".to_string(),
            // self.clone(),
            // self.b_trend.clone(),
        )
    }

    // pub fn set_spread(&mut self, ticks: &TimeSerVec<Tick>) {
    pub fn set_spread(&mut self, ticks: &Vec<BTickData>) {
        // println!("se {}", ticks.len());
        self.spreed_min = f64::MAX;
        for t in ticks {
            let spread = (t.ask_price - t.bid_price).abs() * 10_000.;
            if spread > self.spreed_max {
                self.spreed_max = spread;
            }
            if spread < self.spreed_min {
                self.spreed_min = spread;
            }
        }
    }
}
