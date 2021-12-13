use super::*;
use crate::base::SimpleCrossEvent;
use crate::candle::{CandleConfig, CandleSeriesTA, Tick, TimeSerVec};
use crate::collector::row_data::BTickData;
use crate::{candle, helper};
// use crate::online::bot::PairMeta;
// use crate::online::bot::{Actor, Bot, PairMeta};
// use crate::offline_old::run::TRunner;
// use crate::online::Pair;
use super::*;
use crate::configs::assets::*;
use crate::gate_api::GateWay;
// use crate::online::MiniTick;

#[derive(Debug)]
pub struct PairMeta {
    pub pair: Pair,
    pub last_tick: Option<Tick>,
    // pub mini_tick: MiniTick,
    pub ticks_arr: TimeSerVec<Tick>,
    pub candles: CandleSeriesTA,
}

impl PairMeta {
    pub fn new(p: Pair) -> PairMeta {
        Self {
            pair: p,
            last_tick: None,
            // mini_tick: Default::default(),
            ticks_arr: Default::default(),
            candles: CandleSeriesTA::new_dep(&CandleConfig::default()),
        }
    }
}

impl Brain {
    pub fn on_price_tick_dep(&mut self, symbol_id: i64, tick: Tick) {
        let mut pm = self.borrow_pair_meta(symbol_id);
        // pm.on_price_tick(tick, &self);
        pm.last_tick = Some(tick.clone());
        pm.ticks_arr.push(tick);
        if pm.ticks_arr.len() >= candle::SMALL_TICK as usize {
            pm.candles.add_ticks(pm.ticks_arr.clone());
            pm.ticks_arr.clear();
            self.on_completed_small_candle(symbol_id);
        }
    }

    // run when many ticks complete an small candle
    fn on_completed_small_candle(&mut self, symbol_id: i64) {
        let mut pm = self.borrow_pair_meta(symbol_id);

        // println!("{} - {:?} - small_candle", helper::time_tag_string(), pm.pair);

        let t = &pm.last_tick.clone().unwrap();
        let price = t.price;

        let s = &pm.candles.medium;

        let kt_opt = s.kline_ta_tip.clone();
        let big_ema = pm.candles.big.klines_ta.last();
        if kt_opt.is_none() {
            return;
        }
        if big_ema.is_none() {
            return;
        }
        let kt = kt_opt.unwrap();
        // let big_kline = big_ema.unwrap();
        let big_ema = big_ema.unwrap().ta1.ema200;
        let kid = kt.kline.bucket;
        let ma = kt.ta1.ema200;
        let macd_out = kt.ta1.macd.clone();

        let up = macd_out.signal.0;
        let down = macd_out.signal.1;

        let ta = &kt.ta1;
        let symbol_id = pm.pair.to_symbol_id();

        // bot.go_long(symbol_id, t);

        match up {
            SimpleCrossEvent::Bull(_) => {
                println!("Entering bull entery");

                // bot.go_long(symbol_id);
                // self.strategy1.buy(kid, t);
                // if macd_out.macd < 0. && price > ma && ta.vel.count >= 3 && price > big_ema {
                // if  price > ma && ta.vel.count >= 3 && price > big_ema {
                if macd_out.macd < 0. && price > ma && ta.vel.count >= 3 && big_ema > ma {
                    // if macd_out.macd < 0. && price > ma && ta.vel.count >= 3 {
                    // if price > ma && ta.vel.count >= 3 {
                    // if macd_out.macd < 0. && price > ma {
                    // if macd_out.macd < 0. && price > ma  {
                    //     if macd_out.macd < 0. && price > ma && ta.vel.count >= 3  {
                    // if macd_out.macd < 0. && price > ma {
                    // if macd_out.macd < 0. {
                    // self.strategy1.buy(kid, t, ta);
                    self.go_long(symbol_id, kid, t);
                    // println!("long {} - {} - {:#?}", price, kt.kline.bucket, &macd_out);
                }
            }
            SimpleCrossEvent::None => {}
            SimpleCrossEvent::Bear(_) => {}
        }

        match down {
            SimpleCrossEvent::Bull(_) => {}
            SimpleCrossEvent::None => {}
            SimpleCrossEvent::Bear(_) => {
                println!("Entering bear entery");
                // bot.go_short(symbol_id);
                // self.strategy1.sell(kid, t);
                // if macd_out.macd > 0. && price < ma {
                // if macd_out.macd > 0. && price < ma && ta.vel.count >= 3 && price < big_ema {
                // if  price < ma && ta.vel.count >= 3 && price < big_ema {
                if macd_out.macd > 0. && price < ma && ta.vel.count >= 3 && big_ema > ma {
                    // if macd_out.macd > 0. && price < ma && ta.vel.count >= 3 {
                    // if price < ma && ta.vel.count >= 3 {
                    // if macd_out.macd > 0. && price < ma {
                    // if macd_out.macd > 0. && price < ma  {
                    //     if macd_out.macd > 0. && price < ma && ta.vel.count >= 3 {
                    // if macd_out.macd > 0.  {
                    // self.strategy1.sell(kid, t, ta);
                    // self.port.sell_short(t.price as i64, 10, t.time_s);
                    // bot.go_short(symbol_id);
                    self.go_short(symbol_id, kid, t);
                }
            }
        }
    }
}
