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
    it_num: usize,
    pos_id: usize,
    tick_cnt: usize,
}

impl SimMacdWorld {
    pub fn new() -> Self {
        Self {
            ticks: loader::_load(1400_000, "/media/hamid/K/forex1/EURUSD_tab3.csv"),
            // ticks: loader::_load(1400_000, "/media/hamid/K/forex1/GBPUSD_tab1.csv"),
            // ticks: loader::_load(1400_000, "/media/hamid/K/forex1/AUDCAD_tab1.csv"),
            // ticks: loader::_load(14000_000, "/media/hamid/K/forex1/EURJPY_tab.csv"),
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
}

impl TRunner for SimMacdWorld {
    fn get_next_tick(&mut self) -> Option<Tick> {
        let idx = self.it_num;
        self.it_num += 1;

        if idx < self.ticks.len() {
            let forex_csv_rec = self.ticks.get(idx).unwrap();
            let mut t = forex_csv_rec.to_tick();
            t.price = t.price * 100_000.;
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
        if kt_opt.is_none() {
            return;
        }
        let kt = kt_opt.unwrap();
        let kid = kt.kline.bucket;
        let ma = kt.ta1.ema10;
        let ma = kt.ta1.ema10;
        let macd_out = kt.ta1.macd.clone();

        let up = macd_out.signal.0;
        let down = macd_out.signal.1;

        let ta = &kt.ta1;
        // Close
        self.strategy1.try_close_satasfied_postions(t, ta);

        match up {
            SimpleCrossEvent::Bull(_) => {
                // self.strategy1.buy(kid, t);
                if macd_out.macd < 0. && price > ma && ta.vel.count >= 3 {
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
                if macd_out.macd > 0. && price < ma && ta.vel.count >= 3 {
                    // if macd_out.macd > 0.  {
                    // self.strategy1.sell(kid, t, ta);
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
        println!("on exit - all pos {}", self.pos_id);
        self.ticks.clear();

        let kt = get_frame_klineta(cst).unwrap();
        self.strategy1.close_all_exit(&self.last_tick, &kt.ta1);

        self.strategy1.report();
        println!("========================= on exit ============================");
    }
}

fn get_frame_klineta(cst: &CandleSeriesTA) -> Option<KlineTA> {
    let res = cst.medium.klines_ta.last().cloned();
    res
}
