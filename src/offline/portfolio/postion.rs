use super::*;
use crate::candle::{Tick, TA1};
use chrono::*;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PosParam {
    pub open_price: XPrice,
    pub pos_size: XLot,
    pub time: u64,
    pub ta: TA1,
}

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Position {
    pub pos_id: u64,
    pub direction: PosDir,
    pub pos_size_usd: f64,
    pub pos_size_xlot: XLot,
    pub open_xprice: XPrice,
    pub open_time: u64,
    pub open_time_str: String,
    pub to_exit_xpip: XPip,
    pub to_stop_loss_xpip: XPip,
    pub spread: XSpread,
    pub close_xprice: XPrice,
    pub close_time: u64,
    pub close_time_str: String,
    pub finished: bool, // tod: status
    pub duration: String,
    pub profit_xpip: XPip,
    pub profit: f64,
    pub spread_fees: f64,
    pub final_balance: f64,
    // Add tailing max/min prices
    pub max_touch: XPrice,
    pub min_touch: XPrice,
    pub tailing_loose: XPrice,

    // Context flat - When rust fixed csv out move it to ctx
    pub s_ema: f64,
    pub s_mom: f64,
    pub s_roc: f64,
    pub s_rsi: f64,
    pub s_cci: f64,
    pub s_macd: f64,
    pub s_fisher: f64,
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
    pub fn new_long(open_price: XPrice, pos_size: XLot, time: u64) -> Self {
        assert!(pos_size > 0);

        Self {
            pos_id: 0,
            direction: PosDir::Long,
            pos_size_usd: (pos_size * 1_000) as f64,
            pos_size_xlot: pos_size,
            open_xprice: open_price,
            open_time: time,
            open_time_str: to_date(time),
            to_exit_xpip: 100,
            to_stop_loss_xpip: 100,
            spread: 0,
            close_xprice: 0,
            close_time: 0,
            close_time_str: "".to_string(),
            finished: false,
            duration: "".to_string(),
            profit_xpip: 0,
            profit: 0.0,
            spread_fees: 0.0,
            final_balance: 0.0,

            max_touch: open_price,
            min_touch: open_price,
            tailing_loose: open_price - 50,
            ..Default::default()
        }
    }

    pub fn new_short(open_price: XPrice, pos_size: XLot, time: u64) -> Self {
        assert!(pos_size > 0);

        Self {
            pos_id: 0,
            direction: PosDir::Short,
            pos_size_usd: (pos_size * 1_000) as f64,
            pos_size_xlot: pos_size,
            open_xprice: open_price,
            open_time: time,
            open_time_str: to_date(time),
            to_exit_xpip: 100,
            to_stop_loss_xpip: 100,
            spread: 0,
            close_xprice: 0,
            close_time: 0,
            close_time_str: "".to_string(),
            finished: false,
            duration: "".to_string(),
            profit_xpip: 0,
            profit: 0.0,
            spread_fees: 0.0,
            final_balance: 0.0,

            max_touch: open_price,
            min_touch: open_price,
            tailing_loose: open_price + 50,
            ..Default::default()
        }
    }

    pub fn close_pos(&mut self, close_price: XPrice, time: u64) {
        self.close_time_str = to_date(time);
        self.duration = to_duration(self.open_time as i64 - time as i64);

        match self.direction {
            PosDir::Long => self.close_long(close_price, time),
            PosDir::Short => self.close_short(close_price, time),
        };
    }

    fn close_long(&mut self, close_price: XPrice, time: u64) {
        let pl_xpip = close_price - self.open_xprice;
        let pure_pl_xpip = pl_xpip - self.spread;

        self.close_xprice = close_price;
        self.close_time = time;
        self.finished = true;

        let pure_pl = self.pos_size_usd * (pure_pl_xpip as f64 / 100_000.);
        self.profit_xpip = pure_pl_xpip;
        self.profit = pure_pl;
        self.spread_fees = self.pos_size_usd * (self.spread as f64 / 100_000.);
        self.final_balance = self.pos_size_usd + pure_pl;
    }

    fn close_short(&mut self, close_price: XPrice, time: u64) {
        let pl_xpip = self.open_xprice - close_price;
        let pure_pl_xpip = pl_xpip - self.spread;

        self.close_xprice = close_price;
        self.close_time = time;
        self.finished = true;

        let pure_pl = self.pos_size_usd * (pure_pl_xpip as f64 / 100_000.);
        self.profit_xpip = pure_pl_xpip;
        self.profit = pure_pl;
        self.spread_fees = self.pos_size_usd * (self.spread as f64 / 100_000.);
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

    pub(crate) fn should_close_bk_simple(&self, close_price: XPrice) -> bool {
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
    }

    // bk
    pub fn close_pos_bk(&mut self, close_price: XPrice, time: u64) {
        // let pl_xpip = match self.direction {
        //     PosDir::Long => self.open_xprice - close_price,
        //     PosDir::Short => close_price - self.open_xprice,
        // };

        let pl_xpip = match self.direction {
            PosDir::Long => close_price - self.open_xprice,
            PosDir::Short => self.open_xprice - close_price,
        };

        let pure_pl_xpip = pl_xpip - self.spread;

        self.close_xprice = close_price;
        self.close_time = time;
        self.finished = true;

        // let pure_pl = (self.pos_size_xlot * pure_pl_xpip) as f64 / 100_000.; // todo fix cal
        let pure_pl = self.pos_size_usd * (pure_pl_xpip as f64 / 100_000.); // todo fix cal
        self.profit_xpip = pure_pl_xpip;
        self.profit = pure_pl;
        self.final_balance = self.pos_size_usd + pure_pl;
    }
}

/*
impl Ord for Position {
    fn cmp(&self, other: &Self) -> Ordering {
        self.pos_id.cmp(&other.pos_id)
    }
}

impl PartialOrd for Position {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.pos_id.partial_cmp(&other.pos_id)
    }
}
*/
