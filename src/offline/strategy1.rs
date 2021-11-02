use super::portfolio::*;
use super::*;
use crate::candle::{Tick, TA1};

#[derive(Debug, Default, Clone)]
pub struct Strategy1 {
    acted: Vec<u64>,
    pub port: Portfolio,
    balance: Vec<f64>,
}

impl Strategy1 {
    pub fn new() -> Self {
        Self {
            port: Portfolio::new(100_000.),
            ..Default::default()
        }
    }

    pub fn buy(&mut self, kline_id: u64, t: &Tick, ta: &TA1) {
        if self.acted.contains(&kline_id) {
            // println!("skiping {} ", kline_id);
            return;
        }
        self.acted.push(kline_id);
        println!("bying");
        let param = PosParam {
            open_price: t.price as i64,
            price: t.price as i64,
            pos_size: 10,
            pos_id: 0,
            time: t.time_s,
            ta: ta.clone(),
        };
        self.port.buy_long(&param);
    }

    pub fn sell(&mut self, kline_id: u64, t: &Tick, ta: &TA1) {
        if self.acted.contains(&kline_id) {
            // println!("skiping short {} ", kline_id);
            return;
        }
        self.acted.push(kline_id);
        println!("selling");

        let param = PosParam {
            open_price: t.price as i64,
            price: t.price as i64,
            pos_size: 10,
            pos_id: 0,
            time: t.time_s,
            ta: ta.clone(),
        };
        self.port.sell_short(&param);
    }

    pub fn try_close_satasfied_postions(&mut self, t: &Tick, ta: &TA1) {
        let param = PosParam {
            open_price: t.price as i64,
            price: t.price as i64,
            pos_size: 0,
            pos_id: 0,
            time: t.time_s,
            ta: ta.clone(),
        };

        let done = self.port.try_close_satasfied_postions(&param);

        if done {
            println!("==== sold at clodes");
        }
    }

    pub fn close_all_exit(&mut self, t: &Tick, ta: &TA1) {
        let param = PosParam {
            open_price: t.price as i64,
            price: t.price as i64,
            pos_size: 10,
            pos_id: 0,
            time: t.time_s,
            ta: ta.clone(),
        };

        self.port
            .report
            .collect_balance(self.port.get_total_balance((t.price * 100_000.) as i64));
        self.port.close_all_positions(&param);
    }

    pub fn report(&self) {
        println!("portfolio #{:#?}", self.port);
        self.port.report.write_to_folder(&self.port);
    }
}
