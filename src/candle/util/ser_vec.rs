use serde::{Deserialize, Serialize};
use std::slice::Iter;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SerVecErr {
    IncorrectTime,
}

pub type SVResult<T> = std::result::Result<T, SerVecErr>;

// Used for Trades which only has time and not bucketing.
pub trait TimeKey {
    fn get_time(&self) -> u64;
}

impl<T> TimeKey for &T {
    fn get_time(&self) -> u64 {
        self.get_time()
    }
}

// The none unique Time Series Vector for Trades vector for example.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeSerVec<T: TimeKey>(Vec<T>);

impl<T: TimeKey + Clone> TimeSerVec<T> {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.len() == 0
    }

    pub fn clear(&mut self) {
        self.0.clear()
    }

    pub fn push(&mut self, item: T) -> SVResult<()> {
        if self.0.len() == 0 {
            self.0.push(item);
            return Ok(());
        }
        let last = self.0.last().unwrap();

        // new item time must be equal or greater than last one
        if last.get_time() > item.get_time() {
            return Err(SerVecErr::IncorrectTime);
        }

        self.0.push(item);
        Ok(())
    }

    pub fn pop(&mut self) -> Option<T> {
        self.0.pop()
    }

    pub fn first(&self) -> Option<&T> {
        self.0.first()
    }

    pub fn last(&self) -> Option<&T> {
        self.0.last()
    }

    pub fn iter(&self) -> Iter<'_, T> {
        self.0.iter()
    }

    pub fn get_vec(&self) -> &Vec<T> {
        &self.0
    }

    pub fn owen_vec(self) -> Vec<T> {
        self.0
    }

    pub fn get_from(&self, time_start: u64) -> TimeSerVec<T> {
        self.get_range(time_start, u64::MAX)
    }

    // Note: time_end_ex is exclusive range
    pub fn get_range(&self, time_start: u64, time_end_ex: u64) -> TimeSerVec<T> {
        let idx = self.0.binary_search_by(|p| p.get_time().cmp(&time_start));
        let mut id = match idx {
            Ok(id) => id,
            Err(id) => id,
        };

        let mut res = Self::new();
        let len = self.0.len();

        // This loop reduce id index until those ids are equalls to start time.
        // todo this is buggy > backward looking
        /*        if id > 1{
            let mut tmp_ix = id - 1;

            while tmp_ix > 0 {
                let v = self.0.get(tmp_ix).unwrap();
                if v.get_time() == time_start {
                    id = tmp_ix;
                    tmp_ix -= 1;
                } else {
                    break;
                }
            }
        }*/

        while id < len {
            let v = self.0.get(id).unwrap();
            if v.get_time() >= time_end_ex {
                break;
            }
            res.push(v.clone());
            id += 1;
        }

        res
    }
}

impl<T: TimeKey + Clone> Default for TimeSerVec<T> {
    fn default() -> TimeSerVec<T> {
        TimeSerVec::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone)]
    struct Rec {
        time: u64,
    }

    impl TimeKey for Rec {
        fn get_time(&self) -> u64 {
            self.time
        }
    }

    fn new(t: u64) -> Rec {
        Rec { time: t }
    }

    #[test]
    fn test_ser_vec() {
        let mut a = TimeSerVec::new();

        a.push(new(1)).unwrap();
        assert_eq!(a.len(), 1);
        a.push(new(1)).unwrap();
        assert_eq!(a.len(), 2);
        a.push(new(1)).unwrap();
        assert_eq!(a.len(), 3);
        a.push(new(2)).unwrap();
        assert_eq!(a.len(), 4);
        a.push(new(3)).unwrap();
        assert_eq!(a.len(), 5);
        a.push(new(3)).unwrap();
        assert_eq!(a.len(), 6);
        assert!(a.push(new(2)).is_err());
        assert_eq!(a.len(), 6);
        a.push(new(4)).unwrap();
        assert_eq!(a.len(), 7);

        assert_eq!(a.get_vec().len(), 7);
        assert_eq!(a.get_from(1).len(), 7);
        assert_eq!(a.get_from(3).len(), 3);
    }
}
