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

    // Context flat - When rust fixed csv out move it to ctx
    // s_ prefix: start_
    pub s_ema: f64,
    pub s_mom: f64,
    pub s_roc: f64,
    pub s_atr: f64,
    pub s_rsi: f64,
    pub s_cci: f64,
    pub s_macd: f64,
    pub s_fisher: f64,
    pub s_start_vel: f64,
    pub s_count: u32,
    pub s_avg_vel: f64,
    pub s_end_vel: f64,
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
    pub fn new(p: &NewPos) -> Self {
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
            ..Default::default()
        };
        res.set_techichal_anylse(p);
        res
    }

    pub fn close_pos(&mut self, param: &CloseParm) {
        self.close_time_str = to_date(param.time);
        self.duration = to_duration(self.open_time as i64 - param.time as i64);
        self.close_price = param.at_price;

        let mut pl = (self.open_price - self.close_price) * self.pos_size_usd;
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
        let t = &p.ta;

        self.s_ema = t.ema200;
        self.s_mom = t.mom;
        self.s_roc = t.roc;
        self.s_atr = t.atr;
        self.s_rsi = t.rsi;
        self.s_cci = t.cci;
        self.s_macd = t.macd.macd;
        self.s_fisher = t.fisher.fisher;

        // Set vel resutl
        let vel = &t.vel;
        self.s_start_vel = vel.start_vel;
        self.s_count = vel.count;
        self.s_avg_vel = vel.avg_vel;
        self.s_end_vel = vel.end_vel;
    }
}
