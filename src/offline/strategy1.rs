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
            println!("skiping {} ", kline_id);
            return;
        }
        self.acted.push(kline_id);

        self.port.buy_long(t.price as i64, 10, t.time_s);
    }

    pub fn sell(&mut self, kline_id: u64, t: &Tick) {
        if self.acted.contains(&kline_id) {
            println!("skiping short {} ", kline_id);
            return;
        }
        self.acted.push(kline_id);

        self.port.sell_short(t.price as i64, 10, t.time_s);
    }
    pub fn collect_balance(&mut self, t: &Tick) {
        let b = self.port.get_total_balance(t.price as i64);
        self.balance.push(b);
    }

    pub fn try_close_satasfied_postions(&mut self, t: &Tick) {
        self.port
            .try_close_satasfied_postions(t.price as i64, t.time_s);
    }

    pub fn close_all_exit(&mut self, t: &Tick) {
        self.port.close_all_positions(t.price as i64, t.time_s);
    }

    pub fn report(&self) {
        println!("balance {:#?}", self.balance);

        let o: Vec<f64> = self.port.closed.iter().map(|p| p.final_balance).collect();
        let os = to_csv_out(&o);
        println!("{}", os);

        println!(
            "acted kiline buckets {} \n {:#?}",
            self.acted.len(),
            self.acted
        );
    }
}
