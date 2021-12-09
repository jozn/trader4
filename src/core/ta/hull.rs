use serde::{Deserialize, Serialize};

use super::*;

pub type HMA = HulllMovingAverage;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HulllMovingAverage {
    period: usize,
    wma1: WMA,
    wma2: WMA,
    wma3: WMA,
}

impl HulllMovingAverage {
    pub fn new(period: usize) -> TAResult<Self> {
        match period {
            0 | 1 => Err(TAErr::WrongArgs),
            _ => {
                let ps = (period as f64).sqrt() as usize;
                Ok(Self {
                    period,
                    wma1: WMA::new(period / 2)?,
                    wma2: WMA::new(period)?,
                    wma3: WMA::new(ps)?,
                })
            }
        }
    }

    pub fn next(&mut self, next_val: f64) -> f64 {
        let w1 = self.wma1.next(next_val);
        let w2 = self.wma2.next(next_val);

        let feed = (2.0 * w1) - w2;
        self.wma3.next(feed)
    }
}

impl Default for HulllMovingAverage {
    fn default() -> Self {
        Self::new(9).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        assert!(HulllMovingAverage::new(0).is_err());
        assert!(HulllMovingAverage::new(1).is_err());
        assert!(HulllMovingAverage::new(2).is_ok());
    }

    #[test]
    fn test_next() {
        let mut hma = HulllMovingAverage::new(5).unwrap();

        assert_eq!(round(hma.next(10.0)), 10.0);
        assert_eq!(round(hma.next(12.0)), 11.333);
        assert_eq!(round(hma.next(14.0)), 13.867);
    }

    #[test]
    fn test_next_print() {
        let mut hma = HulllMovingAverage::new(10).unwrap();

        for i in 0..10 {
            let f = (i + 10) as f64;
            let v = hma.next(f);
            println!("{}  {}", f, v);
        }
    }

    #[test]
    fn test_next_print2() {
        let mut hma = HulllMovingAverage::new(5).unwrap();
        let mut ema = EMA::new(5).unwrap();
        let arr = vec![
            10, 11, 12, 11, 9, 8, 9, 10, 11, 12, 13, 14, 13, 14, 16, 18, 16,
        ];
        for i in arr {
            let f = i as f64;
            let v = round(hma.next(f));
            let ve = round(ema.next(f));
            println!("{:>2}  {:>5}    ema: {:>5}", f, v, ve);
        }
    }

    #[test]
    fn test_next_print_tang() {
        let mut hma = HulllMovingAverage::new(5).unwrap();
        let mut ema = EMA::new(5).unwrap();
        let mut roc = ROC::new(1).unwrap();
        let arr = vec![
            10, 11, 12, 11, 9, 8, 9, 10, 11, 12, 13, 14, 13, 14, 16, 18, 16,
        ];

        let mut last = 0.;
        for i in arr {
            let f = i as f64;
            let v = round(hma.next(f));
            let ve = round(ema.next(f));
            let roc_val = round(roc.next(v));

            let tang = round(v - last);
            last = v;

            println!(
                "{:>2}  {:>5}    Tang: {:>5}     ROC(Hull): {:>5}",
                f, v, tang, roc_val
            );
        }
    }

    #[test]
    fn test_default() {
        HulllMovingAverage::default();
    }
}
