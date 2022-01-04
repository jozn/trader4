use super::*;
use crate::base::*;
use crate::candle::{Kline, KlineTA, Tick, TimeSerVec};
use crate::helper;
use crate::ta::*;
use serde::{Deserialize, Serialize};

// pub type FrameCsv = (NECandle,NEStrength,StochRes, NEFrame,  VelRes,  VelRes2);
// pub type FrameCsv = (NECandle, DCSRes, NEDC, NEStrength, NEFrame, VelRes, VelRes2);
// pub type FrameCsv = (NECandle, DCSRes, VelRes, NEDC, NEStrength, NEFrame, VelRes2);
pub type FrameCsv = (NECandle, DCSRes, VelRes, NEFrame);

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct NEFrame {
    pub fid: u64, // frame_id
    pub finished: bool,
    pub duration: String,

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

    // TA
    // pub ma1: f64,
    // pub ma2: f64,
    #[serde(skip)]
    pub vel: VelRes,
    pub trd1: f64,
    pub trd2: f64,
    pub trd2b: f64,
    pub trd3: f64,
    pub trd4: f64,
    pub trd_ad: f64, // trend advanced with look bakc
    pub atr_p: f64,
    pub rsi: f64,
    #[serde(skip)]
    pub rsi_sth: StochRes, // rsi_stoch
    #[serde(skip)]
    pub vel2: VelRes2,

    // pub ticks_ohlc: [f64; 4], // open, high, low, close of frame ticks
    #[serde(skip)]
    pub ohlc: NECandle,
    #[serde(skip)]
    pub strength: NEStrength,
    #[serde(skip)]
    pub dc: NEDC,
    #[serde(skip)]
    pub dcs: DCSRes,
}

impl NEFrame {
    pub fn to_csv(&self) -> FrameCsv {
        (
            self.ohlc.clone(),
            self.dcs.clone(),
            self.dcs.vvv.clone(),
            // self.dc.clone(),
            // self.strength.clone(),
            // self.rsi_sth.clone(),
            self.clone(),
            // self.vel.clone(),
            // self.vel2.clone(),
        )
    }

    pub fn set_spread(&mut self, ticks: &TimeSerVec<Tick>) {
        self.spreed_min = f64::MAX;
        for t in ticks.get_vec() {
            let spread = (t.ask_price - t.bid_price).abs() * 10_000.;
            if spread > self.spreed_max {
                self.spreed_max = spread;
            }
            if spread < self.spreed_min {
                self.spreed_min = spread;
            }
        }
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

        let trd2 = trend_base * v.end_vel_pip;

        self.trd2 = trd2;
        if trd2 != 0. {
            self.trd2b = trd2 / trd2.abs(); // set 1 or -1 of trd2b
        }

        // mix trend
        let v1 = &self.vel;
        let v2 = &self.vel2;
        let trd3 = if v1.avg_vel_pip > 0. && v2.v2_avg_vel_pip > 0. {
            1.
        } else if v1.avg_vel_pip < 0. && v2.v2_avg_vel_pip < 0. {
            -1.
        } else {
            0.
        };
        self.trd3 = trd3;
        let trd4 = if trd3 > 0. && trd2 > 0. {
            1.
        } else if trd3 < 0. && trd2 < 0. {
            -1.
        } else {
            0.
        };
        self.trd4 = trd4;
    }

    pub fn set_advanced_trend(&mut self, frames: &Vec<NEFrame>, tick: &Tick) {
        if frames.len() > 2 {
            let rel = FramesRels {
                frames2: frames,
                last: &self,
                period: 14,
            };

            self.trd_ad = rel.trends_detection();

            // dirty code
            let rel = FramesRels {
                frames2: frames,
                last: &self,
                period: 14,
            };
            self.dc = NEDC::new(&self, tick, &rel);
        }
        self.strength = NEStrength::new(&self);
    }
}

pub fn new_frame(k_med: &KlineTA, k_big: &KlineTA) -> NEFrame {
    let med_k = &k_med.kline;
    let big_k = &k_big.kline;

    let med_ta = &k_med.ta1.ta2;
    let big_ta = &k_big.ta1.ta2;

    let dur = med_k.close_time - med_k.open_time;

    let mut frame = NEFrame {
        fid: med_k.kid,
        finished: false,
        duration: helper::to_duration(dur as i64),
        med_low: med_ta.dc.low,
        med_high: med_ta.dc.high,
        med_mid: (med_ta.dc.high + med_ta.dc.low) / 2.,
        big_low: big_ta.dc.low,
        big_high: big_ta.dc.high,
        big_mid: (big_ta.dc.high + big_ta.dc.low) / 2.,
        spreed_min: 0.0,
        spreed_max: 0.0,
        med_dc_hl_pip: (med_ta.dc.high - med_ta.dc.low) * 10_000.,
        big_dc_hl_pip: (big_ta.dc.high - big_ta.dc.low) * 10_000.,
        // ma1: 0.0,
        // ma2: 0.0,
        vel: med_ta.vel1.clone(),
        trd1: 0.0,
        trd2: 0.0,
        atr_p: med_ta.atr * 10_000.,
        rsi: med_ta.rsi,
        rsi_sth: big_ta.rsi_sth.clone(),
        vel2: med_ta.vel2.clone(),
        dcs: med_ta.dcs.clone(),
        ohlc: NECandle::new(med_k),
        strength: Default::default(),
        ..Default::default()
    };

    // frame.strength = NEStrength::new(&frame);

    frame
}
