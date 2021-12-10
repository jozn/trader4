use super::*;
use crate::candle::{CandleConfig, CandleSeriesTA, Tick, TimeSerVec};
use crate::configs::assets;
use crate::configs::assets::*;
use crate::offline_old::run::{MiniTick, TRunner};
use crate::online::ctrader::*;
use crate::online::pb;
use crate::online::pb::TickData;
use std::fs;
use std::sync::Arc;

#[derive(Debug)]
pub struct Bot {
    pub con: Arc<CTrader>,
    pub db: Vec<PairMeta>,
}

impl Bot {
    pub fn on_connect(&self) {
        // self.con.application_auth_req(&cfg.client_id, &cfg.client_secret);
        self.con.auth(self.con.clone());
        std::thread::sleep(std::time::Duration::new(2, 0));
        let ids = assets::get_all_symbols_ids();
        println!("ids {:?}", ids);
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
            self.db.push(PairMeta::new(Pair::id_to_symbol(si)));
        }
        let m = self.db.get_mut(idx).unwrap();
        m
    }

    // This is blocks forever.
    pub(crate) fn listen_events(mut self, event_chans: std::sync::mpsc::Receiver<ResponseEvent>) {
        // event handling
        let mut actor = Actor {
            con: self.con.clone(),
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
                ResponseEvent::DisConnected => {
                    std::thread::sleep(std::time::Duration::new(2, 0));
                    self.con.reconnect_socket();
                    self.on_connect();
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
                ResponseEvent::ErrorRes(e) => {
                    println!("Error {:#?}", e);
                }
                ResponseEvent::GetTickDataRes(r) => {}
                ResponseEvent::AssetClassListRes(_) => {}
                ResponseEvent::SubscribeDepthQuotesRes(_) => {}
                ResponseEvent::UnsubscribeDepthQuotesRes(_) => {}
                ResponseEvent::SymbolCategoryListRes(_) => {}

                ResponseEvent::TraderUpdatedEvent(_) => {}
                ResponseEvent::ReconcileRes(_) => {}
                ResponseEvent::ExecutionEvent(e) => {
                    println!("ExecutionEvent {:#?}", e);
                }
                ResponseEvent::OrderErrorEvent(e) => {
                    println!("OrderErrorEvent {:#?}", e);
                }
                ResponseEvent::DisConnected => {
                    // todo
                }
            };
        }
    }
}
