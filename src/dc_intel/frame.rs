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
    pub fid: u64, // frame_id
    pub finished: bool,
    pub duration: String,

    // Donchain Channel
    pub med_high: f64,
    pub med_low: f64,
    pub big_high: f64,
    pub big_low: f64,

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

    // pub ticks_ohlc: [f64; 4], // open, high, low, close of frame ticks
    #[serde(skip)]
    pub ohlc: SimpleCandle,
    #[serde(skip)]
    pub dc_strength: DCStrength,
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

    pub fn set_trend(&mut self) {
        // set trend
        let v = &self.vel;
        let sign = if v.avg_vel_pip > 0. { 1. } else { -1. };

        let mut trend_base = v.end_vel_pip / (v.avg_vel_pip);
        let trend = trend_base * v.end_vel_pip;

        self.trd1 = trend;

        // trd2 - ignore lost momentums
        let trend_base = if v.end_vel_pip.abs() > v.avg_vel_pip.abs() * 0.75 {
            (v.end_vel_pip / (v.avg_vel_pip)) // always +
        } else {
            0.
        };

        let trend = trend_base * v.end_vel_pip;

        self.trd2 = trend;
    }

    pub fn to_csv(&self) -> (FrameMem, SimpleCandle, VelRes, DCStrength) {
        (
            self.clone(),
            self.ohlc.clone(),
            self.vel.clone(),
            self.dc_strength.clone(),
        )
    }

    pub fn to_csv_bk(&self) -> (FrameMem, SimpleCandle, VelRes) {
        (self.clone(), self.ohlc.clone(), self.vel.clone())
    }
}
