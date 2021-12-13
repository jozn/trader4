use super::*;
use crate::candle::{Tick, TA1};
use crate::gate_api::NewPos;
use chrono::*;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use crate::configs::assets::Pair;

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Position {
    pub pos_id: u64,
    pub symbol_id: i64,
    pub direction: PosDir,
    pub pos_size_usd: f64,
    // pub pos_size_xlot: XLot,
    // pub open_xprice: XPrice,
    pub open_time: u64,
    pub open_time_str: String,
    pub to_exit_price: f64,
    pub to_stop_loss_price: f64,
    // pub to_exit_xpip: XPip,
    // pub to_stop_loss_xpip: XPip,
    // pub spread: XSpread,
    // pub close_xprice: XPrice,
    pub close_price: f64,
    pub close_time: u64,
    pub close_time_str: String,
    pub finished: bool, // tod: status
    pub duration: String,
    // pub profit_xpip: XPip,
    pub profit: f64,
    pub spread_fees: f64,
    pub final_balance: f64,
    // Add tailing max/min prices
    // pub max_touch: XPrice,
    // pub min_touch: XPrice,
    // pub tailing_loose: XPrice,

    // Context flat - When rust fixed csv out move it to ctx
    // s_ prefix: start_
    pub s_ema: f64,
    pub s_mom: f64,
    pub s_roc: f64,
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

impl Position {

    pub fn new(p: &NewPos) -> Self {
        assert!(p.size_usd > 5);
        let dir = if p.is_short {
            PosDir::Short
        }else {
            PosDir::Long
        };

        let mut res = Self {
            pos_id: 0,
            symbol_id: p.symbol_id,
            direction: dir,
            pos_size_usd: p.size_usd as f64,
            open_time: p.time,
            open_time_str: to_date(p.time),
            to_exit_price: p.take_profit_price,
            to_stop_loss_price: p.stop_loose_price,
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

/*    pub fn new_long(p: &NewPos) -> Self {
        assert!(p.pos_size > 0);

        let mut res = Self {
            pos_id: 0,
            symbol_id: p.symbol_id,
            direction: PosDir::Long,
            pos_size_usd: p.get_usd(),
            open_time: p.time,
            open_time_str: to_date(p.time),
            to_exit_price: p.take_profit_price,
            to_stop_loss_price: p.stop_loose_price,
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

    pub fn new_short(p: &NewPos) -> Self {
        assert!(p.pos_size > 0);

        let mut res = Self {
            pos_id: 0,
            symbol_id: p.symbol_id,
            direction: PosDir::Short,
            pos_size_usd: p.get_usd(),
            open_time: p.time,
            open_time_str: to_date(p.time),
            to_exit_price: 0.0,
            to_stop_loss_price: 0.0,
            close_time: 0,
            close_time_str: "".to_string(),
            finished: false,
            duration: "".to_string(),
            profit: 0.0,
            spread_fees: 0.0,
            final_balance: 0.0,
            ..Default::default()
        };
        res.set_techichal_anylse(p);
        res
    }*/

    pub fn close_pos(&mut self, param: &NewPos) {
        self.close_time_str = to_date(param.time);
        self.duration = to_duration(self.open_time as i64 - param.time as i64);

        match self.direction {
            PosDir::Long => self.close_long(param),
            PosDir::Short => self.close_short(param),
        };
    }

    fn close_long(&mut self, param: &NewPos) {
        let pl_xpip = param.price - self.open_xprice;
        let pure_pl_xpip = pl_xpip - self.spread;

        self.close_xprice = param.price;
        self.close_time = param.time;
        self.finished = true;

        let pure_pl = self.pos_size_usd * (pure_pl_xpip as f64 / param.price_multi);
        self.profit_xpip = pure_pl_xpip;
        self.profit = pure_pl;
        self.spread_fees = self.pos_size_usd * (self.spread as f64 / param.price_multi);
        self.final_balance = self.pos_size_usd + pure_pl;
    }

    fn close_short(&mut self, param: &NewPos) {
        let pl_xpip = self.open_xprice - param.price;
        let pure_pl_xpip = pl_xpip - self.spread;

        self.close_xprice = param.price;
        self.close_time = param.time;
        self.finished = true;

        let pure_pl = self.pos_size_usd * (pure_pl_xpip as f64 / param.price_multi);
        self.profit_xpip = pure_pl_xpip;
        self.profit = pure_pl;
        self.spread_fees = self.pos_size_usd * (self.spread as f64 / param.price_multi);
        self.final_balance = self.pos_size_usd + pure_pl;
    }

    pub(crate) fn update_ailing(&mut self, price: XPrice) {
        let pl_xpip = match self.direction {
            PosDir::Long => {
                let new_sl = price - 50;
                if self.tailing_loose < new_sl {
                    self.tailing_loose = new_sl;
                }
            }
            PosDir::Short => {
                let new_sl = price + 50;
                if self.tailing_loose > new_sl {
                    self.tailing_loose = new_sl;
                }
            }
        };
    }

    pub(crate) fn should_close_tailing(&self, close_price: XPrice) -> bool {
        let mut trig = false;
        let pl_xpip = match self.direction {
            PosDir::Long => {
                if close_price < self.tailing_loose {
                    true
                } else {
                    false
                }
            }
            PosDir::Short => {
                if close_price > self.tailing_loose {
                    true
                } else {
                    false
                }
            }
        };
        pl_xpip
    }

/*    pub(crate) fn should_close_bk_simple(&self, close_price: XPrice) -> bool {
        let mut trig = false;
        let pl_xpip = match self.direction {
            PosDir::Long => {
                let pl = close_price - self.open_xprice;
                if pl > 0 && pl >= self.to_exit_xpip {
                    trig = true
                }

                if pl < 0 && pl.abs() >= self.to_stop_loss_xpip {
                    trig = true
                }
            }
            PosDir::Short => {
                let pl = self.open_xprice - close_price;
                if pl > 0 && pl >= self.to_exit_xpip {
                    trig = true
                }

                if pl < 0 && pl.abs() >= self.to_stop_loss_xpip {
                    trig = true
                }
            }
        };
        trig
    }*/

    pub fn set_techichal_anylse(&mut self, p: &NewPos) {
        let t = &p.ta;

        self.s_ema = t.ema200;
        self.s_mom = t.mom;
        self.s_roc = t.roc;
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
