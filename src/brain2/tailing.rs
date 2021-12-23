use super::*;
use crate::base::SignalsRes;
use crate::candle;
use crate::candle::{CandleConfig, CandleSeriesTA, Tick, TimeSerVec, TA1};
use crate::configs::assets;
use crate::configs::assets::*;
use crate::gate_api::{GateWay, NewPos, PosRes, UpdatePos};
use crate::offline::num5;
use std::borrow::BorrowMut;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

impl Brain2 {
    pub fn update_all_tailing_pos(&mut self) {
        for (_, p) in self.open.clone() {
            // self.on_full_tailing_pl(p);
            // todo enalbe
            self.on_tailing(p);
        }
    }

    pub fn on_tailing(&mut self, pos: PosRes) {
        // println!("Notify {:?}", pos);
        // if pos.is_closed || true {
        if pos.is_closed {
            return;
        }

        let tick = self.last_tick.clone().unwrap();
        let price = tick.price_raw;
        let ta = self.candles.big.klines_ta.last().unwrap();
        let atr = ta.ta1.atr;

        if pos.is_short {
        } else {
            // if we alredy in proift dispostin go on tailing
            if pos.low_exit_price >= pos.open_price {
                self.on_full_tailing_pl(pos);
                return;
            }

            let half_distance = (pos.high_exit_price - pos.open_price) / 2.;
            let half = pos.open_price + (pos.high_exit_price - pos.open_price) / 2.;
            // if we have travedl half of profit set new stop lose to not this trade be a looser
            if price > half && pos.low_exit_price < pos.open_price {
                let mut final_low = pos.open_price + half_distance * 0.1; // 0.1 for equality and fees
                let mut final_hihg = pos.open_price + 4. * half_distance; // 2

                let up = UpdatePos {
                    pos_id: pos.pos_id,
                    close: false,
                    take_profit_price: rond5(final_hihg),
                    stop_loose_price: rond5(final_low),
                    at_price: tick.price_raw,
                    time_s: tick.time_s,
                    // ta_med: Default::default(),
                    // ta_big: Default::default()
                };

                self.con.update_position(&up);
            }
        }
    }

    pub fn on_full_tailing_pl(&mut self, pos: PosRes) {
        // println!("Notify {:?}", pos);
        // if pos.is_closed || true {
        if pos.is_closed {
            return;
        }

        // let old_ops = self.open.get(&pos.pos_id);
        let tick = self.last_tick.clone().unwrap();
        let ta = self.candles.big.klines_ta.last().unwrap();
        let atr = ta.ta1.atr;

        if pos.is_short {
        } else {
            let new_stop = tick.price_raw - atr / 2.;
            let new_profit = tick.price_raw + atr / 2.;

            let mut changed = false;
            let mut final_low = pos.low_exit_price;
            let mut final_hihg = pos.high_exit_price;
            if pos.low_exit_price < new_stop {
                final_low = new_stop;
                changed = true;
            }

            if pos.high_exit_price < new_profit {
                let atr_profit = pos.open_price + atr * 1.5;
                if atr_profit > new_profit {
                    final_hihg = atr_profit;
                } else {
                    final_hihg = new_profit;
                }
                changed = true;
            }

            if changed {
                let up = UpdatePos {
                    pos_id: pos.pos_id,
                    close: false,
                    take_profit_price: rond5(final_hihg),
                    stop_loose_price: rond5(final_low),
                    at_price: tick.price_raw,
                    time_s: tick.time_s,
                    // ta_med: Default::default(),
                    // ta_big: Default::default()
                };

                self.con.update_position(&up);
            }
        }
    }
}
