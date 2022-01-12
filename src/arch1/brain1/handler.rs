use super::*;
use super::*;
use crate::base::SimpleCrossEvent;
use crate::candle::{CandleConfig, CandleSeriesTA, Tick, TimeSerVec};
use crate::collector::row_data::BTickData;
use crate::configs::assets::*;
use crate::gate_api::GateWay;
use crate::{candle, helper};

#[derive(Debug)]
pub struct PairMemory {
    pub pair: Pair,
    pub last_tick: Option<Tick>,
    pub last_trade_time: u64,
    pub ticks_arr: TimeSerVec<Tick>,
    pub candles: CandleSeriesTA,
}

impl PairMemory {
    pub fn new(p: Pair, conf: &CandleConfig) -> PairMemory {
        Self {
            pair: p,
            last_tick: None,
            last_trade_time: 0,
            ticks_arr: Default::default(),
            candles: CandleSeriesTA::new(conf),
        }
    }
}

impl Brain1 {
    pub fn on_price_tick(&mut self, symbol_id: i64, tick: Tick) {
        let mut pm = self.borrow_pair_meta(symbol_id);
        // pm.on_price_tick(tick, &self);
        pm.last_tick = Some(tick.clone());
        pm.ticks_arr.push(tick);
        let small_tick_size = pm.candles.big.cfg.small_tick;
        if pm.ticks_arr.len() >= small_tick_size as usize {
            pm.candles.add_ticks(pm.ticks_arr.clone());
            pm.ticks_arr.clear();
            self.on_completed_small_candle(symbol_id);
            self.update_all_tailing_pos();
        }
    }

    // run when many ticks complete an small candle
    fn on_completed_small_candle(&mut self, symbol_id: i64) {
        // println!("{} - {:?} - small_candle", helper::time_tag_string(), pm.pair);
        let mut pair_mem = self.borrow_pair_meta(symbol_id);

        let last_tick = &pair_mem.last_tick.clone().unwrap();
        let price = last_tick.price_raw;

        let kline_holder = &pair_mem.candles.medium;

        let kt_opt = kline_holder.kline_ta_tip.clone();
        let kline_ta_opt = kline_holder.klines_ta.last().clone();

        let big_line_ta_opt = pair_mem.candles.big.klines_ta.last();
        if kline_ta_opt.is_none() {
            return;
        }
        if big_line_ta_opt.is_none() {
            return;
        }
        let kline_ta = kline_ta_opt.unwrap().to_owned();
        let big_kline_ta = big_line_ta_opt.unwrap().to_owned();
        // let big_kline = big_ema.unwrap();
        let big_ema = big_line_ta_opt.unwrap().ta1.vel2.ma;
        let kline_id = kline_ta.kline.bucket;
        let med_ma = kline_ta.ta1.vel2.ma;
        let med_macd_out = kline_ta.ta1.macd.clone();

        let up = med_macd_out.signal.0;
        let down = med_macd_out.signal.1;

        let med_ta = &kline_ta.ta1;
        let big_ta = &big_kline_ta.ta1;
        let symbol_id = pair_mem.pair.to_symbol_id();

        match up {
            SimpleCrossEvent::Bull(_) => {
                // println!("Entering bull entery");
                // if macd_out.macd < 0. && price > ma && ta.vel.count >= 3 && big_ema > ma {
                // if macd_out.macd < 0. && price > ma && ta.vel.count >= 1 && ta.vel.avg_vel_zz > 0. {

                // self.go_long(symbol_id, kline_id, last_tick, &med_ta, &big_ta);

                if big_ta.mom <= 0. && med_ta.mom <= 0. {
                    // self.go_long(symbol_id, kline_id, last_tick, &med_ta, &big_ta);

                    // println!("ma {:#?} ", big_ta);
                    // println!("ma {:#?} {}  {} ", price , big_ta.ema200 , med_ma );
                    // self.go_long(symbol_id, kline_id, last_tick, &med_ta, &big_ta);
                }

                // if big_ta.macd.macd_zz < -400. && price > med_ma {
                if big_ta.macd.macd_pip < -400. {
                    // println!("ma {:#?} ", big_ta);
                    // println!("ma {:#?} {}  {} ", price , big_ta.ema200 , med_ma );
                    // self.go_long(symbol_id, kline_id, last_tick, &med_ta, &big_ta);
                }

                // // if price > big_ta.ema200 {
                // if big_ta.vel.count > 1
                //     && big_ta.vel.avg_vel_pip > 0.
                //     && med_ta.vel.count > 1
                //     && med_ta.vel.avg_vel_pip > 0.
                // {
                //     // self.go_long(symbol_id, kline_id, last_tick, &med_ta, &big_ta);
                // }
                if price > big_ema
                // big_ta.vel.count > 1
                //      big_ta.vel.avg_vel_pip > 0.
                // && med_ta.vel.count > 1
                // && big_ta.vel.avg_vel_pip > 0.
                {
                    // self.go_long(symbol_id, kline_id, last_tick, &med_ta, &big_ta);
                    if med_macd_out.macd < 0. && price > med_ma && med_ta.vel1.count >= 1 {
                        // self.go_long(symbol_id, kline_id, last_tick, &med_ta, &big_ta);
                    }
                }
                // }
                // self.go_long(symbol_id, kline_id, last_tick, &med_ta, &big_ta);
                if med_macd_out.macd < 0. && price > med_ma && med_ta.vel1.count >= 1 {
                    self.go_long(symbol_id, kline_id, last_tick, &med_ta, &big_ta);
                }
            }
            SimpleCrossEvent::None => {}
            SimpleCrossEvent::Bear(_) => {}
        }

        match down {
            SimpleCrossEvent::Bull(_) => {}
            SimpleCrossEvent::None => {}
            SimpleCrossEvent::Bear(_) => {
                // println!("Entering bear entery");
                // if macd_out.macd > 0. && price < ma && ta.vel.count >= 3 && big_ema > ma {
                // if macd_out.macd > 0. && price < ma && ta.vel.count >= 1 && ta.vel.avg_vel_zz < 0. {

                // self.go_short(symbol_id, kline_id, last_tick, &med_ta, &big_ta);

                if big_ta.macd.macd_pip > 400. {
                    // self.go_short(symbol_id, kline_id, last_tick, &med_ta, &big_ta);
                }
                // if price < big_ta.ema200 {
                // if big_ta.vel.count > 1
                //     && big_ta.vel.avg_vel_pip < 0.
                //     && med_ta.vel.count > 1
                //     && med_ta.vel.avg_vel_pip < 0.
                // {
                //     // self.go_short(symbol_id, kline_id, last_tick, &med_ta, &big_ta);
                // }
                // }

                if med_macd_out.macd > 0. && price < med_ma && med_ta.vel1.count >= 1 {
                    // todo enable
                    // self.go_short(symbol_id, kline_id, last_tick, &med_ta, &big_ta);
                }
            }
        }
    }
}