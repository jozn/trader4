use serde::{Deserialize, Serialize};
use std::slice::Iter;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SerVecUErr {
    IncorrectTime,
}

pub type SVUResult<T> = std::result::Result<T, SerVecUErr>;

// SerVec is Time Series Vector for holding Trades, Kline and KlineTA with Explecit time range
//  checking of their buckets.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SerVecUnique<T: TimeKeyRange>(Vec<T>);

impl<T: TimeKeyRange + Clone> SerVecUnique<T> {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.len() == 0
    }

    // push_replace is the only way to add items to vector. It will replace the last item if it has
    //  start_time equal for both.
    pub fn push_replace(&mut self, item: T) -> SVUResult<()> {
        if self.0.len() == 0 {
            self.0.push(item);
            return Ok(());
        }
        let last = self.0.last().unwrap();

        // todo: push back the poped item if we early return
        if last.get_start_time() == item.get_start_time() {
            self.0.pop();
        }

        match self.0.last() {
            None => {}
            Some(pre) => {
                if pre.get_start_time() > item.get_start_time() {
                    return Err(SerVecUErr::IncorrectTime);
                }

                if pre.get_end_time() != u64::MAX {
                    if !pre.get_end_time() <= item.get_start_time() {
                        return Err(SerVecUErr::IncorrectTime);
                    }
                }
            }
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

    pub fn get_from(&self, time_start: u64) -> SerVecUnique<T> {
        self.get_range(time_start, u64::MAX)
    }

    // Note: time_end is checked against start_time of buckets not the end of buckets.
    // time_end: is exclusive
    pub fn get_range(&self, time_start: u64, time_end_ex: u64) -> SerVecUnique<T> {
        let idx = self
            .0
            .binary_search_by(|p| p.get_start_time().cmp(&time_start));
        let mut id = match idx {
            Ok(id) => id,
            Err(id) => id,
        };

        let mut res = SerVecUnique::new();
        let len = self.0.len();

        while id < len {
            let v = self.0.get(id).unwrap();
            if v.get_start_time() >= time_end_ex {
                break;
            }
            res.push_replace(v.clone());
            id += 1;
        }

        res
    }

    pub fn get_from_limit(&self, time_start: u64, count: u64) -> SerVecUnique<T> {
        let idx = self
            .0
            .binary_search_by(|p| p.get_start_time().cmp(&time_start));
        let mut id = match idx {
            Ok(id) => id,
            Err(id) => id,
        };

        let mut res = SerVecUnique::new();
        let len = self.0.len();

        let mut num = 0;
        while id < len && num < count {
            let v = self.0.get(id).unwrap();
            res.push_replace(v.clone());
            id += 1;
            num += 1;
        }

        res
    }
}

// Kline and other types who implement this should provide the range of Bucket (start and end timing).
// Note end timing is also needed to provide a more solid platform rather than a flexible one.
pub trait TimeKeyRange {
    fn get_start_time(&self) -> u64;
    fn get_end_time(&self) -> u64;
}
impl<T> TimeKeyRange for &T {
    fn get_start_time(&self) -> u64 {
        self.get_start_time()
    }

    fn get_end_time(&self) -> u64 {
        self.get_end_time()
    }
}
