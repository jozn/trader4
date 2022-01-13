use super::*;
use crate::base::SignalsRes;
use crate::candle;
use crate::candle::{CandleConfig, CandleSeriesTA, Tick, TimeSerVec, TA1};
use crate::configs::assets;
use crate::configs::assets::*;
use crate::gate_api::{GateWay, NewPos, PosRes, UpdatePos};
use crate::ta::Vel;
use std::borrow::BorrowMut;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

impl Brain4 {
    pub fn update_all_tailing_pos(&mut self) {}

    pub fn update_all_tailing_pos2(&mut self) {
        // simple tailing
        // for (_, ph) in self.open_pos.clone() {
        for (_, ph) in self.open_pos.iter() {
            let pm = self.read_pair_meta(ph.pos_res.symbol_id);
            let tick = pm.last_tick.clone().unwrap();
            let p = &ph.pos_res;
            let new_sl = cal_price(tick.price_raw, -4.5);
            if p.low_exit_price < new_sl {
                // println!(">>> update new sl {} -- {} {:?}", p.pos_id, new_sl,p);

                let up = UpdatePos {
                    pos_id: p.pos_id,
                    close: false,
                    take_profit_price: 0.,
                    stop_loose_price: new_sl,
                    at_price: tick.price_raw,
                    time_s: self.last_trade_time,
                };
                self.con.update_position(&up);
            }
        }
    }
}
