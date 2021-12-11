use crate::collector::row_data::{BTickData, TransTickData};
use crate::helper;
use crate::online::pb;

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
