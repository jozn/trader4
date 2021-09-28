use serde::{Deserialize, Serialize};
use std::slice::Iter;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SerVecErr {
    IncorrectTime,
}

pub type SVResult<T> = std::result::Result<T, SerVecErr>;

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
    pub fn push_replace(&mut self, item: T) -> SVResult<()> {
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
                    return Err(SerVecErr::IncorrectTime);
                }

                if pre.get_end_time() != u64::MAX {
                    if !pre.get_end_time() <= item.get_start_time() {
                        return Err(SerVecErr::IncorrectTime);
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
pub struct SerVec<T: TimeKey>(Vec<T>);

impl<T: TimeKey + Clone> SerVec<T> {
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

    pub fn get_from(&self, time_start: u64) -> SerVec<T> {
        self.get_range(time_start, u64::MAX)
    }

    // Note: time_end_ex is exclusive range
    pub fn get_range(&self, time_start: u64, time_end_ex: u64) -> SerVec<T> {
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

impl<T: TimeKey + Clone> Default for SerVec<T> {
    fn default() -> SerVec<T> {
        SerVec::new()
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
        let mut a = SerVec::new();

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
