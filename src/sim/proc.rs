use super::*;
use crate::base::SimpleCrossEvent;

pub fn proc_tick_buy(cst: &CandleSeriesTA, port: &mut Portfolio) {
    let tip = cst.big.kline_ta_tip.clone();
    // let tip = cst.big.klines_ta.last().clone();
    if tip.is_none() {
        return;
    }
    let tip = tip.unwrap();
    let kt_tip = tip;
    let m = kt_tip.ta1.macd.clone();
    // let m = kt_tip.ta1.fisher.clone();
    let k = &kt_tip.kline;
    let buy_s = m.signal.0;
    let sell_s = m.signal.1;

    // Buy go Long
    match buy_s {
        SimpleCrossEvent::Bull(v) => {
            println!(" >>>>>>>>>>>>>>>>>>>>> fisher: {:?}", v);
            /*if tip.ta1.fisher.fisher > -2.5 {
                // println!(" >>>>>>>>>>>>>>>>>>>>> skping fisher: {:?}", m);
                // return
            }*/
            let min = port.free_usd.min(1000.);
            if min < 15. {
                return;
            }
            port.buy_long(k.close as i64, 1 as i64, k.close_time);
        }
        SimpleCrossEvent::None => {}
        SimpleCrossEvent::Bear(_) => {
            panic!("not shoul be in here")
        }
    }

    // Sell for Long
    match sell_s {
        SimpleCrossEvent::Bull(v) => {
            panic!("not shoul be in here");
        }
        SimpleCrossEvent::None => {}
        SimpleCrossEvent::Bear(v) => {
            println!(" --------------------- sell bear: {:?}", m);
        }
    }
}

/*
pub fn proc_tick_sell(cst: &CandleSeriesTA, port: &mut Portfolio) {
    // let m = f.ta1.macd.clone();
    let tip = cst.medium.kline_ta_tip.clone();
    if tip.is_none() {
        return;
    }
    let tip = tip.unwrap();
    let kt_tip = tip;
    let m = kt_tip.ta1.macd.clone();
    let k = &kt_tip.kline;
    let buy_s = m.signal.0;
    let sell_s = m.signal.1;

    let opens = port.opens.clone();

    let current_price = k.close;

    // jsut long now
    for mut p in &opens {
        if p.finished {
            continue;
        }
        // let l = p.long.unwrap();
        // Sell at profit
        if p.open_price * (1. + p.to_exit_per / 100.) <= current_price {
            // sell
            println!("sell profit {:?} ", p.pos_id);
            // port.sell_long(k.close, p.pos_id, k.close_time);
            port.close_pos(k.close, p.pos_id, k.close_time);
        }
        // Sell at loose
        else if p.open_price * (1. - p.to_stop_loss_per / 100.) > current_price {
            println!("sell loose {:?} ", p.pos_id);
            // port.sell_long(k.close, p.pos_id, k.close_time);
            port.close_pos(k.close, p.pos_id, k.close_time);
        }
    }

    // Sell for Long
    match sell_s {
        SimpleCrossEvent::Bull(v) => {
            panic!("not shoul be in here");
        }
        SimpleCrossEvent::None => {}
        SimpleCrossEvent::Bear(v) => {}
    }
}
*/
/*
//// random ////////////
pub fn proc_tick_buy_random(cst: &CandleSeriesTA, port: &mut Portfolio) {
    let tip = cst.medium.klines_ta.last().clone();
    let tip = cst.medium.kline_ta_tip.clone();
    let big = cst.medium.kline_ta_tip.clone().unwrap();
    if tip.is_none() {
        return;
    }
    let tip = tip.unwrap();
    let kt_tip = tip;
    let m = kt_tip.ta1.macd.clone();
    let fish = kt_tip.ta1.fisher.clone();
    let k = &kt_tip.kline;
    let buy_s = m.signal.0;
    let sell_s = m.signal.1;

    let sma50 = kt_tip.ta1.sma50;

    let min = port.free_usd.min(20.);
    if min < 15. {
        return;
    }

    if k.close > kt_tip.ta1.sma50 {
        // go long
        port.buy_long(k.close, min, k.close_time);
    } else {
        let coin = min / k.close;
        //port.sell_short(k.close, coin, k.close_time);
    }

    // Ranodm buy
    port.buy_long(k.close, min, k.close_time);
}
*/
