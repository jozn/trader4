use crate::brain::{Brain, PairCandleCfg};
use crate::candle::CandleConfig;
use crate::collector;
use crate::collector::row_data::BTickData;
use crate::configs::assets::Pair;
use crate::gate_api::GateWay;
use crate::offline::report::ReportSummery;
use crate::offline::{BackReportConf, BackendEngine, BackendEngineOuter};
use std::sync::Arc;

#[derive(Debug)]
pub struct CfgSummery {
    pub pair_cfg: PairCandleCfg,
    pub summer: ReportSummery,
}

pub fn run1() {
    let mut all_summery_cfg = vec![];

    // let weeks = vec![40, 41, 42, 43];
    let weeks = vec![48];
    let mut all_ticks = vec![];
    for w in weeks {
        let p = format!("/mnt/c/me/data/EURUSD/{}.tsv", w);
        let ticks = collector::loader::load_rows(&p);
        ticks.iter().for_each(|t| all_ticks.push(t.clone()));
    }

    let pair_cfgs = get_all_candle_cfgs();
    for (i, pair_cfg) in pair_cfgs.iter().enumerate() {
        if i == 100 {
            //break;
        }
        if i % 30 != 0 {
            // only 10%
            continue;
        }
        let sum = run_sample(pair_cfg.clone(), &all_ticks);

        let cs = CfgSummery {
            pair_cfg: pair_cfg.clone(),
            summer: sum,
        };
        all_summery_cfg.push(cs);
    }

    all_summery_cfg.sort_by(|v0, v1| {
        let a = (v0.summer.win_ratio * 100.) as u64;
        let b = (v1.summer.win_ratio * 100.) as u64;
        b.cmp(&a) // decending
    });

    let mut filterd = vec![];
    for s in all_summery_cfg {
        if s.summer.total_time != 0 // did some trade (win or loose)
        || s.summer.win_cnt + s.summer.lose_cnt > 4
        // least amount of trades
        {
            filterd.push(s);
        }
    }

    println!("{:#?}", filterd);
}

pub fn run_sample(pair_cfg: PairCandleCfg, all_tikcs: &Vec<BTickData>) -> ReportSummery {
    let report_cfg = BackReportConf {
        report_folder: "../trader4_out/".to_string(),
        report_sub_folder: "".to_string(),
    };
    let backend = BackendEngineOuter::new(100_000, &report_cfg);
    let mut back_arc = Arc::new(backend);

    let mut brain = Brain::new(back_arc.clone(), vec![pair_cfg]);

    for (i, t) in all_tikcs.iter().enumerate() {
        if i % 10000 == 0 {
            // println!("{}", i);
        }
        back_arc.next_tick(1, t.clone());
        brain.on_price_tick(1, t.to_tick())
    }
    let mut x = back_arc.engine.borrow_mut();
    x.close_all_positions();

    x.get_report_summery()
}

pub fn get_all_candle_cfgs() -> Vec<PairCandleCfg> {
    let mut arr = vec![];

    for s in (5..50).step_by(5) {
        for m in (4..30).step_by(4) {
            for mut b in (20..150).step_by(10) {
                for vel in 10..=60 {
                    if m * s > 500 || b * s > 1000 {
                        continue;
                    }
                    if b < m * 3 {
                        continue;
                    }

                    let cfg = CandleConfig {
                        small_tick: s,
                        medium_tick: m,
                        big_tick: b,
                        vel_period: vel,
                    };

                    // faster b in higher numbers
                    if b > 50 {
                        b += 5;
                    }

                    arr.push((Pair::EURUSD, cfg));
                }
            }
        }
    }

    arr
}

pub fn get_all_candle_cfgs_bk() -> Vec<PairCandleCfg> {
    let mut arr = vec![];

    for s in (4..50).step_by(2) {
        for m in (4..40).step_by(2) {
            for mut b in (10..150).step_by(5) {
                if m * s > 500 || b * s > 1000 {
                    continue;
                }
                if b < m * 3 {
                    continue;
                }

                let cfg = CandleConfig {
                    small_tick: s,
                    medium_tick: m,
                    big_tick: b,
                    vel_period: 30,
                };

                // faster b in higher numbers
                if b > 50 {
                    b += 5;
                }

                arr.push((Pair::EURUSD, cfg));
            }
        }
    }

    arr
}
