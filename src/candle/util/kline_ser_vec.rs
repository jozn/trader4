use crate::candle::*;
use serde::{Deserialize, Serialize};
use std::slice::Iter;

pub trait KlineId {
    fn get_kline_id(&self) -> u64;
}

// VolSerVec holds volume/tick kline with uniuqe id replaced if it exsits
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct KlineSerVec<T: KlineId>(Vec<T>);

impl<T: KlineId + Clone> KlineSerVec<T> {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.len() == 0
    }

    pub fn push_replace(&mut self, item: T) -> TResult<()> {
        if self.0.len() == 0 {
            self.0.push(item);
            return Ok(());
        }
        let last = self.0.last().unwrap();

        if last.get_kline_id() == item.get_kline_id() {
            self.0.pop();
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

    pub fn get_from(&self, time_start: u64) -> KlineSerVec<T> {
        self.get_range(time_start, u64::MAX)
    }

    // Note: time_end is checked against start_time of buckets not the end of buckets.
    // time_end: is exclusive
    pub fn get_range(&self, bucket_start: u64, bucket_end_ex: u64) -> KlineSerVec<T> {
        let idx = self
            .0
            .binary_search_by(|p| p.get_kline_id().cmp(&bucket_start));
        let mut id = match idx {
            Ok(id) => id,
            Err(id) => id,
        };

        let mut res = KlineSerVec::new();
        let len = self.0.len();

        while id < len {
            let v = self.0.get(id).unwrap();
            if v.get_kline_id() >= bucket_end_ex {
                break;
            }
            res.push_replace(v.clone());
            id += 1;
        }

        res
    }

    pub fn get_from_limit(&self, bucket_start: u64, count: u64) -> KlineSerVec<T> {
        let idx = self
            .0
            .binary_search_by(|p| p.get_kline_id().cmp(&bucket_start));
        let mut id = match idx {
            Ok(id) => id,
            Err(id) => id,
        };

        let mut res = KlineSerVec::new();
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

    pub fn get_from_lower(&self, bucket_start: u64) -> KlineSerVec<T> {
        let idx = self
            .0
            .binary_search_by(|p| p.get_kline_id().cmp(&bucket_start));

        let mut id = match idx {
            Ok(id) => id, // found -
            Err(id) => {
                if id > 0 {
                    id - 1
                } else {
                    0
                }
            }
        };

        let mut res = KlineSerVec::new();
        let len = self.0.len();

        while id < len {
            let v = self.0.get(id).unwrap();
            res.push_replace(v.clone());
            id += 1;
        }

        res
    }

    pub fn get_single(&self, bucket_start: u64) -> Option<T> {
        let idx = self
            .0
            .binary_search_by(|p| p.get_kline_id().cmp(&bucket_start));

        let mut id = match idx {
            Ok(id) => id, // found -
            Err(id) => {
                if id > 0 {
                    id - 1
                } else {
                    0
                }
            }
        };

        let len = self.0.len();

        while id < len {
            let v = self.0.get(id).unwrap();
            let vid = v.get_kline_id();
            if vid <= bucket_start {
                return Some(v.clone());
            }

            id += 1;
        }

        None
    }
}
