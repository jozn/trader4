use super::*;
use crate::base::CrossRes;
use crate::candle;
use crate::candle::{CandleConfig, CandleSeriesTA, Tick, TimeSerVec, TA1};
use crate::configs::assets;
use crate::configs::assets::*;
use crate::gate_api::*;
use crate::ta::Vel;
use std::borrow::BorrowMut;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

impl Brain6 {
    pub fn update_all_tailing_pos(&mut self) {}

    pub fn update_all_tailing_pos2(&mut self) {
        // simple tailing
        for (_, ph) in self.open_pos.iter() {
            let pm = self.read_pair_meta(ph.pos_res.pair.to_symbol_id());
            let tick = pm.last_tick.clone().unwrap();
            let pair = &ph.pos_res.pair;
            let p = &ph.pos_res;
            // let new_sl = cal_price(tick.price_raw, -4.5);
            let new_sl = pair.cal_price(tick.get_price(), -5.5);
            if p.exit_low_price < new_sl {
                // println!(">>> update new sl {} -- {} {:?}", p.pos_id, new_sl,p);
                let up = UpdatePos {
                    pos_id: p.pos_id,
                    close: false,
                    exit_high_price: 0.,
                    exit_low_price: new_sl,
                    at_price: tick.get_price(), // todo
                    time_s: self.last_trade_time,
                };
                self.con.update_position(&up);
            }
        }
    }
}
