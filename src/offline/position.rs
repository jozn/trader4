use super::*;
use crate::collector::row_data::BTickData;
use crate::configs::assets;
use crate::configs::assets::Pair;
use crate::gate_api::*;
use crate::helper;
use crate::helper::rond;
use crate::ta::round;
use chrono::*;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

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
    pub pair: Pair,
    pub tick: BTickData,
    pub locked: f64,
    pub time_sec: u64,
}

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Position {
    pub pos_id: u64,
    pub virtual_id: u64,
    pub fid: u64,
    pub won: i64,
    #[serde(skip)]
    pub symbol_id: i64,
    pub pair: Pair,
    #[serde(rename = "dir")]
    pub direction: PosDir,
    pub signal_key: String,
    #[serde(rename = "ssngt")]
    pub signal_strength: f64,
    #[serde(rename = "base")]
    pub base_asset_size: f64,
    #[serde(rename = "quote")]
    pub quote_asset_size: f64,
    // pub got_assets: f64,
    #[serde(skip)]
    pub open_time: u64, // seconds
    #[serde(rename = "time_o")]
    pub open_time_str: String,
    pub updates: u64,
    pub open_price: f64,
    pub exit_high_price: f64,
    pub exit_low_price: f64,
    pub close_price: f64,
    pub pl_ratio_s: f64,
    #[serde(skip)]
    pub close_time: u64, // seconds
    #[serde(rename = "time_c")]
    pub close_time_str: String,
    #[serde(skip)]
    pub finished: bool, // tod: status
    pub profit: f64,
    pub duration: String,
    pub touch_high_pip: f64,
    pub touch_low_pop: f64,
    #[serde(skip)]
    pub spread_open: f64,
    #[serde(skip)]
    pub spread_close: f64,
    #[serde(rename = "sprd")]
    pub spread_fees: f64,
    pub locked: f64,

    #[serde(skip)]
    pub new_pos: NewPosInter,
}

#[derive(Debug, Clone, Default)]
pub struct NewPosInter {
    pub new_pos: NewPosReq,
    pub tick: BTickData,
    pub locked: f64,
    pub time_sec: u64,
    pub pos_id: u64,
}

impl Position {
    pub fn new(npi: NewPosInter) -> Self {
        // assert!(p.pair.to_symbol_id() == tick)
        let p = &npi.new_pos;

        let dir = if p.is_short {
            PosDir::Short
        } else {
            PosDir::Long
        };

        let pl_ratio_s = if p.is_short {
            (p.at_price - p.exit_low_price) / (p.exit_high_price - p.at_price)
        } else {
            (p.exit_high_price - p.at_price) / (p.at_price - p.exit_low_price)
        };
        let pl_ratio_s = rond(pl_ratio_s, 2);

        let pair = npi.new_pos.pair.clone();

        let spreed_open = npi.tick.get_spread_pip(&pair);

        // Dependent: means dependent on short or long

        let mut new_position = Self {
            pos_id: npi.pos_id,
            virtual_id: 0,
            fid: 0,
            won: 0,
            symbol_id: p.pair.to_symbol_id(),
            pair: p.pair.clone(),
            direction: dir,
            signal_key: "".to_string(),
            signal_strength: 0.0,
            base_asset_size: p.base_asset_size,
            quote_asset_size: 0.0, // Dependent
            open_time: npi.time_sec,
            open_time_str: helper::to_date(npi.time_sec),
            updates: 0,
            open_price: 0.0, // Dependent
            exit_high_price: p.exit_high_price,
            exit_low_price: p.exit_low_price,
            close_price: 0.0,
            pl_ratio_s,
            close_time: 0,
            close_time_str: "".to_string(),
            finished: false,
            duration: "".to_string(),
            profit: 0.0,
            touch_low_pop: 0.0,
            touch_high_pip: 0.0,
            spread_open: spreed_open,
            spread_close: 0.0,
            spread_fees: 0.0,
            locked: npi.locked,
            new_pos: npi.clone(),
        };

        if p.is_short {
        } else {
            new_position.set_new_long(&npi);
        }

        new_position
    }

    pub fn set_new_long(&mut self, npi: &NewPosInter) {
        let p = &npi.new_pos;
        let price = npi.tick.ask_price; // higher price bar
        let used_quote_asset = p.base_asset_size * price;

        self.open_price = price;
        self.quote_asset_size = used_quote_asset;
    }

    pub fn update_touch_prices(&mut self, btick: &BTickData) {
        if self.is_long() {
            let multi = btick.pair.get_pip_multi();
            let high = (btick.bid_price - self.open_price) * multi;
            if self.touch_high_pip < high {
                self.touch_high_pip = helper::rond(high, 5);
            }

            let low = (btick.bid_price - self.open_price) * multi;
            if self.touch_low_pop > low {
                self.touch_low_pop = helper::rond(low, 5);
            }
        }
    }

    pub fn should_close(&self, btick: &BTickData) -> bool {
        if self.is_long() {
            if btick.bid_price >= self.exit_high_price || btick.bid_price <= self.exit_low_price {
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    pub fn close_pos(&mut self, param: &CloseParm) {
        let pair = &param.pair;
        let tick = &param.tick;

        self.spread_close = tick.get_spread_pip(&pair);
        self.spread_fees = ((self.spread_open + self.spread_close) / 2.); // todo 2 digits
        self.close_time = param.time_sec;
        self.close_time_str = helper::to_date(param.time_sec);
        self.duration = helper::to_duration(self.open_time as i64 - param.time_sec as i64);
        self.finished = true;

        if self.is_long() {
            self._close_long(param);
        }

        if self.profit > 0. {
            self.won = 1;
        } else {
            self.won = -1;
        }
    }

    fn _close_long(&mut self, param: &CloseParm) {
        let pair = &param.pair;
        let tick = &param.tick;

        self.close_price = tick.bid_price; // lower band of price - sell to buyers

        let pl = (self.close_price - self.open_price) * self.base_asset_size;
        self.profit = helper::rond(pl, 2);
    }

    pub fn is_short(&self) -> bool {
        self.direction == PosDir::Short
    }

    pub fn is_long(&self) -> bool {
        self.direction == PosDir::Long
    }
    pub fn is_won(&self) -> bool {
        assert!(self.finished); // only call when finshed
        self.profit > 0.
    }

    pub fn to_event(&self) -> EventPosition {
        let s = self;
        EventPosition {
            pos_id: s.pos_id,
            pair: s.pair.clone(),
            is_closed: s.finished,
            is_short: s.is_short(),
            base_asset_size: s.base_asset_size,
            quote_asset_size: s.quote_asset_size,
            exit_high_price: s.exit_high_price,
            exit_low_price: s.exit_low_price,
            open_time: s.open_time,
            open_price: s.open_price,

            profit: s.profit,
            close_time: s.close_time as i64,
            position: Some(s.clone()),
        }
    }
}
