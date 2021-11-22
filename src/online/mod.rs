pub mod actions;
pub mod assets;
pub mod decider;
pub mod runner;
pub mod start;

use super::*;
use crate::candle::{CandleSeriesTA, Tick, TimeSerVec};
use crate::ctrader::*;
use crate::pb;
use crate::pb::TickData;
use crate::run::{MiniTick, TRunner};
use std::fs;
use std::sync::Arc;

#[derive(Debug)]
pub struct Bot1 {
    con: Arc<CTrader>,
    last_tick: Option<Tick>,
    mini_tick: MiniTick,
    pub ticks_arr: TimeSerVec<Tick>,
    // event_chans: std::sync::mpsc::Receiver<ResponseEvent>,
    candles: CandleSeriesTA,
}

impl Bot1 {
    pub fn on_connect(&self) {
        self.con.subscribe_spots_req(vec![1]);
    }

    // This is blocks forever.
    fn listen_events(mut self, event_chans: std::sync::mpsc::Receiver<ResponseEvent>) {
        // event handling
        for e in event_chans {
            match e.clone() {
                _ => {
                    // println!("EVENT: {:#?}", e);
                }
            };

            match e {
                ResponseEvent::Refresh => {
                    println!("Refresh");
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
                ResponseEvent::SpotEvent(r) => {
                    if r.bid.is_some() {
                        let price = r.bid.unwrap() as f64;
                        let t = Tick {
                            time_s: helper::get_time_ms(),
                            price,
                            price_raw: price / 100_000.,
                            price_multi: 100_000.,
                            qty: 0.0,
                        };
                        self.on_price_tick(t);
                    }
                }
                ResponseEvent::DealListRes(_) => {}
                ResponseEvent::GetTrendbarsRes(_) => {}
                ResponseEvent::ErrorRes(_) => {}
                ResponseEvent::GetTickDataRes(r) => {}
                ResponseEvent::AssetClassListRes(_) => {}
                ResponseEvent::SubscribeDepthQuotesRes(_) => {}
                ResponseEvent::UnsubscribeDepthQuotesRes(_) => {}
                ResponseEvent::SymbolCategoryListRes(_) => {}
                _ => {}
            };
        }
    }
}
