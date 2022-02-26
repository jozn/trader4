use super::*;
use crate::configs::assets::Pair;
use crate::core::helper::get_time_sec;
use crate::core::helper::*;
use crate::gate_api::*;
use crate::helper;
use crate::types::WeekData;
use rand::Rng;
use serde::{Deserialize, Serialize};

static OUTPUT_FOLDER: &str = "/mnt/t/trader/trades_res/";

// todo: add week numbers to near rnd in file names

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Report {
    pub folder: String,
    pub sub_folder: String,
    pub base_time: u64,
    pub rnd_num: u16,
    pub middles: Vec<MiddleStatic>,
}

pub struct BalanceTag {
    pub time: u64,
    pub balance: f64,
}

#[derive(Debug)]
pub struct BackReportConf {
    pub report_folder: String,
    pub report_sub_folder: String,
}

impl Report {
    pub fn new(cfg: &BackReportConf) -> Self {
        Self {
            folder: cfg.report_folder.clone(),
            sub_folder: cfg.report_sub_folder.clone(),
            base_time: get_time_sec(),
            rnd_num: rand::thread_rng().gen_range(1..1000),
            middles: vec![],
        }
    }

    pub fn collect_balance(&mut self, time_sec: u64, money: &Money) {
        // self.balance.push(money.balance);
        let ms = MiddleStatic {
            time_sec: time_sec,
            time_str: helper::to_date(time_sec),
            balance: money.balance,
            locked: money.locked,
            profit: 0.,
        };
        self.middles.push(ms);
    }

    pub fn on_new_trade(&mut self, t: &NewPos, money: &Money) {
        let ms = MiddleStatic {
            time_sec: t.time_sec,
            time_str: helper::to_date(t.time_sec),
            balance: money.balance,
            locked: money.locked,
            profit: 0.,
        };
        self.middles.push(ms);
    }

    pub fn on_close_trade(&mut self, t: &Position, money: &Money) {
        let ms = MiddleStatic {
            time_sec: t.open_time,
            time_str: helper::to_date(t.open_time),
            balance: money.balance,
            locked: money.locked,
            profit: t.profit,
        };
        self.middles.push(ms);
    }

    pub fn write_to_folder_weeks(
        &self,
        port: &BackendEngine,
        week_data: &Vec<WeekData>,
        pair: &Pair,
    ) {
        let time = get_time_sec();
        let folder_base = if self.folder.is_empty() {
            OUTPUT_FOLDER.to_string()
        } else {
            self.folder.clone()
        };

        let w_first = week_data.first().unwrap().week_id;
        let w_last = week_data.last().unwrap().week_id;
        let main_folder = format!("{}{}_{:?}_{}_{}/", folder_base, time, pair, w_first, w_last);

        // All trades in main folder
        let all_closed_pos = get_all_postions_range(port, 0, u64::MAX);
        let rnd = helper::get_rand(1000);
        write_reports(&main_folder, rnd, &all_closed_pos);
        write_reports_middles(&main_folder, rnd, &self.middles);

        // Weeks reports
        for wd in week_data {
            let week_closed_pos =
                get_all_postions_range(port, wd.start as u64 / 1000, wd.end as u64 / 1000);
            let week_folder = format!("{}weeks/{}/", main_folder, wd.week_id);
            let rnd = helper::get_rand(1000);
            write_reports(&week_folder, rnd, &week_closed_pos);

            let mids = self.get_all_middles_range(wd.start / 1000, wd.end / 1000);
            write_reports_middles(&week_folder, rnd, &mids);
        }
    }

    fn get_all_middles_range(&self, start_sec: i64, end_sec: i64) -> Vec<MiddleStatic> {
        let mut out = vec![];
        for p in &self.middles {
            if p.time_sec >= start_sec as u64 && p.time_sec < end_sec as u64 {
                out.push(p.clone());
            }
        }
        out
    }

    pub fn get_report_summery(&self, port: &BackendEngine) -> ReportSummery {
        let all_closed_pos = get_all_postions_range(port, 0, u64::MAX);
        report_summery(&all_closed_pos)
    }
}

