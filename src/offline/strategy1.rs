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
            port: Portfolio::new(1000_000.),
            ..Default::default()
        }
    }
    pub fn buy(&mut self, kline_id: u64, t: &Tick) {
        if self.acted.contains(&kline_id) {
            // println!("skiping {} ", kline_id);
            return;
        }
        self.acted.push(kline_id);

        let param = PosParam {
            open_price: t.price as i64,
            price: t.price as i64,
            pos_size: 10,
            usd: 10. * 1000.,
            pos_id: 0,
            time: t.time_s,
            ta: Default::default(),
        };
        self.port.buy_long(&param);
    }

    pub fn sell(&mut self, kline_id: u64, t: &Tick) {
        if self.acted.contains(&kline_id) {
            // println!("skiping short {} ", kline_id);
            return;
        }
        self.acted.push(kline_id);

        let param = PosParam {
            open_price: t.price as i64,
            price: t.price as i64,
            pos_size: 10,
            usd: 10. * 1000.,
            pos_id: 0,
            time: t.time_s,
            ta: Default::default(),
        };
        self.port.sell_short(&param);
    }

    pub fn try_close_satasfied_postions(&mut self, t: &Tick) {
        let param = PosParam {
            open_price: t.price as i64,
            price: t.price as i64,
            pos_size: 10,
            usd: 10. * 1000.,
            pos_id: 0,
            time: t.time_s,
            ta: Default::default(),
        };

        let done = self.port.try_close_satasfied_postions(&param);

        if done {}
    }

    pub fn close_all_exit(&mut self, t: &Tick) {
        let param = PosParam {
            open_price: t.price as i64,
            price: t.price as i64,
            pos_size: 10,
            usd: 10. * 1000.,
            pos_id: 0,
            time: t.time_s,
            ta: Default::default(),
        };

        self.port
            .report
            .collect_balance(self.port.get_total_balance((t.price * 100_000.) as i64));
        self.port.close_all_positions(&param);
    }

    pub fn report(&self) {
        self.port.report.write_to_folder(&self.port);
    }
}
