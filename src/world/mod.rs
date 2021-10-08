use crate::candle::{CandleSeriesTA, Tick};
use crate::forex::CSVForexRecord;
use crate::forex::*;
use crate::run::*;
use crate::*;

#[derive(Debug, Default, Clone)]
pub struct HullWorld {
    ticks: Vec<CSVForexRecord>,
    it_num: usize,
    tick_cnt: usize,
}

impl HullWorld {
    pub fn new() -> Self {
        Self {
            ticks: forex::_load(1_000, "/media/hamid/K/forex1/EURUSD_tab3.csv"),
            it_num: 0,
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
            let r = self.ticks.get(idx).unwrap();
            Some(r.to_tick())
        } else {
            None
        }
    }

    fn on_next_tick_bulk(&mut self, cst: &CandleSeriesTA) {
        self.tick_cnt += 1;
        println!("on tick {} - {}", self.it_num, self.tick_cnt)
    }

    fn on_exit(&mut self) {
        println!("on exit")
    }
}
