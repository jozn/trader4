use super::*;
use crate::candle::Tick;
use chrono::*;
use serde::{Deserialize, Serialize};

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
            finished: false,
            profit_xpip: 0,
            profit: 0.0,
            final_balance: 0.0,
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
            finished: false,
            profit_xpip: 0,
            profit: 0.0,
            final_balance: 0.0,
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

    pub(crate) fn should_close(&self, close_price: XPrice) -> bool {
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
