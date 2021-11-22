use crate::base::SimpleCrossEvent;
use super::*;
use crate::candle::{CandleSeriesTA, Tick};
use crate::run::TRunner;

impl Bot1 {
    pub(crate) fn on_price_tick(&mut self, tick: Tick) {
        // println!("on_price_tick {:?}", &tick);
        self.last_tick = Some(tick.clone());
        self.ticks_arr.push(tick);
        if self.ticks_arr.len() >= 3 {
            self.candles.add_ticks(self.ticks_arr.clone());
            self.ticks_arr.clear();
            self.on_completed_small_candle();
        }
    }

    // run when many ticks complete an small candle
    fn on_completed_small_candle(&mut self) {
        println!("on_completed_small_candle");

        let t = &self.last_tick.clone().unwrap();
        let price = t.price;

        let s = &self.candles.medium;

        let kt_opt = s.kline_ta_tip.clone();
        let big_ema = self.candles.big.klines_ta.last();
        if kt_opt.is_none() {
            return;
        }
        if big_ema.is_none() {
            // return;
        }
        let kt = kt_opt.unwrap();
        // let big_kline = big_ema.unwrap();
        // let big_ema = big_kline.ta1.ema200;
        let kid = kt.kline.bucket;
        let ma = kt.ta1.ema200;
        let macd_out = kt.ta1.macd.clone();

        let up = macd_out.signal.0;
        let down = macd_out.signal.1;

        let ta = &kt.ta1;

        match up {
            SimpleCrossEvent::Bull(_) => {
                // self.go_long();
                // self.strategy1.buy(kid, t);
                // if macd_out.macd < 0. && price > ma && ta.vel.count >= 3 && price > big_ema {
                // if  price > ma && ta.vel.count >= 3 && price > big_ema {
                if macd_out.macd < 0. && price > ma && ta.vel.count >= 3 {
                    // if macd_out.macd < 0. && price > ma  {
                    //     if macd_out.macd < 0. && price > ma && ta.vel.count >= 3  {
                    // if macd_out.macd < 0. && price > ma {
                    // if macd_out.macd < 0. {
                    // self.strategy1.buy(kid, t, ta);
                    self.go_long();
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
                // self.go_short();
                // self.strategy1.sell(kid, t);
                // if macd_out.macd > 0. && price < ma {
                // if macd_out.macd > 0. && price < ma && ta.vel.count >= 3 && price < big_ema {
                // if  price < ma && ta.vel.count >= 3 && price < big_ema {
                if macd_out.macd > 0. && price < ma && ta.vel.count >= 3 {
                    // if macd_out.macd > 0. && price < ma  {
                    //     if macd_out.macd > 0. && price < ma && ta.vel.count >= 3 {
                    // if macd_out.macd > 0.  {
                    // self.strategy1.sell(kid, t, ta);
                    // self.port.sell_short(t.price as i64, 10, t.time_s);
                    self.go_short();
                }
            }
        }

    }

    // end of trading session (in Friday, closing market)
    fn finlise(&mut self) {}
}

/*
// old - delete
impl TRunner for Bot1 {
    fn get_next_tick(&mut self) -> Option<Tick> {
        todo!()
    }

    fn on_next_tick_bulk(&mut self, cst: &CandleSeriesTA) {
        todo!()
    }

    fn on_price_tick(&mut self, cst: &CandleSeriesTA, tikc: &Tick) {
        todo!()
    }

    fn on_exit(&mut self, cst: &CandleSeriesTA) {
        todo!()
    }
}*/
