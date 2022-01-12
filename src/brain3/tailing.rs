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
use crate::ta::Vel;

impl Brain3 {
    pub fn update_all_tailing_pos3(&mut self) {}
    pub fn update_all_tailing_pos(&mut self) { // simple tailing
        // for (_, ph) in self.open_pos.clone() {
        for (_, ph) in self.open_pos.iter_mut() {
            let tick = self.last_tick.clone().unwrap();
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

    pub fn update_all_tailing_pos2(&mut self) {
        // for (_, ph) in self.open_pos.clone() {
        for (_, ph) in self.open_pos.iter_mut() {
            let tick = self.last_tick.clone().unwrap();
            let p = &ph.pos_res;
            let tail_param = NewTailParam {
                now_price: tick.price_raw,
                open_price: p.open_price,
                profit_price: p.high_exit_price,
                stop_price: p.low_exit_price,
                profit_level: ph.profit_level
            };

            let new_tail = cal_new_tail_long2(&tail_param);
            if new_tail.update {
                ph.profit_level = new_tail.profit_level;
                println!(">>> {} -- \n {:?}", p.pos_id, new_tail);
            }
            if new_tail.close {
                let up = UpdatePos {
                    pos_id: p.pos_id,
                    close: true,
                    take_profit_price: 0.0,
                    stop_loose_price: 0.0,
                    at_price: tick.price_raw,
                    time_s: self.last_trade_time,
                };
                // self.con.update_position(&up);
            } else if new_tail.update {
                let up = UpdatePos {
                    pos_id: p.pos_id,
                    close: false,
                    take_profit_price: new_tail.high_price,
                    // take_profit_price: 0.,
                    stop_loose_price: new_tail.low_price,
                    at_price: tick.price_raw,
                    time_s: self.last_trade_time,
                };
                // println!("*****----diff-- {}-- {:?}", p.pos_id, new_tail);
                self.con.update_position(&up);
            }
        }
    }

   /*
   pub fn update_all_tailing_pos_bk(&mut self) {
        for (_, p) in self.open_pos.clone() {
            let tick = self.last_tick.clone().unwrap();
            // self.on_full_tailing_pl(p);
            // todo enalbe
            // self.on_tailing(p);
            // println!("-------- {:?}", p);
            let low_sl = cal_price(p.open_price, -2.2);
            let level1_profit = cal_price(p.open_price, 2.2);
            let level2_profit = cal_price(p.open_price, 4.9);
            let level3_profit = cal_price(level2_profit, 4.7);

            let price = tick.price_raw;
            let time = self.last_trade_time;
            if tick.price_raw < low_sl {
                let up = UpdatePos {
                    pos_id: p.pos_id,
                    close: true,
                    take_profit_price: 0.0,
                    stop_loose_price: 0.0,
                    at_price: tick.price_raw,
                    time_s: self.last_trade_time,
                };
                self.con.update_position(&up);
            }

            if price > p.open_price {
                let dif_pip = (price - p.open_price) * 10_000.;
                println!("----diff-- {}-- {:?}", p.pos_id, dif_pip);
                if dif_pip < 2.2 {
                    // nothing
                } else if dif_pip < 3.9 {
                    let up = UpdatePos {
                        pos_id: p.pos_id,
                        close: false,
                        take_profit_price: cal_price(p.open_price, 7.),
                        // take_profit_price: 0.,
                        stop_loose_price: p.open_price,
                        at_price: price,
                        time_s: time,
                    };
                    println!("*****----diff-- {}-- {:?}", p.pos_id, dif_pip);
                    self.con.update_position(&up);
                } else if dif_pip < 4.5 {
                    let up = UpdatePos {
                        pos_id: p.pos_id,
                        close: false,
                        take_profit_price: cal_price(p.open_price, 8.),
                        stop_loose_price: cal_price(p.open_price, -2.3),
                        at_price: price,
                        time_s: time,
                    };
                    self.con.update_position(&up);
                } else if dif_pip < 7. {
                    let up = UpdatePos {
                        pos_id: p.pos_id,
                        close: false,
                        take_profit_price: level3_profit,
                        stop_loose_price: level1_profit,
                        at_price: price,
                        time_s: time,
                    };
                    self.con.update_position(&up);
                } else if dif_pip < 11. {
                    let up = UpdatePos {
                        pos_id: p.pos_id,
                        close: false,
                        take_profit_price: cal_price(p.open_price, 12.),
                        stop_loose_price: cal_price(p.open_price, 6.5),
                        at_price: price,
                        time_s: time,
                    };
                    self.con.update_position(&up);
                } else {
                    let new_profit = cal_price(price, 3.);
                    let stop_lose = cal_price(price, -4.);
                    if p.low_exit_price < stop_lose {
                        let up = UpdatePos {
                            pos_id: p.pos_id,
                            close: false,
                            take_profit_price: new_profit,
                            stop_loose_price: stop_lose,
                            at_price: price,
                            time_s: time,
                        };
                        self.con.update_position(&up);
                    }
                }
            }
        }
    }
    */
}

#[derive(Debug)]
struct NewTailRes {
    close: bool,
    update: bool,
    high_price: f64,
    low_price: f64,
    profit_level: i32,
}

#[derive(Debug)]
struct NewTailParam {
    now_price: f64,
    open_price: f64,
    profit_price: f64,
    stop_price: f64,
    profit_level: i32,
}
type PipPoint = (f64,f64,f64);

// Note: this function return the least of level, so if we already has updated the level
// to a higher number we should ignore the result of this func.
// todo: add level param support
fn get_level_point(price_diff_pip: f64, profit_level:i32, points: &Vec<PipPoint>) -> (PipPoint,i32) {
    let mut res = points.get(0).unwrap();
    let mut gid = 0;
    let pdp = price_diff_pip;
    assert!(pdp > res.0); // we should not be closing postion - we should ckeck befoer call this fn

    for (id,l) in points.iter().enumerate() {
        let id = id as i32;
        if id < profit_level {
            continue;
        }
        gid = id;
        if pdp > l.0 && pdp < l.1 {
            return (l.clone(), id);
        }
        if pdp > l.1 && pdp < l.2 {
            return (l.clone(), id);
        }
    }

    (res.clone(), gid)
}

fn cal_new_tail_long2(p: &NewTailParam) -> NewTailRes {
    let levels = vec![
        (-2.5, 2.5, 4.5),
        (-1.5, 3.5, 5.5),
        (0.5, 4.5, 7.5),
        (1.5, 5.5, 8.5),
        (2.5, 6.5, 8.5),
        (3.5, 7.5, 9.5),
        (4.5, 8.0, 10.5),
        (5.5, 9.0, 11.5),
        (6.5, 9.5, 11.5),
        (7.5, 10., 12.5),
        (8.0, 11., 13.5),
        (8.5, 11.5, 14.5),
        (10.5, 12.0, 14.5),
    ];
    let levels2 = vec![
        (-2.5, 2.5, 4.5),
        (-1.5, 3.5, 5.5),
        (0.5, 4.5, 7.5),
        (1.5, 5.5, 8.5),
    ];

    let levels = vec![
        (-2.5, 2.5, 4.5),
        // (-1.5, 3.5, 5.5),
        (0.5, 4.5, 7.5),
        (1.5, 5.5, 8.5),
    ];

    let levels = vec![
        (-4.5, 2.5, 4.5),
        // (-1.5, 3.5, 5.5),
        (0.5, 4.5, 7.5),
        (1.5, 5.5, 8.5),
    ];

    if p.now_price < p.stop_price {
        return NewTailRes {
            close: true,
            update: true,
            high_price: p.profit_price, // not used
            low_price: p.stop_price,    // not used
            profit_level: p.profit_level // not used
        };
    }

    let price_diff_pip = (p.now_price - p.open_price ) * 10_000.;
    let (point,level) = get_level_point(price_diff_pip,p.profit_level,&levels);

    if p.profit_level >= level {
        return NewTailRes {
            close: false,
            update: false,
            high_price: p.profit_price, // not used
            low_price: p.stop_price,    // not used
            profit_level: p.profit_level // not used
        };
    }

    let stop_bound = cal_price(p.open_price, point.0);
    let next_bound = cal_price(p.open_price, point.1);
    let profit_bound = cal_price(p.open_price, point.2);

    NewTailRes {
        close: false,
        update: true,
        high_price: stop_bound,
        low_price: profit_bound,
        profit_level: level,
    }
}
/*
fn cal_new_tail_long(p: &NewTailParam) -> NewTailRes {
    const WIDE: f64 = 2.2;
    // let wide_10pip = (WIDE * 10.) as i64;
    let wide_10pip = WIDE;
    let now_wide_10pip = (p.now_price - p.open_price) * 10_000.;
    let is_in_profit = p.open_price >= p.stop_price;

    let mut close = false;
    let mut upate = false;

    let multi = (now_wide_10pip * 10.) as i64 / (wide_10pip * 10.) as i64;
    println!(">>>>>>>>>>>>>>> {} --  {:?}", multi, now_wide_10pip);
    // let  multi= (multi * 10.) as i64;
    match multi {
        i64::MIN..=-1 => {
            assert!(p.now_price <= p.stop_price);
            NewTailRes {
                close: true,
                update: true,
                high_price: p.profit_price, // not used
                low_price: p.stop_price,    // not used
            }
        }
        0 => {
            NewTailRes {
                close: false,
                update: false,
                high_price: p.profit_price, // not used
                low_price: p.stop_price,    // not used
            }
        }
        1..=i64::MAX => {
            assert!(is_in_profit);
            let wide_mul = multi as f64 * WIDE;
            let multi_f64 = multi as f64;
            // Note: if we price are above first wide stop loss will be opeing price.
            let new_stop = cal_price(p.open_price, (multi_f64 - 1.) * WIDE);
            let new_profit = cal_price(p.open_price, (multi_f64 + 2.) * WIDE);

            if p.stop_price != new_stop || p.profit_price != new_profit {
                NewTailRes {
                    close: false,
                    update: true,
                    high_price: new_profit,
                    low_price: new_stop,
                }
            } else {
                NewTailRes {
                    close: false,
                    update: false,
                    high_price: new_profit, // not used
                    low_price: new_stop,    // not used
                }
            }
        }
    }
}
*/
fn cal_new_tail_long2_bk(p: &NewTailParam) -> NewTailRes {
    let levels = vec![
        (-2.5, 2.5, 4.5),
        (-1.5, 3.5, 5.5),
        (0.5, 4.5, 7.5),
        (1.5, 5.5, 8.5),
        (2.5, 6.5, 8.5),
        (3.5, 7.5, 9.5),
        (4.5, 8.0, 10.5),
        (5.5, 9.0, 11.5),
        (6.5, 9.5, 11.5),
        (7.5, 10., 12.5),
        (8.0, 11., 13.5),
        (8.5, 11.5, 14.5),
        (10.5, 12.0, 14.5),
    ];

    let now_price = p.now_price;
    let mut flag = false;
    let mut profit_price = p.profit_price;
    let mut stop_price = p.stop_price;
    let mut gid = 0;

    if p.now_price < p.stop_price {
        return NewTailRes {
            close: true,
            update: true,
            high_price: p.profit_price, // not used
            low_price: p.stop_price,    // not used
            profit_level: 0
        };
    }

    for (id,l) in levels.iter().enumerate() {
        assert!(l.0 < l.1);
        assert!(l.1 < l.2);
        let id = id as i32;
        let stop_bound = cal_price(p.open_price, l.0);
        let next_bound = cal_price(p.open_price, l.1);
        let profit_bound = cal_price(p.open_price, l.2);
        if  p.profit_level > id {
            println!("skip...");
            continue; // skip those
        }
        gid = id;

        // if id != 0 {
            println!("{} s:{} n:{} {:?}", id,stop_bound,next_bound, p);
        // }

        if now_price > stop_bound && now_price < next_bound {
            // price now is this bound break - No change in stop loose or profits
            return NewTailRes {
                close: false,
                update: false,
                high_price: p.profit_price, // not used
                low_price: p.stop_price,    // not used
                profit_level: id
            };
        }

        if now_price > stop_bound && now_price > next_bound {
            flag = true;
            stop_price = stop_bound;
            profit_price = profit_bound
        }

        // Explicit code: this code could be mreged with (A) but it's explicit.
        //
        if flag && now_price > stop_bound && now_price < next_bound {
            return NewTailRes {
                close: false,
                update: true,
                high_price: profit_price,
                low_price: stop_price,
                profit_level: id
            };
        }
    }

    // For the last level
    NewTailRes {
        close: false,
        update: flag,
        high_price: profit_price,
        low_price: stop_price,
        profit_level: gid
    }
}

pub fn price_eq(price1: f64, price2: f64, fraction: u32) -> bool {
    let mul = (10.0_f64).powf(fraction as f64);
    let p1 = (price1 * mul) as u64;
    let p2 = (price2 * mul) as u64;
    p1 == p2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next() {
        let levels = vec![
            (-2.5, 2.5, 4.5),
            (-1.5, 3.5, 5.5),
            (0.5, 4.5, 7.5),
            (1.5, 5.5, 8.5),
        ];
        let level1 = (-2.5, 2.5, 4.5);
        let level2 = (-1.5, 3.5, 5.5);
        let level3 = (0.5, 4.5, 7.5);
        let level4 = (1.5, 5.5, 8.5);

        // (price diff, PipPoint)
        let arr = vec![
            (-2. ,level1, 0),
            (-1. ,level1, 0),
            (1. ,level1, 0),
            (2.3 ,level1, 0),
            (2.7 ,level1, 1),
            (3.7 ,level1, 2),
        ];

        for t in arr {
            let res = get_level_point(t.0, &levels);
            assert_eq!(res.0,t.1);
            assert_eq!(res.1,t.2);
        }

    }

    #[test]
    fn test_default() {

    }
}
