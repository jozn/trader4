use super::*;
use crate::candle::{Tick, TA1};
use crate::configs::assets::Pair;
use crate::gate_api::{NewPos, PosRes};
use crate::helper;
use chrono::*;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Position {
    pub pos_id: u64,
    pub fid: u64,
    pub won: i64,
    pub symbol_id: i64,
    pub direction: PosDir,
    pub pos_size_usd: f64,
    pub got_assets: f64,
    pub open_time: u64,
    pub open_time_str: String,
    pub updates: u64,
    pub open_price: f64,
    pub high_exit_price: f64,
    pub low_exit_price: f64,
    pub close_price: f64,
    pub close_time: u64,
    pub close_time_str: String,
    #[serde(skip)]
    pub finished: bool, // tod: status
    pub duration: String,
    pub profit: f64,
    #[serde(skip)]
    pub spread_fees: f64,
    #[serde(skip)]
    pub final_balance: f64,
    pub touch_low_pip: f64,
    pub touch_high_pip: f64,
    pub locked: f64,

    #[serde(skip_serializing)]
    pub ta: PositionTA,
    #[serde(skip)]
    pub new_pos: NewPos,
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
        assert!(p.size_base > 5);
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

        let got_assets = p.size_base as f64 * p.at_price;

        let mut res = Self {
            pos_id: 0,
            symbol_id: p.symbol_id,
            direction: dir,
            pos_size_usd: p.size_base as f64,
            got_assets,
            open_time: p.time_s,
            open_price: p.at_price,
            open_time_str: helper::to_date(p.time_s),
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
            new_pos: p.clone(),

            ..Default::default()
        };
        // res.set_techichal_anylse(p);
        res.ta.set_start_ta(p);
        res
    }

    pub fn close_pos(&mut self, param: &CloseParm) {
        self.close_time_str = helper::to_date(param.time);
        self.duration = helper::to_duration(self.open_time as i64 - param.time as i64);
        self.close_price = param.at_price;

        let price_diff_percentage = (self.close_price - self.open_price) / self.open_price;
        let mut pl =  price_diff_percentage * self.pos_size_usd;
        if self.is_short() {
            pl = -pl;
        }

        if pl > 0. {
            self.won = 1;
        } else {
            self.won = -1;
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
}
