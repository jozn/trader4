use super::*;
use crate::ctrader::*;
use crate::online::assets::Pair;
use crate::pb;
use crate::pb::TickData;
use std::fs;
use std::sync::Arc;

pub fn collect_data_from_api_csv(
    cfg: &Config,
    pari: &Pair,
    from_time_ms: i64,
    to_time_ms: i64,
) -> String {
    let symbol_id = pari.to_symbol_id();
    let start_time = from_time_ms;
    let mut time_ms = to_time_ms;

    let mut collector = Collector::new();
    let mut in_bids = true;

    // Connect to cTrader server
    let (mut ct, rc_event) = CTrader::connect(cfg);
    ct.application_auth_req(&cfg.client_id, &cfg.client_secret);
    std::thread::sleep(std::time::Duration::new(2, 0));
    println!("{:?} > Got connected ", pari);
    ct.get_bid_tick_data_req(symbol_id, from_time_ms, to_time_ms);

    let mut cnt = 0;

    for e in rc_event {
        match e.clone() {
            _ => {
                // println!("EVENT: {:#?}", e);
            }
        };

        match e {
            ResponseEvent::Refresh => {
                // println!("EVENT");
            }
            ResponseEvent::ErrorRes(_) => {}
            ResponseEvent::GetTickDataRes(r) => {
                let ts = trans_ticks(&r.tick_data);
                // let first_tick = ts.first().unwrap();
                cnt += 1;
                if in_bids {
                    if ts.len() > 0 {
                        let first_tick = ts.first().unwrap();
                        println!(
                            "{:?} > Bid {} - Time: {} - Dur: {} - Tick Counts: {}",
                            pari,
                            cnt,
                            helper::to_time_string(time_ms / 1000),
                            ( time_ms - first_tick.timestamp) / 3600_000,
                            ts.len()
                        );
                        // bids
                        ts.iter().for_each(|v| collector.bids.push(v.clone()));
                    }

                    if r.has_more {
                        time_ms = ts.first().unwrap().timestamp;
                        ct.get_bid_tick_data_req(symbol_id, from_time_ms, time_ms);
                    } else {
                        in_bids = false;
                        time_ms = to_time_ms;
                        ct.get_ask_tick_data_req(symbol_id,from_time_ms, time_ms);
                    }
                } else {
                    if ts.is_empty() {
                        break;
                    }
                    let first_tick = ts.first().unwrap();
                    println!(
                        "{:?} > Ask {} - Time: {} - Dur: {} - Tick Counts: {}",
                        pari,
                        cnt,
                        helper::to_time_string(time_ms / 1000),
                        (time_ms - first_tick.timestamp) / 3600_000,
                        ts.len()
                    );
                    ts.iter().for_each(|v| collector.asks.push(v.clone()));
                    if r.has_more {
                        time_ms = first_tick.timestamp ;
                        ct.get_ask_tick_data_req(symbol_id, from_time_ms ,time_ms);
                    } else {
                        break;
                    }
                }
            }
            _ => {}
        };
    }

    // let res = collector.final_result();
    let res = collector.to_csv();
    format!("{:}", res)
}

// Notes: <pb::TickData> data is decending, means we have newer time in the first time
fn trans_ticks(arr: &Vec<pb::TickData>) -> Vec<pb::TickData> {
    // let mut arr = arr.clone();
    let first = arr.first();

    match first {
        None => {
            vec![]
        }
        Some(v) => {
            let mut res = vec![];

            let mut price = v.tick;
            let mut time = v.timestamp;
            res.push(v.clone());

            for t in arr.iter().skip(1) {
                price += t.tick;
                time += t.timestamp;

                res.push(TickData {
                    timestamp: time,
                    tick: price,
                })
            }

            // Reverse data so we have the oldest in the first of zero index
            res.reverse();
            res
        }
    }
}

#[derive(Debug)]
pub struct Collector {
    pub bids: Vec<pb::TickData>, // bids is the lower price
    pub asks: Vec<pb::TickData>,
}

impl Collector {
    pub fn new() -> Self {
        Collector {
            bids: vec![],
            asks: vec![],
        }
    }
    pub fn sort(&mut self) {
        let mut bids = self.bids.clone();
        bids.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));

        let mut asks = self.asks.clone();
        asks.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));

        self.bids = bids;
        self.asks = asks;
    }

    pub fn assert_data(&self) {
        let start = (self.asks.first().unwrap().timestamp - self.bids.first().unwrap().timestamp)
            .abs()
            < 60_000;
        let end = (self.asks.last().unwrap().timestamp - self.bids.last().unwrap().timestamp).abs()
            < 60_000;
        assert!(start);
        assert!(end);
    }

    pub fn to_csv(&mut self) -> String {
        let arr = self.final_result();
        let res_str = helper::to_csv_out(&arr, true);
        res_str
    }

    pub fn final_result(&mut self) -> Vec<BTickData> {
        self.sort();
        // self.assert_data();
        self.to_bticks()
    }

    pub fn to_bticks(&self) -> Vec<BTickData> {
        let mut arr = vec![];
        self.bids.iter().for_each(|v| {
            arr.push(TransTickData {
                timestamp: v.timestamp,
                bid_price: v.tick,
                ask_price: 0,
            })
        });
        self.asks.iter().for_each(|v| {
            arr.push(TransTickData {
                timestamp: v.timestamp,
                bid_price: 0,
                ask_price: v.tick,
            })
        });

        arr.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));

        let mut tdt_arr = vec![];
        let mut last_bid = 0;
        let mut last_ask = 0;
        for t in arr.iter() {
            if t.bid_price > 0 {
                last_bid = t.bid_price;
            }
            if t.ask_price > 0 {
                last_ask = t.ask_price;
            }

            if last_bid == 00 || last_ask == 0 {
                continue;
            }

            tdt_arr.push(TransTickData {
                timestamp: t.timestamp,
                bid_price: last_bid,
                ask_price: last_ask,
            });
        }

        // @Later: Should wer reduce ticks counts
        let mut tdt_arr2 = vec![];
        let mut last_time = 0;
        // reverse as the reverse has the correct updated ticks for both bid and asks
        tdt_arr.reverse();
        for t in tdt_arr {
            if t.timestamp == last_time {
                continue;
            }
            last_time = t.timestamp;
            tdt_arr2.push(t);
        }
        tdt_arr2.reverse();

        let mut arr_res = vec![];
        for t in tdt_arr2.iter() {
            let bt = BTickData {
                date_str: helper::to_time_string(t.timestamp / 1000),
                // timestamp: format!("{}_{:0<3}", t.timestamp / 1000, t.timestamp % 1000),
                timestamp_sec: t.timestamp / 1000,
                timestamp: t.timestamp,
                bid_price: t.bid_price as f64 / 100_000.,
                ask_price: t.ask_price as f64 / 100_000.,
            };

            arr_res.push(bt);
        }

        arr_res
    }
}

#[derive(Debug)]
struct TransTickData {
    pub timestamp: i64,
    pub bid_price: i64,
    pub ask_price: i64,
}

#[derive(Debug, serde::Serialize)]
pub struct BTickData {
    pub date_str: String,
    pub timestamp_sec: i64,
    pub timestamp: i64,
    pub bid_price: f64,
    pub ask_price: f64,
}
