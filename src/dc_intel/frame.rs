use super::*;
use crate::base::*;
use crate::candle::{Tick, TimeSerVec};
use crate::helper;
use crate::ta::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FrameMemConfig {
    pub small_tick: u64,
    pub medium_tick: u64,
    pub big_tick: u64,
    pub vel1_period: u64,
    pub vel2_period: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct FrameMem {
    pub frame_id: u64,
    pub finished: bool,
    pub duration: String,

    // Donchain Channel
    pub med_high: f64,
    pub med_low: f64,
    pub big_high: f64,
    pub big_low: f64,

    pub spreed_min: f64,
    pub spreed_max: f64,

    // TA
    pub ma1: f64,
    #[serde(skip)]
    pub vel: VelRes,

    // pub ticks_ohlc: [f64; 4], // open, high, low, close of frame ticks
    #[serde(skip)]
    pub ohlc: SimpleCandle,
}

impl FrameMem {
    pub fn add_ticks(&mut self, ticks: &TimeSerVec<Tick>) {
        if ticks.len() == 0 {
            println!(">> Trades are empty.");
            // return Err(TErr::EmptyTradesErr);
            return;
        }
        for t in ticks.get_vec() {
            let spread = (t.ask_price - t.bid_price).abs() * 10_000.;
            if spread > self.spreed_max {
                self.spreed_max = spread;
            }
            if spread < self.spreed_min || self.spreed_min == 0. {
                self.spreed_min = spread;
            }
        }

        self.ohlc = SimpleCandle::new(ticks.get_vec());
        let dur = self.ohlc.close_time - self.ohlc.open_time;
        self.duration = helper::to_duration(dur as i64);
    }

    pub fn to_csv(&self) -> (FrameMem, SimpleCandle, VelRes) {
        (self.clone(), self.ohlc.clone(), self.vel.clone())
    }
}
