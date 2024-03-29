use rand::Rng;
// use super::portfolio::*;
// use super::offline_helper::*;
use super::*;
use crate::core::helper::get_time_sec;
use crate::core::helper::*;
use crate::gate_api::*;
use crate::helper;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Report {
    pub folder: String,
    pub sub_folder: String,
    pub base_time: u64,
    pub rnd_num: u16,
    pub balance: Vec<f64>,
    pub middles: Vec<MiddleStatic>,
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
            balance: vec![],
            middles: vec![],
        }
    }

    // todo all money
    pub fn collect_balance(&mut self, money: &Money) {
        self.balance.push(money.balance);
    }

    pub fn on_new_trade(&mut self, t: &NewPos, money: &Money) {
        let ms = MiddleStatic {
            time_str: helper::to_date(t.time_sec),
            balance: money.balance,
            locked: money.locked,
            profit: 0.,
        };
        self.middles.push(ms);
    }

    pub fn on_close_trade(&mut self, t: &Position, money: &Money) {
        let ms = MiddleStatic {
            time_str: helper::to_date(t.open_time),
            balance: money.balance,
            locked: money.locked,
            profit: t.profit,
        };
        self.middles.push(ms);
    }

    pub fn write_to_folder(&self, port: &BackendEngine, name: &str) {
        let time = get_time_sec();

        // Build folder out
        // let folder = format!("../trader6_out/{}_{}", time, name);
        let folder_base = if self.folder.is_empty() {
            "../trader6_out/".to_string()
        } else {
            self.folder.clone()
        };
        let folder = if self.sub_folder.is_empty() {
            format!("../trader6_out/{}_{}", time, name)
        } else {
            format!("../trader6_out/{}_SUB/{}_{}", self.sub_folder, time, name)
        };

        let folder_json = format!("{}/json", folder);
        std::fs::create_dir_all(&folder);
        std::fs::create_dir_all(&folder_json).unwrap();
        let dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(&folder);

        let all_pos = get_all_postions(port);
        let all_closed_pos = get_all_postions(port);

        std::fs::write(
            format!("result_{}.txt", self.rnd_num),
            format!("{:#?}", self.report_summery(&all_closed_pos)),
        );

        self.report_balance();
        self.report_seq_profit(&all_closed_pos);
        self.report_wins(&all_closed_pos);
        self.report_loose(&all_closed_pos);

        write_pos("all", self.rnd_num, all_pos);

        // println!("balance: {:#?}", self.middles.last());

        std::env::set_current_dir(dir);
    }

    fn report_balance(&self) {
        let os = to_csv_out(&self.middles, false);
        let txt = format!("{}", os);

        std::fs::write(format!("balance_{}.csv", self.rnd_num), txt);

        let js = to_json_out(&self.middles);
        std::fs::write("./json/balance.json", format!("{}", js));
    }

    pub fn get_report_summery(&self, port: &BackendEngine) -> ReportSummery {
        let all_closed_pos = get_all_postions(port);
        self.report_summery(&all_closed_pos)
    }

    pub fn get_report_summery_range(&self, port: &BackendEngine, start: u64, end: u64) -> ReportSummery {
        let all_closed_pos = get_all_postions(port);
        self.report_summery(&all_closed_pos)
    }

    fn report_summery(&self, all_pos: &Vec<Position>) -> ReportSummery {
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

    fn report_seq_profit(&self, all_pos: &Vec<Position>) {
        let mut res = vec![];

        for p in all_pos {
            res.push(p.profit)
        }

        let os = to_csv_out(&res, false);
        let txt = format!("{}", os);

        std::fs::write(format!("seq_profit_{}.csv", self.rnd_num), txt);
    }

    fn report_wins(&self, all_pos: &Vec<Position>) {
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

        write_pos("wins_all", self.rnd_num, all_arr);
        write_pos("wins_long", self.rnd_num, longs_arr);
        write_pos("wins_short", self.rnd_num, short_arr);
    }

    fn report_loose(&self, all_pos: &Vec<Position>) {
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

        write_pos("lose_all", self.rnd_num, all_arr);
        write_pos("lose_long", self.rnd_num, longs_arr);
        write_pos("lose_short", self.rnd_num, short_arr);
    }
}

pub fn get_all_postions(port: &BackendEngine) -> Vec<Position> {
    let mut all_pos = vec![];
    for (_, p) in port.opens.iter() {
        all_pos.push(p.clone());
    }
    for (_, p) in port.closed.iter() {
        all_pos.push(p.clone());
    }

    all_pos.sort_by(|p1, p2| p1.pos_id.cmp(&p2.pos_id));

    all_pos
}

pub fn get_all_closed_postions(port: &BackendEngine) -> Vec<Position> {
    let mut all_pos = vec![];
    for (_, p) in port.closed.iter() {
        all_pos.push(p.clone());
    }

    all_pos.sort_by(|p1, p2| p1.pos_id.cmp(&p2.pos_id));

    all_pos
}

fn write_pos(name: &str, rnd_num: u16, arr: Vec<Position>) {
    let os = serialize_position_v4(&arr);
    let txt = format!("{}", os);
    std::fs::write(format!("{}_{}.csv", name, rnd_num), txt);

    let js = to_json_out(&arr);
    std::fs::write(format!("./json/{}.json", name), format!("{}", js));
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
    pub time_str: String,
    pub balance: f64,
    pub locked: f64,
    pub profit: f64,
}
