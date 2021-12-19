use super::*;
use crate::candle::{Tick, TA1};
use crate::configs::assets::Pair;
use crate::gate_api::NewPos;
use chrono::*;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Position {
    pub pos_id: u64,
    pub symbol_id: i64,
    pub direction: PosDir,
    pub pos_size_usd: f64,
    pub open_time: u64,
    pub open_time_str: String,
    pub open_price: f64,
    pub high_exit_price: f64,
    pub low_exit_price: f64,
    pub close_price: f64,
    pub close_time: u64,
    pub close_time_str: String,
    pub finished: bool, // tod: status
    pub duration: String,
    pub profit: f64,
    pub spread_fees: f64,
    pub final_balance: f64,
    pub touch_low_pip: f64,
    pub touch_high_pip: f64,
    pub locked: f64,

    // Context flat - When rust fixed csv out move it to ctx
    // sm_ prefix: start-medium_
    pub sm_ema: f64,
    pub sm_mom: f64,
    pub sm_roc: f64,
    pub sm_atr: f64,
    pub sm_rsi: f64,
    pub sm_cci: f64,
    pub sm_macd_pip: f64,
    pub sm_fisher: f64,
    pub sm_start_vel_pip: f64,
    pub sm_count: u32,
    pub sm_avg_vel_pip: f64,
    pub sm_end_vel_pip: f64,
    // sb_ prefix: start-big_
    pub sb_ema: f64,
    pub sb_mom: f64,
    pub sb_roc: f64,
    pub sb_atr: f64,
    pub sb_rsi: f64,
    pub sb_cci: f64,
    pub sb_macd_pip: f64,
    pub sb_fisher: f64,
    pub sb_start_vel_pip: f64,
    pub sb_count: u32,
    pub sb_avg_vel_pip: f64,
    pub sb_end_vel_pip: f64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum PosDir {
    Long,
    Short,
}

impl Default for PosDir {
    fn default() -> Self {
        PosDir::Long
    }
}

#[derive(Debug, Clone, Default)]
pub struct CloseParm {
    pub at_price: f64,
    pub time: u64, // Brain time
    pub ta: TA1,
}

impl Position {
    pub fn new(p: &NewPos, locked: f64) -> Self {
        assert!(p.size_usd > 5);
        let dir = if p.is_short {
            PosDir::Short
        } else {
            PosDir::Long
        };

        let (high, low) = if p.is_short {
            (p.stop_loose_price, p.take_profit_price)
        } else {
            (p.take_profit_price, p.stop_loose_price)
        };
        assert!(high > low);

        let mut res = Self {
            pos_id: 0,
            symbol_id: p.symbol_id,
            direction: dir,
            pos_size_usd: p.size_usd as f64,
            open_time: p.time_s,
            open_price: p.at_price,
            open_time_str: to_date(p.time_s),
            high_exit_price: high,
            low_exit_price: low,
            close_time: 0,
            close_time_str: "".to_string(),
            finished: false,
            duration: "".to_string(),
            // profit_xpip: 0,
            profit: 0.0,
            spread_fees: 0.0,
            final_balance: 0.0,
            touch_low_pip: 0.,
            touch_high_pip: 0.,
            locked: locked,

            ..Default::default()
        };
        res.set_techichal_anylse(p);
        res
    }

    pub fn close_pos(&mut self, param: &CloseParm) {
        self.close_time_str = to_date(param.time);
        self.duration = to_duration(self.open_time as i64 - param.time as i64);
        self.close_price = param.at_price;

        let mut pl = (self.close_price - self.open_price) * self.pos_size_usd;
        if self.is_short() {
            pl = -pl;
        }

        self.close_price = param.at_price;
        self.close_time = param.time;
        self.finished = true;
        self.profit = pl;
        self.spread_fees = 0.;
        self.final_balance = self.pos_size_usd + pl;
    }

    pub fn is_short(&self) -> bool {
        self.direction == PosDir::Short
    }

    pub fn set_techichal_anylse(&mut self, p: &NewPos) {
        let t = &p.ta_med;

        self.sm_ema = t.ema200;
        self.sm_mom = t.mom;
        self.sm_roc = t.roc;
        self.sm_atr = t.atr;
        self.sm_rsi = t.rsi;
        self.sm_cci = t.cci;
        self.sm_macd_pip = t.macd.macd_pip;
        self.sm_fisher = t.fisher.fisher;

        // Set vel resutl
        let vel = &t.vel;
        self.sm_start_vel_pip = vel.start_vel_pip;
        self.sm_count = vel.count;
        self.sm_avg_vel_pip = vel.avg_vel_pip;
        self.sm_end_vel_pip = vel.end_vel_pip;

        // Set big time frame TA
        let t = &p.ta_big;

        self.sb_ema = t.ema200;
        self.sb_mom = t.mom;
        self.sb_roc = t.roc;
        self.sb_atr = t.atr;
        self.sb_rsi = t.rsi;
        self.sb_cci = t.cci;
        self.sb_macd_pip = t.macd.macd_pip;
        self.sb_fisher = t.fisher.fisher;

        // Set vel resutl
        let vel = &t.vel;
        self.sb_start_vel_pip = vel.start_vel_pip;
        self.sb_count = vel.count;
        self.sb_avg_vel_pip = vel.avg_vel_pip;
        self.sb_end_vel_pip = vel.end_vel_pip;
    }
}
