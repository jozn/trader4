use super::*;
use super::*;
use crate::base::SimpleCrossEvent;
use crate::candle::{CandleConfig, CandleSeriesTA, Tick, TimeSerVec};
use crate::collector::row_data::BTickData;
use crate::configs::assets::*;
use crate::gate_api::GateWay;
use crate::{candle, helper};

impl Brain2 {
    pub fn on_price_tick(&mut self, symbol_id: i64, tick: Tick) {
        self.last_tick = Some(tick.clone());
        self.ticks_arr.push(tick);
        let small_tick_size = self.candles_dep.big.cfg.small_tick;
        if self.ticks_arr.len() >= small_tick_size as usize {
            self.candles_dep.add_ticks(self.ticks_arr.clone());
            self.ticks_arr.clear();
            // self.on_completed_small_candle(symbol_id);
            self.on_completed_small_candle_just_trend(symbol_id);
            self.update_all_tailing_pos();
        }
    }

    fn on_completed_small_candle(&mut self, symbol_id: i64) {
        if self.candles_dep.big.klines_ta.is_empty() {
            return;
        }
        let rnd = helper::get_rand(100);
        // println!("rnd {} - {}", rnd, self.candles.small.klines_ta.len());
        if rnd < 90 {
            // return;
        }
        let med = self.candles_dep.medium.klines_ta.last().unwrap().to_owned();
        let big = self.candles_dep.big.klines_ta.last().unwrap().to_owned();
        let last_tik = self.last_tick.clone().unwrap();
        let price = last_tik.ask_price;

        let big_ta = &big.ta1;
        let med_ta = &med.ta1;
        let ema200 = med_ta.vel2.ma;

        let r = dc_strategy(&self.candles_dep, self.last_tick.clone().unwrap());
        let last_tik = self.last_tick.clone().unwrap();

        if r > 0. || true {
            self.go_long(
                symbol_id,
                med.kline.open_time,
                &last_tik,
                &med.ta1,
                &big.ta1,
            );
        }
    }

    fn on_completed_small_candle_just_trend(&mut self, symbol_id: i64) {
        if self.candles_dep.big.klines_ta.is_empty() {
            return;
        }
        let rnd = helper::get_rand(100);
        // println!("rnd {} - {}", rnd, self.candles.small.klines_ta.len());
        if rnd < 90 {
            // return;
        }
        let med = self.candles_dep.medium.klines_ta.last().unwrap().to_owned();
        let big = self.candles_dep.big.klines_ta.last().unwrap().to_owned();
        let last_tik = self.last_tick.clone().unwrap();
        let price = last_tik.ask_price;

        let big_ta = &big.ta1;
        let med_ta = &med.ta1;
        let ema200 = med_ta.vel2.ma;

        let r = dc_strategy(&self.candles_dep, self.last_tick.clone().unwrap());
        let last_tik = self.last_tick.clone().unwrap();

        if med_ta.vel1.avg_vel_pip < 0. && med_ta.vel1.count > 5 {
            self.go_long(
                symbol_id,
                med.kline.open_time,
                &last_tik,
                &med.ta1,
                &big.ta1,
            );
        }
    }
    // run when many ticks complete an small candle
    fn on_completed_small_candle_bk_random(&mut self, symbol_id: i64) {
        if self.candles_dep.big.klines_ta.is_empty() {
            return;
        }
        let rnd = helper::get_rand(100);
        // println!("rnd {} - {}", rnd, self.candles.small.klines_ta.len());
        if rnd < 90 {
            // return;
        }
        let med = self.candles_dep.medium.klines_ta.last().unwrap().to_owned();
        let big = self.candles_dep.big.klines_ta.last().unwrap().to_owned();
        let last_tik = self.last_tick.clone().unwrap();
        let price = last_tik.ask_price;

        let big_ta = &big.ta1;
        let med_ta = &med.ta1;
        let ema200 = med_ta.vel2.ma;

        // if med_ta.vel2.avg_vel_pip > 0. || true {
        if med_ta.macd.macd_pip < -2. && price < ema200 {
            // println!("> p ema -  {} {}", price , ema200);
            self.go_long(
                symbol_id,
                med.kline.open_time,
                &last_tik,
                &med.ta1,
                &big.ta1,
            );
            // self.go_short(symbol_id, big.kline.open_time, &last_tik, &med.ta1, &big.ta1);
        }
    }
}
