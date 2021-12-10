use super::*;
use crate::base::SimpleCrossEvent;
use crate::candle::{CandleConfig, CandleSeriesTA, Tick};
// use crate::online::bot::PairMeta;
// use crate::online::bot::{Actor, Bot, PairMeta};
use crate::offline_old::run::TRunner;

#[derive(Debug)]
pub struct PairMeta {
    pub pair: Pair,
    pub last_tick: Option<Tick>,
    pub mini_tick: MiniTick,
    pub ticks_arr: TimeSerVec<Tick>,
    pub candles: CandleSeriesTA,
}

impl PairMeta {
    pub fn new(p: Pair) -> PairMeta {
        Self {
            pair: p,
            last_tick: None,
            mini_tick: Default::default(),
            ticks_arr: Default::default(),
            candles: CandleSeriesTA::new_dep(&CandleConfig::default()),
        }
    }
}

// todo move to brain
impl PairMeta {
    pub(crate) fn on_price_tick(&mut self, tick: Tick, bot: &mut Actor) {
        // println!("on_price_tick {:?}", &tick);
        self.last_tick = Some(tick.clone());
        self.ticks_arr.push(tick);
        if self.ticks_arr.len() >= candle::SMALL_TICK as usize {
            self.candles.add_ticks(self.ticks_arr.clone());
            self.ticks_arr.clear();
            self.on_completed_small_candle(bot);
        }
    }

    // run when many ticks complete an small candle
    fn on_completed_small_candle(&mut self, bot: &mut Actor) {
        println!(
            "{} - {:?} - on_completed_small_candle",
            helper::time_tag_string(),
            self.pair
        );

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
        let symbol_id = self.pair.to_symbol_id();

        // bot.go_long(symbol_id, t);

        match up {
            SimpleCrossEvent::Bull(_) => {
                println!("Entering bull entery");

                // bot.go_long(symbol_id);
                // self.strategy1.buy(kid, t);
                // if macd_out.macd < 0. && price > ma && ta.vel.count >= 3 && price > big_ema {
                // if  price > ma && ta.vel.count >= 3 && price > big_ema {
                // if macd_out.macd < 0. && price > ma && ta.vel.count >= 3 {
                if price > ma && ta.vel.count >= 3 {
                    // if macd_out.macd < 0. && price > ma {
                    // if macd_out.macd < 0. && price > ma  {
                    //     if macd_out.macd < 0. && price > ma && ta.vel.count >= 3  {
                    // if macd_out.macd < 0. && price > ma {
                    // if macd_out.macd < 0. {
                    // self.strategy1.buy(kid, t, ta);
                    bot.go_long(symbol_id, t);
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
                // if macd_out.macd > 0. && price < ma && ta.vel.count >= 3 {
                if price < ma && ta.vel.count >= 3 {
                    // if macd_out.macd > 0. && price < ma {
                    // if macd_out.macd > 0. && price < ma  {
                    //     if macd_out.macd > 0. && price < ma && ta.vel.count >= 3 {
                    // if macd_out.macd > 0.  {
                    // self.strategy1.sell(kid, t, ta);
                    // self.port.sell_short(t.price as i64, 10, t.time_s);
                    // bot.go_short(symbol_id);
                    bot.go_short(symbol_id, t);
                }
            }
        }
    }

    // end of trading session (in Friday, closing market)
    fn finlise(&mut self) {}
}
