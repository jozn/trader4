use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

use super::*;
use std::collections::vec_deque;

// Window of time
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Window {
    period_len: usize,
    arr: VecDeque<f64>,
}

impl Window {
    pub fn new(len: usize) -> TAResult<Self> {
        match len {
            0 => Err(TAErr::WrongArgs),
            _ => Ok(Window {
                period_len: len,
                arr: VecDeque::with_capacity(len),
            }),
        }
    }

    pub fn push(&mut self, price: f64) -> Option<f64> {
        let last = if self.arr.len() == self.period_len {
            self.arr.pop_back()
        } else {
            None
        };
        self.arr.push_front(price);

        last
    }

    pub fn peek_period_tail(&self) -> Option<f64> {
        let size = self.arr.len();
        if self.period_len == size {
            let first = self.arr.get(size - 1).unwrap();
            Some(*first)
        } else {
            None
        }
    }

    pub fn tail(&self) -> Option<f64> {
        let size = self.arr.len();
        match size {
            0 => None,
            _ => {
                let first = self.arr.get(size - 1).unwrap();
                Some(*first)
            }
        }
    }

    pub fn size(&self) -> usize {
        self.arr.len()
    }

    // todo: should revers
    pub fn iter(&self) -> vec_deque::Iter<f64> {
        self.arr.iter()
    }

    // todo: should revers
    pub fn iter_no_tail(&self) -> vec_deque::Iter<f64> {
        self.arr.iter()
    }
}

impl Default for Window {
    fn default() -> Self {
        Window::new(9).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_window() {
        let mut w = Window::new(3).unwrap();
        assert_eq!(w.peek_period_tail(), None);
        assert_eq!(w.size(), 0);
        assert_eq!(w.push(5.), None);
        assert_eq!(w.push(6.), None);
        assert_eq!(w.push(7.), None);
        assert_eq!(w.push(8.), Some(5.));
    }

    fn test_window_full() {
        let mut w = Window::new(3).unwrap();
        assert_eq!(w.peek_period_tail(), None);
        assert_eq!(w.size(), 0);
        assert_eq!(w.push(5.), None);
        assert_eq!(w.peek_period_tail(), Some(5.));
        assert_eq!(w.size(), 1);
        assert_eq!(w.push(6.), None);
        assert_eq!(w.peek_period_tail(), Some(6.));
        assert_eq!(w.size(), 2);
        assert_eq!(w.push(7.), None);
        assert_eq!(w.push(8.), Some(5.));
        assert_eq!(w.peek_period_tail(), Some(6.));
        assert_eq!(w.size(), 3);
    }
}
