use super::*;
use crate::candle::{CandleSeriesTA, Tick, TimeSerVec};
use crate::ctrader::*;
use crate::online::assets;
use crate::online::assets::*;
use crate::pb;
use crate::pb::TickData;
use crate::run::{MiniTick, TRunner};
use std::fs;
use std::sync::Arc;

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
            candles: Default::default()
        }
    }
}

#[derive(Debug)]
pub struct Bot {
    pub con: Arc<CTrader>,
    pub db: Vec<PairMeta>,
}

impl Bot {
    pub fn on_connect(&self) {
        let ids = assets::get_all_symbols_ids();
        println!("ids {:?}",ids);
        self.con.subscribe_spots_req(assets::get_all_symbols_ids());
    }

    pub fn borrow_pair_meta(&mut self, si: i64) -> &mut PairMeta {
        // let mut pm = self.db.iter().find_position(|d| d.pair.to_symbol_id() == si ).unwrap();
        let mut idx = 0;
        let mut found = false;
        for pm in &self.db {
            if pm.pair.to_symbol_id() == si {
                found = true;
                break;
            }
            idx += 1;
        }
        if !found {
            self.db.push(PairMeta::new(Pair::symbol_to_id(si)));
        }
        let m = self.db.get_mut(idx).unwrap();
        m
    }

    // This is blocks forever.
    pub(crate) fn listen_events(mut self, event_chans: std::sync::mpsc::Receiver<ResponseEvent>) {
        // event handling
        let mut actor = Actor {
            con: self.con.clone()
        };
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
                    let mut pm = self.borrow_pair_meta(r.symbol_id);
                    if r.bid.is_some() {
                        let price = r.bid.unwrap() as f64;
                        let t = Tick {
                            time_s: helper::get_time_ms(),
                            price,
                            price_raw: price / 100_000.,
                            price_multi: 100_000.,
                            qty: 0.0,
                        };
                        pm.on_price_tick(t, &mut actor);
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

#[derive(Debug)]
pub struct Actor {
    pub con: Arc<CTrader>,
}

impl Actor {
/*    pub fn new(&mut self, symbol_id: i64) -> Self {
        
    }*/
    pub fn go_long(&mut self, symbol_id: i64) {
        println!("Open long postion");
        self.con.open_postion_req(symbol_id);
    }

    pub fn go_short(&mut self,  symbol_id: i64) {
        println!("Open short postion");
        self.con.open_postion_short_req(symbol_id);
    }
}


