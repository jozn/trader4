use super::*;
use crate::ctrader::*;
use crate::pb;
use crate::pb::TickData;
use std::fs;

pub fn get_ticks() {
    let cfg = Config {
        host: "demo.ctraderapi.com".to_string(),
        port: 5035,
        client_id: "3042_mso8gOm4NPAzIYizUC0gp941QCGvnXcRPJzTrNjVZNG0EeRFYT".to_string(),
        client_secret: "geDkrRiRyfbanU6OUwZMXKIjr4vKQyfs1Ete0unffXtS8Ah14o".to_string(),
        client_token: "l4jT24BWu3etFSEVViQKu1NsGpBYf2nKN0DyUGgqjy0".to_string(),
        ctid: 22851452,
    };
    let (mut cti, rc_event) = CTrader::connect(&cfg);
    // let mut ct = cti.lock().unwrap();
    let mut ct = cti;
    ct.application_auth_req(&cfg.client_id, &cfg.client_secret);

    std::thread::sleep(std::time::Duration::new(2, 0));

    let d = 1636317000_000;
    let de = d + 1 * 86400_000;
    ct.get_bid_tick_data_req(1, d, de);
    // ct.get_ask_tick_data_req(1, d, de);

    let mut collector = Collector::new();
    let mut done_asks = false;
    // event handling
    for e in rc_event {
        match e.clone() {
            _ => {
                // println!("EVENT: {:#?}", e);
            }
        };

        match e {
            ResponseEvent::Refresh => {
                println!("EVENT");
            }
            ResponseEvent::ApplicationAuthRes(_) => {}
            ResponseEvent::AccountAuthRes(_) => {}
            ResponseEvent::VersionRes(_) => {}
            ResponseEvent::AssetListRes(_) => {}
            ResponseEvent::SymbolsListRes(_) => {}
            ResponseEvent::SymbolByIdRes(_) => {}
            ResponseEvent::SymbolsForConversionRes(_) => {}
            ResponseEvent::TraderRes(_) => {}
            ResponseEvent::SubscribeSpotsRes(_) => {}
            ResponseEvent::UnsubscribeSpotsReq(_) => {}
            ResponseEvent::SpotEvent(_) => {}
            ResponseEvent::DealListRes(_) => {}
            ResponseEvent::GetTrendbarsRes(e) => {}
            ResponseEvent::ErrorRes(_) => {}
            ResponseEvent::GetTickDataRes(r) => {
                if !done_asks {
                    let ts = trans_ticks(&r.tick_data);
                    collector.bids = ts;
                    done_asks = true;
                    ct.get_ask_tick_data_req(1, d, de);
                } else {
                    let ts = trans_ticks(&r.tick_data);
                    collector.asks = ts;

                    // println!("{:#?}", collector);
                    println!("{:?}", &collector.asks);
                    println!("{:?}", &collector.bids);
                    break;
                }

                /*                let ts = trans_ticks(&r.tick_data);
                println!("{:#?}", ts);
                println!("more: {:#?}", r.has_more);*/
            }
            ResponseEvent::AssetClassListRes(_) => {}
            ResponseEvent::SubscribeDepthQuotesRes(_) => {}
            ResponseEvent::UnsubscribeDepthQuotesRes(_) => {}
            ResponseEvent::SymbolCategoryListRes(_) => {}
            _ => {}
        };
    }

    std::thread::sleep(std::time::Duration::new(100000, 0));
    /* */
}

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

            res
        }
    }
}

fn trans_ticks2(arr: &Vec<pb::TickData>) -> Vec<BTickData> {
    vec![]
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
}

#[derive(Debug)]
pub struct BTickData {
    pub date_str: String,
    pub timestamp: i64,
    pub bid_price: f64,
    pub ask_price: f64,
}
