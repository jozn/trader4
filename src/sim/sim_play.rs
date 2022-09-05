use rand::Rng;
use std::sync::Arc;

use crate::bar::BarConfig;
use crate::collector;
use crate::configs::assets;
use crate::configs::assets::Pair;
use crate::gate_api::GateWay;
use crate::helper::get_time_sec;
use crate::offline::*;

use super::*;

pub fn run_setting() {
    let args: Args = Args::parse();
    println!("Hello {:#?}!", args);

    let set_path = format!("./{}.json", args.setting);
    let js = std::fs::read_to_string(set_path).unwrap();
    let set: app::sim::Setting = deser_hjson::from_str(&js).unwrap();
    // println!("{:#?}", set);

    for p in &set.pairs {
        run_pair_setting(p, &set);
    }
}
fn run_pair_setting(pair: &Pair, setting: &Setting) {
    let primary_ticks = 150;
    let pair_cfg = (
        // Pair::EURUSD,
        pair.clone(), // todo: not used this field - go to sky_eng in there we have this
        BarConfig {
            primary_ticks,
            big_ticks: primary_ticks * 3,
        },
    );

    let mut run_cfg = SimConfig {
        balance: 100_000.,
        pairs_conf: vec![pair_cfg],
        ticks: vec![],
        week_data: vec![],
        pair: pair.clone(),
        out: FilesOutputConfig {
            week_data: vec![],
            pair: pair.clone(),
            print: setting.print,
            report: setting.report,
            // days_out: false,
            days_out: setting.days_out,
            web: setting.web,
        },
        report_cfg: BackReportConf {
            report_folder: "".to_string(),
            report_sub_folder: "".to_string(),
        },
    };
    let rng = Range {
        start: setting.week_start,
        end: setting.week_end,
    };
    run_cfg.run_web_sim(rng, false);
}
