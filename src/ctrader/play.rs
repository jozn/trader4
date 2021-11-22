use super::*;
use crate::pb;
use crate::pb::TickData;
use std::fs;

pub fn open_trade() {
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

    ct.open_postion_req();
    ct.subscribe_spots_req(vec![1, 2]);

    // event handling
    for e in rc_event {
        match e.clone() {
            _ => {
                println!("EVENT: {:#?}", e);
            }
        };

        match e {
            ResponseEvent::Refresh => {
                // println!("EVENT");
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

    std::thread::sleep(std::time::Duration::new(100000, 0));
    /* */
}

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

    // ct.list_assets_req();
    // ct.version_req();

    // ct.list_symbols_req();
    // ct.symbol_by_id_req(vec![1, 22398]);

    // ct.subscribe_spots_req(vec![22397,22398]);

    // ct.get_trendbars_req();
    // ct.get_tick_data_req_old_bk();

    let d = 1636317000_000;
    let de = d + 1 * 86400_000;
    ct.get_bid_tick_data_req(1, d, de);
    ct.get_ask_tick_data_req(1, d, de);
    // ct.get_ask_tick_data_req(1,d,d+7*8640_000);

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
            ResponseEvent::GetTrendbarsRes(_) => {}
            ResponseEvent::ErrorRes(_) => {}
            ResponseEvent::GetTickDataRes(r) => {
                let ts = trans_ticks(&r.tick_data);
                println!("{:#?}", ts);
                println!("more: {:#?}", r.has_more);
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

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TickDataNormal {
    /// Tick timestamp.
    #[prost(int64, required, tag = "1")]
    pub timestamp: i64,
    /// Tick price.
    #[prost(int64, required, tag = "2")]
    pub tick: i64,
}

pub fn response_collector() {
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

    ct.list_assets_req();
    ct.version_req();

    ct.list_symbols_req();
    // ct.symbol_by_id_req(vec![1, 22398]);

    // ct.subscribe_spots_req(vec![22397,22398]);

    ct.get_trendbars_req();
    ct.get_tick_data_req_old_bk();

    // event handling
    for e in rc_event {
        match e.clone() {
            _ => {
                // println!("EVENT: {:?}", e);
            }
        };

        match e {
            ResponseEvent::Refresh => {
                // println!("refresh")
            }
            ResponseEvent::ApplicationAuthRes(r) => {
                fs::write(
                    "./ctrader_dubg/application_auth_res.txt",
                    format!("{:#?}", r),
                );
            }
            ResponseEvent::AccountAuthRes(r) => {
                fs::write("./ctrader_dubg/account_auth_res.txt", format!("{:#?}", r));
            }
            ResponseEvent::VersionRes(r) => {
                fs::write("./ctrader_dubg/version.txt", format!("{:#?}", r));
            }
            ResponseEvent::AssetListRes(r) => {
                let s = format!("{:#?}", r);
                fs::write("./ctrader_dubg/assets_list.txt", s);
            }
            ResponseEvent::SymbolsListRes(r) => {
                let s = format!("{:#?}", &r);
                fs::write("./ctrader_dubg/symbols_list.txt", s);

                // get symbols details
                let mut symols = vec![];
                for s in r.symbol {
                    symols.push(s.symbol_id);
                }
                ct.symbol_by_id_req(symols);
            }
            ResponseEvent::SymbolByIdRes(r) => {
                let s = format!("{:#?}", &r);
                fs::write("./ctrader_dubg/symbols_details.txt", s);
            }
            ResponseEvent::SymbolsForConversionRes(_) => {}
            ResponseEvent::SubscribeSpotsRes(_) => {}
            ResponseEvent::UnsubscribeSpotsReq(_) => {}
            ResponseEvent::SpotEvent(r) => {
                let s = format!("{:#?}", r);
                fs::write("./ctrader_dubg/assets_list.txt", s);
            }
            ResponseEvent::DealListRes(r) => {
                println!("{:?}", r);
            }
            ResponseEvent::GetTrendbarsRes(r) => {
                fs::write("./ctrader_dubg/trend_bars.txt", format!("{:#?}", r));
            }
            ResponseEvent::ErrorRes(_) => {}
            ResponseEvent::GetTickDataRes(r) => {
                fs::write("./ctrader_dubg/get_tick_data.txt", format!("{:#?}", r));
            }
            _ => {}
        };
    }

    std::thread::sleep(std::time::Duration::new(100000, 0));
    /* */
}

pub fn play2() {
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

    ct.list_assets_req();
    ct.version_req();

    ct.list_symbols_req();
    ct.symbol_by_id_req(vec![1, 22398]);

    // ct.subscribe_spots_req(vec![22397,22398]);

    ct.get_trendbars_req();
    ct.get_tick_data_req_old_bk();

    // event handling
    for e in rc_event {
        match e {
            _ => println!("event {:?}", e),
        }
    }

    std::thread::sleep(std::time::Duration::new(100000, 0));
    /* */
}
