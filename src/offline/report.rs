use super::portfolio::*;
use super::*;
use crate::candle::Tick;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Report {
    acted: Vec<u64>,
    // pub port_start: Portfolio, // todo add lifetime?
    // pub port: &'a Portfolio, // todo add lifetime?
    balance: Vec<f64>,
    middles: Vec<MiddleStatic>,
}

impl Report {
    // pub fn new(port: &'a Portfolio) -> Self {
    pub fn new() -> Self {
        Self {
            acted: vec![],
            // port: port,
            balance: vec![],
            middles: vec![],
        }
    }

    pub fn collect_balance(&mut self, t: &Tick, port: &Portfolio) {
        let b = port.get_total_balance(t.price as i64);
        self.balance.push(b);
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

    pub fn report_all(&self, port: &Portfolio) {
        self.report_balance(port);
        self.report_success(port);
    }

    pub fn report_balance(&self, port: &Portfolio) {
        let os = to_csv_out(&self.middles);
        println!("{}", os);
    }

    pub fn report_success(&self, port: &Portfolio) {
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

        let win_ratio = win_cnt as f32 / lose_cnt as f32 / 2.;
        let pl_ratio = win_amount / lose_amount.abs();
        let total_time_str = to_duration(total_time as i64);

        let report_res = ReportResult {
            win_cnt,
            lose_cnt,
            win_amount,
            lose_amount,
            total_time,
            total_time_str,
            win_ratio,
            pl_ratio,
        };

        println!("{:#?}", report_res);
    }

    // delte
    /*    pub fn report_balance_incorrect(&self) {
        let mut all_pos = self.port.opens.clone();
        all_pos.append(&mut self.port.closed.clone());

        all_pos.sort_by(|p1,p2| p1.pos_id.cmp(&p2.pos_id));

        let mut balance = self.port_start.free_usd;

        for p in all_pos.iter(){
            balance += p.profit;

            let ms = MiddleStatic {
                time_str: "".to_string(),
                balance: balance,
                locked: 0,
                profit: p.profit
            };
        }

        println!("balance {:#?}", self.balance);

        let o: Vec<f64> = self.port.closed.iter().map(|p| p.final_balance).collect();
        let os = to_csv_out(&o);
        println!("{}", os);
    }*/
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
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct MiddleStatic {
    pub time_str: String,
    pub balance: f64,
    pub locked: u64,
    pub profit: f64,
}
