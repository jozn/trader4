use super::dl_collector::Collector;
use crate::collector::collect_utils::trans_ticks;
use crate::configs::assets::Pair;
use crate::helper;
use crate::online::*;

pub fn collect_data_from_api_csv(
    // cfg: &Config,
    cr: ConnectRes,
    pari: &Pair,
    from_time_ms: i64,
    to_time_ms: i64,
) -> (String, bool) {
    let symbol_id = pari.to_symbol_id();
    let start_time = from_time_ms;
    let mut time_ms = to_time_ms;

    let mut collector = Collector::new();
    let mut in_bids = true;

    let ct = cr.conn;
    let rc_event = cr.response_chan;

    ct.auth(ct.clone());

    ct.get_bid_tick_data_req(symbol_id, from_time_ms, to_time_ms);

    let mut cnt = 0;
    let mut pre_mature_end = false;

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
            ResponseEvent::DisConnected => {
                println!("{:?} > Disconnected ...", pari);
                pre_mature_end = true;
                break;
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
                            (time_ms - first_tick.timestamp) / 3600_000,
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
                        ct.get_ask_tick_data_req(symbol_id, from_time_ms, time_ms);
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
                        time_ms = first_tick.timestamp;
                        ct.get_ask_tick_data_req(symbol_id, from_time_ms, time_ms);
                    } else {
                        break;
                    }
                }
            }
            _ => {}
        };
    }

    ct.disconnect();

    // let res = collector.final_result();
    let res = collector.to_csv();
    let tsv = format!("{:}", res);
    (tsv, pre_mature_end)
}
