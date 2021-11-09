use crate::base::SimpleCrossEvent;
use crate::candle::{CandleSeriesTA, KlineTA, Tick};
use crate::loader::CSVForexRecord;
use crate::loader::*;
use crate::offline::strategy1::Strategy1;
use crate::offline::to_csv_out;
use crate::portfolio::*;
use crate::run::*;
use crate::*;

#[derive(Debug, Default, Clone)]
pub struct SimMacdWorld {
    ticks: Vec<CSVForexRecord>,
    last_tick: Tick,
    strategy1: Strategy1,
    name: String,
    it_num: usize,
    pos_id: usize,
    tick_cnt: usize,
}

impl SimMacdWorld {
    pub fn new() -> Self {
        Self {
            ticks: loader::_load(1400_000, "/media/hamid/K/forex1/EURUSD_tab3.csv"),
            // ticks: loader::_load(2400_000, "/media/hamid/K/forex1/EURUSD_tab4.csv"),
            // ticks: loader::_load(2400_000, "/media/hamid/K/forex1/EURUSD_tab5.csv"),
            // ticks: loader::_load(14000_000, "/media/hamid/K/forex1/GBPUSD_tab1.csv"),
            // ticks: loader::_load(1400_000, "/media/hamid/K/forex1/AUDCAD_tab1.csv"),
            // ticks: loader::_load(14000_000, "/media/hamid/K/forex1/EURJPY_tab.csv"),
            // ticks: loader::_load(14_000_000, "/media/hamid/K/forex1/USDJPY_tab.csv"),
            it_num: 0,
            strategy1: Strategy1::new(),
            ..Default::default()
        }
    }

    pub fn new_runner() -> WorldRunner {
        let hull_world = SimMacdWorld::new();
        let mut wr = WorldRunner::new(hull_world);
        wr
    }

    pub fn run_all() {
        let ds = std::fs::read_dir("/media/hamid/K/forex1/month/").unwrap();
        for d in ds {
            let d = d.unwrap().path();
            if d.is_dir() {
                continue;
            }
            let d = d.to_str().unwrap();
            let name = d.split("/").last().unwrap().replace(".csv", "");

            let r = Self {
                ticks: loader::_load(1400_000, d),
                it_num: 0,
                name,
                strategy1: Strategy1::new(),
                ..Default::default()
            };

            // println!(">>paht> {:?}", &d);
            let mut wr = WorldRunner::new(r);
            wr.run();
        }
    }
}

impl TRunner for SimMacdWorld {
    fn get_next_tick(&mut self) -> Option<Tick> {
        let idx = self.it_num;
        self.it_num += 1;

        if idx < self.ticks.len() {
            let forex_csv_rec = self.ticks.get(idx).unwrap();
            let mut t = forex_csv_rec.to_tick();
            self.last_tick = t.clone();
            Some(t)
        } else {
            None
        }
    }

    fn on_next_tick_bulk(&mut self, cst: &CandleSeriesTA) {
        self.tick_cnt += 1;
        let t = &self.last_tick;

        let price = t.price;
        // let kt_opt = &cst.medium.klines_ta.last();
        let kt_opt = get_frame_klineta(cst);
        let big_ema = cst.big.klines_ta.last();
        if kt_opt.is_none() || big_ema.is_none() {
            return;
        }
        let kt = kt_opt.unwrap();
        let big_kline = big_ema.unwrap();
        let kid = kt.kline.bucket;
        let ma = kt.ta1.ema200;
        let macd_out = kt.ta1.macd.clone();

        let up = macd_out.signal.0;
        let down = macd_out.signal.1;

        let big_ema = big_kline.ta1.ema200;

        let ta = &kt.ta1;
        // Close
        self.strategy1.try_close_satasfied_postions(t, ta);

        match up {
            SimpleCrossEvent::Bull(_) => {
                // self.strategy1.buy(kid, t);
                // if macd_out.macd < 0. && price > ma && ta.vel.count >= 3 && price > big_ema {
                if macd_out.macd < 0. && price > ma && ta.vel.count >= 3 {
                    // if macd_out.macd < 0. && price > ma  {
                    //     if macd_out.macd < 0. && price > ma && ta.vel.count >= 3  {
                    // if macd_out.macd < 0. && price > ma {
                    // if macd_out.macd < 0. {
                    self.strategy1.buy(kid, t, ta);
                    // println!("long {} - {} - {:#?}", price, kt.kline.bucket, &macd_out);
                }
            }
            SimpleCrossEvent::None => {}
            SimpleCrossEvent::Bear(_) => {}
        }

        match down {
            SimpleCrossEvent::Bull(_) => {}
            SimpleCrossEvent::None => {}
            SimpleCrossEvent::Bear(_) => {
                // self.strategy1.sell(kid, t);
                // if macd_out.macd > 0. && price < ma {
                // if macd_out.macd > 0. && price < ma && ta.vel.count >= 3 && price < big_ema {
                if macd_out.macd > 0. && price < ma && ta.vel.count >= 3 {
                    // if macd_out.macd > 0. && price < ma  {
                    //     if macd_out.macd > 0. && price < ma && ta.vel.count >= 3 {
                    // if macd_out.macd > 0.  {
                    self.strategy1.sell(kid, t, ta);
                    // self.port.sell_short(t.price as i64, 10, t.time_s);
                }
            }
        }
    }

    fn on_price_tick(&mut self, cst: &CandleSeriesTA, tikc: &Tick) {
        let kt_opt = get_frame_klineta(cst);
        if kt_opt.is_none() {
            return;
        }
        let kt = kt_opt.unwrap();

        self.strategy1.try_close_satasfied_postions(tikc, &kt.ta1);
    }

    fn on_exit(&mut self, cst: &CandleSeriesTA) {
        println!(
            "----------------------on exit - all pos {}------------------",
            self.pos_id
        );
        self.ticks.clear();

        let kt = get_frame_klineta(cst).unwrap();
        self.strategy1.close_all_exit(&self.last_tick, &kt.ta1);

        self.strategy1.report(self.name.as_str());

        println!("security : {:#?}", self.name);
        println!(
            "balance: {:#?}",
            self.strategy1.port.report.middles.last().unwrap().balance
        );

        println!("========================= exit end ============================");
    }
}

fn get_frame_klineta(cst: &CandleSeriesTA) -> Option<&KlineTA> {
    let res = cst.medium.klines_ta.last();
    res
}
