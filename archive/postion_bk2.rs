use super::*;
use crate::candle::{Tick, TA1};
use crate::configs::assets::Pair;
use crate::gate_api::{NewPos, PosRes};
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
    // Medium time frame sm_ prefix: start-medium_
    pub sm_mom: f64,
    pub sm_roc: f64,
    pub sm_atr: f64,
    pub sm_rsi: f64,
    pub sm_cci: f64,
    pub sm_macd_pip: f64,
    pub sm_fisher: f64,

    pub sm_1_vel_ma: f64,
    pub sm_1_start_vel_pip: f64,
    pub sm_1_count: u32,
    pub sm_1_avg_vel_pip: f64,
    pub sm_1_end_vel_pip: f64,
    pub sm_2_vel_ma: f64,
    pub sm_2_start_vel_pip: f64,
    pub sm_2_count: u32,
    pub sm_2_avg_vel_pip: f64,
    pub sm_2_end_vel_pip: f64,

    // Big time frame- sb_ prefix: start-big_
    pub sb_mom: f64,
    pub sb_roc: f64,
    pub sb_atr: f64,
    pub sb_rsi: f64,
    pub sb_cci: f64,
    pub sb_macd_pip: f64,
    pub sb_fisher: f64,

    pub sb_1_vel_ma: f64,
    pub sb_1_start_vel_pip: f64,
    pub sb_1_count: u32,
    pub sb_1_avg_vel_pip: f64,
    pub sb_1_end_vel_pip: f64,
    pub sb_2_vel_ma: f64,
    pub sb_2_start_vel_pip: f64,
    pub sb_2_count: u32,
    pub sb_2_avg_vel_pip: f64,
    pub sb_2_end_vel_pip: f64,
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

    pub fn to_notify(&self) -> PosRes {
        let s = self;
        PosRes {
            pos_id: s.pos_id,
            symbol_id: s.symbol_id,
            is_closed: s.finished,
            is_short: s.is_short(),
            pos_size_usd: s.pos_size_usd,
            open_time: s.open_time,
            open_price: s.open_price,
            high_exit_price: s.high_exit_price,
            low_exit_price: s.low_exit_price,
        }
    }

    pub fn is_short(&self) -> bool {
        self.direction == PosDir::Short
    }

    pub fn set_techichal_anylse(&mut self, p: &NewPos) {
        // ================ For Start Medium Time Frame =============== //
        let t = &p.ta_med;

        self.sm_mom = t.mom;
        self.sm_roc = t.roc;
        self.sm_atr = t.atr;
        self.sm_rsi = t.rsi;
        self.sm_cci = t.cci;
        self.sm_macd_pip = t.macd.macd_pip;
        self.sm_fisher = t.fisher.fisher;

        let vel_m1 = &t.vel1;
        self.sm_1_vel_ma = vel_m1.ma;
        self.sm_1_start_vel_pip = vel_m1.start_vel_pip;
        self.sm_1_count = vel_m1.count;
        self.sm_1_avg_vel_pip = vel_m1.avg_vel_pip;
        self.sm_1_end_vel_pip = vel_m1.end_vel_pip;

        let vel_m2 = &t.vel2;
        self.sm_2_vel_ma = vel_m2.ma;
        self.sm_2_start_vel_pip = vel_m2.start_vel_pip;
        self.sm_2_count = vel_m2.count;
        self.sm_2_avg_vel_pip = vel_m2.avg_vel_pip;
        self.sm_2_end_vel_pip = vel_m2.end_vel_pip;

        // ================ For Start Big Time Frame =============== //
        // Set big time frame TA
        let t = &p.ta_big;

        self.sb_mom = t.mom;
        self.sb_roc = t.roc;
        self.sb_atr = t.atr;
        self.sb_rsi = t.rsi;
        self.sb_cci = t.cci;
        self.sb_macd_pip = t.macd.macd_pip;
        self.sb_fisher = t.fisher.fisher;

        let vel_b1 = &t.vel1;
        self.sb_1_vel_ma = vel_b1.ma;
        self.sb_1_start_vel_pip = vel_b1.start_vel_pip;
        self.sb_1_count = vel_b1.count;
        self.sb_1_avg_vel_pip = vel_b1.avg_vel_pip;
        self.sb_1_end_vel_pip = vel_b1.end_vel_pip;

        let vel_b2 = &t.vel2;
        self.sb_2_vel_ma = vel_b2.ma;
        self.sb_2_start_vel_pip = vel_b2.start_vel_pip;
        self.sb_2_count = vel_b2.count;
        self.sb_2_avg_vel_pip = vel_b2.avg_vel_pip;
        self.sb_2_end_vel_pip = vel_b2.end_vel_pip;
    }
}
