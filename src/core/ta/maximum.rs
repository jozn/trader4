use serde::{Deserialize, Serialize};

use super::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Maximum {
    period: usize,
    max: f64,
    window: Window,
}

impl Maximum {
    pub fn new(period: usize) -> TAResult<Self> {
        match period {
            0 => Err(TAErr::WrongArgs),
            _ => Ok(Self {
                period,
                max: 0.0,
                window: Window::new(period).unwrap(),
            }),
        }
    }

    fn _next_slow(&mut self, next_val: f64) -> f64 {
        let _old_val = self.window.push(next_val);

        let mut new_max = 0.;
        for v in self.window.iter() {
            if *v > new_max {
                new_max = *v;
            }
        }

        new_max
    }

    pub fn next(&mut self, next_val: f64) -> f64 {
        let old_val = self.window.push(next_val);
        if next_val > self.max {
            self.max = next_val;
        }
        match old_val {
            None => {}
            Some(last_val) => {
                if last_val == self.max {
                    let mut new_max = 0.;
                    for v in self.window.iter() {
                        if *v > new_max {
                            new_max = *v;
                        }
                    }
                    self.max = new_max;
                }
            }
        }

        self.max
    }

    fn next_peek(&self, next_val: f64) -> f64 {
        let old_val = self.window.peek_period_tail();
        let mut max = self.max;

        match old_val {
            None => {}
            Some(last_val) => {
                if last_val == max {
                    let mut new_max = 0.;
                    // Note: this is buggy, last old value should not iterated
                    for (i, v) in self.window.iter().enumerate() {
                        // if i == self.window.size() - 1 {
                        if i == 0 {
                            continue;
                        }
                        if *v > new_max {
                            new_max = *v;
                        }
                    }
                    max = new_max;
                }
            }
        }
        max = max.max(next_val); // it really should be here for edge cases (1 window size)

        max
    }
}

impl Default for Maximum {
    fn default() -> Self {
        Self::new(9).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next() {
        let mut max = Maximum::new(3).unwrap();
        assert_eq!(max.next(10.0), 10.0);
        assert_eq!(max.next(9.0), 10.0);
        assert_eq!(max.next(11.0), 11.0);
        assert_eq!(max.next(10.0), 11.0);
        assert_eq!(max.next(8.0), 11.0);
        assert_eq!(max.next(9.0), 10.0);
    }

    #[test]
    fn test_next_redundant() {
        let mut max = Maximum::new(3).unwrap();
        assert_eq!(max.next(10.0), 10.0);
        assert_eq!(max.next(10.0), 10.0);
        assert_eq!(max.next(11.0), 11.0);
        assert_eq!(max.next(11.0), 11.0);
        assert_eq!(max.next(10.0), 11.0);
        assert_eq!(max.next(9.0), 11.0);
        assert_eq!(max.next(9.0), 10.0);
    }

    #[test]
    fn test_next_peek() {
        let mut max = Maximum::new(3).unwrap();
        assert_eq!(max.next_peek(12.0), 12.0);
        assert_eq!(max.next(10.0), 10.0);
        assert_eq!(max.next_peek(12.0), 12.0);
        assert_eq!(max.next_peek(8.0), 10.0);
        assert_eq!(max.next(9.0), 10.0);
        assert_eq!(max.next(11.0), 11.0);
        assert_eq!(max.next_peek(12.0), 12.0);
        assert_eq!(max.next(10.0), 11.0);
        assert_eq!(max.next(8.0), 11.0);
        assert_eq!(max.next_peek(9.0), 11.0);
        assert_eq!(max.next(9.0), 10.0);
        assert_eq!(max.next_peek(12.0), 12.0);
        assert_eq!(max.next_peek(9.0), 10.0);
    }
}
