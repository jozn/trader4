use serde::{Deserialize, Serialize};

use super::*;

pub type Mom = Momentum;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Momentum {
    period: usize,
    window: Window,
}

impl Momentum {
    pub fn new(period: usize) -> TAResult<Self> {
        match period {
            0 => Err(TAErr::WrongArgs),
            _ => Ok(Self {
                period,
                window: Window::new(period).unwrap(),
            }),
        }
    }

    pub(crate) fn next(&mut self, next_val: f64) -> f64 {
        let tail_val = if self.window.size() == 0 {
            next_val
        } else {
            self.window.tail().unwrap()
        };
        self.window.push(next_val);
        next_val - tail_val
    }
}

impl Default for Momentum {
    fn default() -> Self {
        Self::new(9).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        assert!(Momentum::new(0).is_err());
        assert!(Momentum::new(1).is_ok());
    }

    #[test]
    fn test_next() {
        let mut mom = Momentum::new(3).unwrap();

        assert_eq!(round(mom.next(10.0)), 0.0);
        assert_eq!(round(mom.next(10.4)), 4.0);
        assert_eq!(round(mom.next(10.57)), 5.7);
        assert_eq!(round(mom.next(10.8)), 8.0);
        assert_eq!(round(mom.next(10.9)), 4.808);
        assert_eq!(round(mom.next(10.0)), -5.393);
    }

    #[test]
    fn test_default() {
        Momentum::default();
    }
}
