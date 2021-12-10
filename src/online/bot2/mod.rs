use super::*;
use crate::brain::Brain;
use crate::candle::Tick;
use crate::helper;
// use crate::online::bot::bot::Bot;

pub fn run_bot() {
    let cfg = Config {
        host: "demo.ctraderapi.com".to_string(),
        port: 5035,
        client_id: "3042_mso8gOm4NPAzIYizUC0gp941QCGvnXcRPJzTrNjVZNG0EeRFYT".to_string(),
        client_secret: "geDkrRiRyfbanU6OUwZMXKIjr4vKQyfs1Ete0unffXtS8Ah14o".to_string(),
        client_token: "l4jT24BWu3etFSEVViQKu1NsGpBYf2nKN0DyUGgqjy0".to_string(),
        ctid: 22851452,
    };
    // let (mut cti, rc_event) = CTrader::connect(&cfg);
    let con_res = CTrader::connect2(&cfg);
    // let mut ct = cti.lock().unwrap();
    let mut ct = con_res.conn;
    ct.application_auth_req(&cfg.client_id, &cfg.client_secret);
    std::thread::sleep(std::time::Duration::new(2, 0));

    let mut brain = Brain {
        con: Box::new(ct.clone()),
        db: vec![],
    };

    brain.on_connect();

    for e in con_res.response_chan {
        match e {
            ResponseEvent::Refresh => {
                println!("Refresh");
            }
            ResponseEvent::DisConnected => {
                std::thread::sleep(std::time::Duration::new(2, 0));
                ct.reconnect_socket();
                brain.on_connect();
            }
            ResponseEvent::ApplicationAuthRes(_) => {}
            ResponseEvent::AccountAuthRes(_) => {}
            ResponseEvent::VersionRes(_) => {}
            ResponseEvent::AssetListRes(_) => {}
            ResponseEvent::SymbolsListRes(_) => {}
            ResponseEvent::SymbolByIdRes(_) => {}
            ResponseEvent::SymbolsForConversionRes(_) => {}
            ResponseEvent::TraderRes(_) => {}
            ResponseEvent::TraderUpdatedEvent(_) => {}
            ResponseEvent::ReconcileRes(_) => {}
            ResponseEvent::ExecutionEvent(e) => {
                println!("ExecutionEvent {:#?}", e);
            }
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
                    brain.on_price_tick(r.symbol_id, t);
                }
            }
            ResponseEvent::OrderErrorEvent(_) => {}
            ResponseEvent::DealListRes(_) => {}
            ResponseEvent::GetTrendbarsRes(_) => {}
            ResponseEvent::ErrorRes(e) => {
                println!("Error {:#?}", e);
            }
            ResponseEvent::GetTickDataRes(_) => {}
            ResponseEvent::AssetClassListRes(_) => {}
            ResponseEvent::SubscribeDepthQuotesRes(_) => {}
            ResponseEvent::UnsubscribeDepthQuotesRes(_) => {}
            ResponseEvent::SymbolCategoryListRes(_) => {}
        }
    }
}