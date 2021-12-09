use serde::{Deserialize, Serialize};

use super::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Minimum {
    period: usize,
    min: f64,
    is_new: bool,
    window: Window,
}

impl Minimum {
    pub fn new(period: usize) -> TAResult<Self> {
        match period {
            0 => Err(TAErr::WrongArgs),
            _ => Ok(Self {
                period,
                min: f64::MAX,
                is_new: true,
                window: Window::new(period).unwrap(),
            }),
        }
    }

    fn _next_slow(&mut self, next_val: f64) -> f64 {
        let _old_val = self.window.push(next_val);

        let mut new_min = f64::MAX;
        for v in self.window.iter() {
            if *v < new_min {
                new_min = *v;
            }
        }
        new_min
    }

    pub fn next(&mut self, next_val: f64) -> f64 {
        let old_val = self.window.push(next_val);

        match old_val {
            None => {}
            Some(last_val) => {
                if last_val == self.min {
                    let mut new_min = f64::MAX;
                    for v in self.window.iter() {
                        if *v < new_min {
                            new_min = *v;
                        }
                    }
                    self.min = new_min;
                }
            }
        }
        // For edge cases should be in here
        if next_val < self.min {
            self.min = next_val;
        }

        self.min
    }

    fn next_peek(&self, next_val: f64) -> f64 {
        let old_val = self.window.peek_period_tail();
        let mut min = self.min;

        match old_val {
            None => {}
            Some(last_val) => {
                if last_val == min {
                    let mut new_min = 0.;
                    // Note: this is buggy, last old value should not iterated
                    for (i, v) in self.window.iter().enumerate() {
                        // if i == self.window.size() - 1 {
                        if i == 0 {
                            continue;
                        }
                        if *v < new_min {
                            new_min = *v;
                        }
                    }
                    min = new_min;
                }
            }
        }
        min = min.min(next_val); // it really should be here for edge cases (1 window size)

        min
    }
}

impl Default for Minimum {
    fn default() -> Self {
        Self::new(9).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next() {
        let mut min = Minimum::new(3).unwrap();
        assert_eq!(min.next(10.0), 10.0);
        assert_eq!(min.next(9.0), 9.0);
        assert_eq!(min.next(11.0), 9.0);
        assert_eq!(min.next(10.0), 9.0);
        assert_eq!(min.next(8.0), 8.0);
        assert_eq!(min.next(9.0), 8.0);
    }

    #[test]
    fn test_next_redundant() {
        let mut min = Minimum::new(3).unwrap();
        assert_eq!(min.next(11.0), 11.0);
        assert_eq!(min.next(11.0), 11.0);
        assert_eq!(min.next(10.0), 10.0);
        assert_eq!(min.next(10.0), 10.0);
        assert_eq!(min.next(10.0), 10.0);
        assert_eq!(min.next(11.0), 10.0);
        assert_eq!(min.next(12.0), 10.0);
        assert_eq!(min.next(12.0), 11.0);
    }

    #[test]
    fn test_next_peek() {
        let mut min = Minimum::new(3).unwrap();
        assert_eq!(min.next_peek(8.0), 8.0);
        assert_eq!(min.next(10.0), 10.0);
        assert_eq!(min.next_peek(12.0), 10.0);
        assert_eq!(min.next_peek(8.0), 8.0);
        assert_eq!(min.next(9.0), 9.0);
        assert_eq!(min.next(11.0), 9.0);
        assert_eq!(min.next_peek(12.0), 9.0);
        assert_eq!(min.next_peek(5.0), 5.0);
        assert_eq!(min.next(9.0), 9.0);
        assert_eq!(min.next(8.0), 8.0);
        assert_eq!(min.next_peek(9.0), 8.0);
        assert_eq!(min.next(9.0), 8.0);
        assert_eq!(min.next_peek(7.0), 7.0);
        assert_eq!(min.next_peek(9.0), 8.0);
    }
}
