use crate::candle::{CandleSeriesTA, Tick};
use crate::helper;

pub fn dc_strategy(candles: &CandleSeriesTA, last_tick: Tick) -> f64 {
    if candles.big.klines_ta.is_empty() {
        return 0.;
    }
    let rnd = helper::get_rand(100);
    // println!("rnd {} - {}", rnd, self.candles.small.klines_ta.len());
    if rnd < 90 {
        // return;
    }
    let med = candles.medium.klines_ta.last().unwrap();
    let vec_med = candles.medium.klines_ta.get_vec();
    let med_pre = vec_med.get(vec_med.len() - 2).unwrap();
    let med_tip = candles.medium.kline_ta_tip.clone().unwrap();
    let big = candles.big.klines_ta.last().unwrap();
    let big_tip = candles.big.kline_ta_tip.clone().unwrap();

    let price = last_tick.ask_price;

    let big_ta = &big.ta1;
    let med_ta = &med.ta1;
    let ema200 = med_ta.vel2.ma;

    let now_dc = &med_ta.ta2.dc;
    let pre_dc = &med_pre.ta1.ta2.dc;
    // let now_dc = &med_tip.ta1.ta2.dc;

    if now_dc.high > pre_dc.high && price > ema200 {
        1.
    } else {
        0.
    }
}