fn write_reports(folder: &str, rnd_num: u64, all_pos: &Vec<Position>) {
    let rnd_num = rnd_num as u16;
    let folder_json = format!("{}/json", folder);
    std::fs::create_dir_all(&folder);
    std::fs::create_dir_all(&folder_json).unwrap();

    std::env::set_current_dir(&folder);
    std::fs::write(
        format!("result_{}.txt", rnd_num),
        format!("{:#?}", report_summery(&all_pos)),
    );
    report_seq_profit(&all_pos, rnd_num);
    report_wins(&all_pos, rnd_num);
    report_loose(&all_pos, rnd_num);

    write_pos("all", rnd_num, all_pos.clone());
}

fn write_reports_middles(folder: &str, rnd_num: u64, middles: &Vec<MiddleStatic>) {
    std::env::set_current_dir(&folder);

    let os = to_csv_out(&middles, false);
    let txt = format!("{}", os);

    std::fs::write(format!("balance_{}.csv", rnd_num), txt);

    let js = to_json_out(&middles);
    std::fs::write("./json/balance.json", format!("{}", js));
}

fn get_all_postions_range(port: &BackendEngine, start_sec: u64, end_sec: u64) -> Vec<Position> {
    let mut all_pos = vec![];
    // todo desing better policy to have forced postions for reporting
    for (_, p) in port.opens.iter() {
        if p.open_time >= start_sec && p.open_time < end_sec {
            all_pos.push(p.clone());
        }
    }
    for (_, p) in port.closed.iter() {
        if p.open_time >= start_sec && p.open_time < end_sec {
            all_pos.push(p.clone());
        }
    }

    all_pos.sort_by(|p1, p2| p1.pos_id.cmp(&p2.pos_id));

    all_pos
}

fn report_wins(all_pos: &Vec<Position>, rnd_num: u16) {
    let mut all_arr = vec![];
    let mut longs_arr = vec![];
    let mut short_arr = vec![];
    for p in all_pos {
        if p.profit > 0. {
            all_arr.push(p.clone());
            match p.direction {
                PosDir::Long => longs_arr.push(p.clone()),
                PosDir::Short => short_arr.push(p.clone()),
            }
        }
    }

    write_pos("wins_all", rnd_num, all_arr);
    write_pos("wins_long", rnd_num, longs_arr);
    write_pos("wins_short", rnd_num, short_arr);
}

fn report_loose(all_pos: &Vec<Position>, rnd_num: u16) {
    let mut all_arr = vec![];
    let mut longs_arr = vec![];
    let mut short_arr = vec![];
    for p in all_pos {
        if p.profit < 0. {
            all_arr.push(p.clone());
            match p.direction {
                PosDir::Long => longs_arr.push(p.clone()),
                PosDir::Short => short_arr.push(p.clone()),
            }
        }
    }

    write_pos("lose_all", rnd_num, all_arr);
    write_pos("lose_long", rnd_num, longs_arr);
    write_pos("lose_short", rnd_num, short_arr);
}

fn write_pos(name: &str, rnd_num: u16, arr: Vec<Position>) {
    let os = serialize_position_v4(&arr);
    let txt = format!("{}", os);
    if txt.len() > 0 {
        std::fs::write(format!("{}_{}.csv", name, rnd_num), txt);
    }

    let js = to_json_out(&arr);
    if js.len() > 5 {
        std::fs::write(format!("./json/{}.json", name), format!("{}", js));
    }
}

