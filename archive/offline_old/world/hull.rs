use crate::candle::{CandleSeriesTA, Tick};
use crate::loader::CSVForexRecord;
use crate::loader::*;
use crate::offline::to_csv_out;
use crate::portfolio::*;
use crate::run::*;
use crate::*;

#[derive(Debug, Default, Clone)]
pub struct HullWorld {
    ticks: Vec<CSVForexRecord>,
    last_tick: Tick,
    balance: Vec<f64>,
    pub port: Portfolio,
    it_num: usize,
    pos_id: usize,
    tick_cnt: usize,
}

impl HullWorld {
    pub fn new() -> Self {
        Self {
            ticks: loader::_load(140_000, "/media/hamid/K/forex1/EURUSD_tab3.csv"),
            it_num: 0,
            port: Portfolio::new(100_000.),
            ..Default::default()
        }
    }

    pub fn new_runner() -> WorldRunner {
        let hull_world = HullWorld::new();
        let mut wr = WorldRunner::new(hull_world);
        wr
    }
}

impl TRunner for HullWorld {
    fn get_next_tick(&mut self) -> Option<Tick> {
        let idx = self.it_num;
        self.it_num += 1;

        if idx < self.ticks.len() {
            let forex_csv_rec = self.ticks.get(idx).unwrap();
            let mut t = forex_csv_rec.to_tick();
            // t.price = t.price * 100_000.;
            self.last_tick = t.clone();
            Some(t)
        } else {
            None
        }
    }

    fn on_next_tick_bulk(&mut self, cst: &CandleSeriesTA) {
        self.tick_cnt += 1;
        let t = &self.last_tick;

        // Close
        self.port
            .try_close_satasfied_postions(t.price as i64, t.time_s);

        let price = t.price;
        let kt = &cst.small.kline_ta_tip.clone().unwrap();
        if self.tick_cnt % 10 == 0 {
            if price > kt.ta1.sma50 {
                self.port.buy_long(t.price as i64, 1, t.time_s);
            } else {
                self.port.sell_short(t.price as i64, 1, t.time_s);
            }

            self.pos_id += 1;
        }

        /*if self.tick_cnt % 100 == 0 {
            // println!(">>>>> buy sell");
            if self.pos_id % 2 == 0 {
                self.port.sell_short(t.price as i64,1,t.time_s);
            } else {
                self.port.buy_long(t.price as i64,1,t.time_s);
            }
            self.pos_id +=1;
        }*/
        // println!("on tick {} - {}", self.it_num, self.tick_cnt)
    }

    fn on_price_tick(&mut self, cst: &CandleSeriesTA, tikc: &Tick) {
        let t = tikc;
        let b = self.port.get_total_balance(tikc.price as i64);
        if self.tick_cnt % 200 == 0 {
            self.balance.push(b);
        }
        self.port
            .try_close_satasfied_postions(t.price as i64, t.time_s);
    }

    fn on_exit(&mut self) {
        // self.port.close_all_positions()
        println!("on exit - all pos {}", self.pos_id);
        self.ticks.clear(); // for debug - clear array
                            // println!("{:#?}", self);
        self.port
            .close_all_positions(self.last_tick.price as i64, self.last_tick.time_s);
        // self.port.report(&self.last_tick);
        // println!("on exit");

        self.balance
            .push(self.port.get_total_balance(self.last_tick.price as i64));
        // println!("{:#?}", self.balance);

        // balance csv
        let o: Vec<f64> = self.port.closed.iter().map(|p| p.final_balance).collect();
        let os = to_csv_out(&o);
        println!("{}", os);
    }
}
