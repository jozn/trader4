use super::portfolio::*;
use super::*;
use crate::candle::Tick;

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
    pub fn buy(&mut self, kline_id: u64, t: &Tick) {
        if self.acted.contains(&kline_id) {
            // println!("skiping {} ", kline_id);
            return;
        }
        self.acted.push(kline_id);
        self.collect_balance(t);

        self.port.buy_long(t.price as i64, 40, t.time_s);
    }

    pub fn sell(&mut self, kline_id: u64, t: &Tick) {
        if self.acted.contains(&kline_id) {
            // println!("skiping short {} ", kline_id);
            return;
        }
        self.acted.push(kline_id);
        self.collect_balance(t);

        self.port.sell_short(t.price as i64, 40, t.time_s);
    }

    pub fn collect_balance(&mut self, t: &Tick) {
        let b = self.port.get_total_balance(t.price as i64);
        self.balance.push(b);
    }

    pub fn try_close_satasfied_postions(&mut self, t: &Tick) {
        let done = self
            .port
            .try_close_satasfied_postions(t.price as i64, t.time_s);

        if done {
            self.collect_balance(t);
        }
    }

    pub fn close_all_exit(&mut self, t: &Tick) {
        self.collect_balance(t);
        self.port.close_all_positions(t.price as i64, t.time_s);
    }

    pub fn report(&self) {
        self.port.report.report_all(&self.port);
        self.report_old();
    }
    pub fn report_old(&self) {
        println!("balance {:#?}", self.balance);

        let o: Vec<f64> = self.port.closed.iter().map(|p| p.final_balance).collect();
        let os = to_csv_out(&o);
        println!("{}", os);

        let mut total_time = 0;
        let mut win_cnt = 0;
        let mut win_amount = 0.;
        let mut lose_cnt = 0;
        let mut lose_amount = 0.;
        for p in &self.port.closed {
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
        println!(
            "win count:{} - {}$ \n loose count: {} - {}& ",
            win_cnt, win_amount, lose_cnt, lose_amount
        );
        println!("win ratio:{} - P/L ratio amount {}$ ", win_ratio, pl_ratio);

        println!(
            "time :{} mins -  {} hours ",
            total_time / 60,
            total_time / 3600
        );

        println!(
            "acted kiline buckets {} \n {:?}",
            self.acted.len(),
            self.acted
        );

        println!("{:#?}", self.port);
    }
}
