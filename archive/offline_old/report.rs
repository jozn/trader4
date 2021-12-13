use super::portfolio::*;
use super::*;
use crate::candle::Tick;
use crate::core::helper::get_time_sec;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Report {
    pub acted: Vec<u64>,
    pub balance: Vec<f64>,
    pub middles: Vec<MiddleStatic>,
}

impl Report {
    pub fn new() -> Self {
        Self {
            acted: vec![],
            balance: vec![],
            middles: vec![],
        }
    }

    pub fn collect_balance(&mut self, balance: f64) {
        self.balance.push(balance);
    }

    pub fn on_new_trade(&mut self, t: &Position, balance: f64) {
        let ms = MiddleStatic {
            time_str: to_date(t.open_time),
            balance: balance,
            locked: 0,
            profit: 0.,
        };
        self.middles.push(ms);
    }

    pub fn on_close_trade(&mut self, t: &Position, balance: f64) {
        let ms = MiddleStatic {
            time_str: to_date(t.open_time),
            balance: balance,
            locked: 0,
            profit: t.profit,
        };
        self.middles.push(ms);
    }

    pub fn write_to_folder(&self, port: &Portfolio, name: &str) {
        let time = get_time_sec();
        let folder = format!("../trader3_out/{}_{}", time, name);
        let folder_json = format!("{}/json", folder);
        std::fs::create_dir_all(&folder);
        std::fs::create_dir_all(&folder_json).unwrap();
        let dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(&folder);

        std::fs::write("result.txt", format!("{:#?}", self.report_success(port)));

        self.report_balance(port);
        self.report_success(port);
        self.report_seq_profit(port);
        self.report_wins(port);
        self.report_loose(port);

        println!("balance: {:#?}", self.middles.last().unwrap().balance);

        std::env::set_current_dir(dir);
    }

    pub fn report_balance(&self, port: &Portfolio) {
        let os = to_csv_out(&self.middles);
        let txt = format!("{}", os);

        std::fs::write("balance.csv", txt);

        let js = to_json_out(&self.middles);
        std::fs::write("./json/balance.json", format!("{}", js));
    }

    pub fn report_success(&self, port: &Portfolio) -> ReportResult {
        let mut total_time = 0;
        let mut win_cnt = 0;
        let mut win_amount = 0.;
        let mut lose_cnt = 0;
        let mut lose_amount = 0.;

        for p in &port.closed {
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
        let total_time_str = to_duration(total_time as i64);

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

        for p in &port.closed {
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

        let report_res = ReportResult {
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
        };

        report_res
    }

    pub fn report_seq_profit(&self, port: &Portfolio) {
        let all_pos = get_all_postions(port);
        let mut res = vec![];

        for p in &all_pos {
            res.push(p.profit)
        }

        let os = to_csv_out(&res);
        let txt = format!("{}", os);

        std::fs::write("seq_profit.csv", txt);
    }

    pub fn report_wins(&self, port: &Portfolio) {
        let mut all_arr = vec![];
        let mut longs_arr = vec![];
        let mut short_arr = vec![];
        for p in port.closed.iter() {
            if p.profit > 0. {
                all_arr.push(p.clone());
                match p.direction {
                    PosDir::Long => longs_arr.push(p.clone()),
                    PosDir::Short => short_arr.push(p.clone()),
                }
            }
        }

        write_pos("wins_all", all_arr);
        write_pos("wins_long", longs_arr);
        write_pos("wins_short", short_arr);
    }

    pub fn report_loose(&self, port: &Portfolio) {
        let mut all_arr = vec![];
        let mut longs_arr = vec![];
        let mut short_arr = vec![];
        for p in port.closed.iter() {
            if p.profit < 0. {
                all_arr.push(p.clone());
                match p.direction {
                    PosDir::Long => longs_arr.push(p.clone()),
                    PosDir::Short => short_arr.push(p.clone()),
                }
            }
        }

        write_pos("lose_all", all_arr);
        write_pos("lose_long", longs_arr);
        write_pos("lose_short", short_arr);
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ReportResult {
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
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct MiddleStatic {
    pub time_str: String,
    pub balance: f64,
    pub locked: u64,
    pub profit: f64,
}

pub fn get_all_postions(port: &Portfolio) -> Vec<Position> {
    let mut all_pos = port.opens.clone();
    all_pos.append(&mut port.closed.clone());

    all_pos.sort_by(|p1, p2| p1.pos_id.cmp(&p2.pos_id));

    all_pos
}

fn write_pos(name: &str, arr: Vec<Position>) {
    let os = to_csv_out(&arr);
    let txt = format!("{}", os);
    std::fs::write(format!("{}.csv", name), txt);

    let js = to_json_out(&arr);
    std::fs::write(format!("./json/{}.json", name), format!("{}", js));
}