fn report_seq_profit(all_pos: &Vec<Position>, rnd_num: u16) {
    let mut res = vec![];

    for p in all_pos {
        res.push(p.profit)
    }

    let os = to_csv_out(&res, false);
    let txt = format!("{}", os);

    std::fs::write(format!("seq_profit_{}.csv", rnd_num), txt);
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ReportSummery {
    pub win_cnt: u32,
    pub lose_cnt: u32,
    pub win_amount: f64,
    pub lose_amount: f64,
    pub total_time: u64,
    pub total_time_str: String,
    pub win_ratio: f32,
    pub pl_ratio: f64,
    pub net_profit: f64,
    //short/long
    pub long_cnt: u32,
    pub long_win_cnt: u32,
    pub long_win_amount: f64,
    pub long_lose_cnt: u32,
    pub long_lose_amount: f64,
    pub long_net_profit: f64,
    pub short_cnt: u32,
    pub short_win_cnt: u32,
    pub short_win_amount: f64,
    pub short_lose_cnt: u32,
    pub short_lose_amount: f64,
    pub short_net_profit: f64,
    pub long_short_ratio: f32,
    pub long_pl_ratio: f64,
    pub short_pl_ratio: f64,
    pub long_profit_perc: f64,
    pub short_profit_perc: f64,
    pub all_profit_perc: f64,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct MiddleStatic {
    pub time_sec: u64,
    pub time_str: String,
    pub balance: f64,
    pub locked: f64,
    pub profit: f64,
}

fn report_summery(all_pos: &Vec<Position>) -> ReportSummery {
    let mut total_time = 0;
    let mut win_cnt = 0;
    let mut win_amount = 0.;
    let mut lose_cnt = 0;
    let mut lose_amount = 0.;

    for p in all_pos {
        total_time += p.close_time - p.open_time;
        if p.profit > 0. {
            win_cnt += 1;
            win_amount += p.profit;
        } else {
            lose_cnt += 1;
            lose_amount += p.profit;
        }
    }

    let win_ratio = win_cnt as f32 / (win_cnt + lose_cnt) as f32;
    let pl_ratio = win_amount / lose_amount.abs();
    let net_profit = win_amount - lose_amount.abs();
    let total_time_str = helper::to_duration(total_time as i64);

    // short/long state
    let mut long_cnt = 0;
    let mut long_win_cnt = 0;
    let mut long_win_amount = 0.;
    let mut long_lose_cnt = 0;
    let mut long_lose_amount = 0.;
    let mut short_cnt = 0;
    let mut short_win_cnt = 0;
    let mut short_win_amount = 0.;
    let mut short_lose_cnt = 0;
    let mut short_lose_amount = 0.;

    for p in all_pos {
        match p.direction {
            PosDir::Long => {
                long_cnt += 1;
                if p.profit > 0. {
                    long_win_cnt += 1;
                    long_win_amount += p.profit;
                } else {
                    long_lose_cnt += 1;
                    long_lose_amount += p.profit;
                }
            }
            PosDir::Short => {
                short_cnt += 1;
                if p.profit > 0. {
                    short_win_cnt += 1;
                    short_win_amount += p.profit;
                } else {
                    short_lose_cnt += 1;
                    short_lose_amount += p.profit;
                }
            }
        }
    }

    let long_short_ratio = long_cnt as f32 / short_cnt as f32 / 2.;
    let long_pl_ratio = long_win_amount / long_lose_amount.abs();
    let short_pl_ratio = short_win_amount / short_lose_amount.abs();
    let long_net_profit = long_win_amount - long_lose_amount.abs();
    let short_net_profit = short_win_amount - short_lose_amount.abs();

    // Pure profit cal
    let long_profit_perc = long_net_profit / long_win_amount;
    let short_profit_perc = short_net_profit / short_win_amount;
    let all_profit_perc = net_profit / win_amount;

    let report_res = ReportSummery {
        win_cnt,
        lose_cnt,
        win_amount,
        lose_amount,
        total_time,
        total_time_str,
        win_ratio,
        pl_ratio,
        net_profit,
        long_cnt,
        long_win_cnt,
        long_win_amount,
        long_lose_cnt,
        long_lose_amount,
        long_net_profit,
        short_cnt,
        short_win_cnt,
        short_win_amount,
        short_lose_cnt,
        short_lose_amount,
        short_net_profit,
        long_short_ratio,
        long_pl_ratio,
        short_pl_ratio,
        long_profit_perc,
        short_profit_perc,
        all_profit_perc,
    };

    report_res
}
