use serde::{Deserialize, Serialize};
use std::slice::Iter;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VolVecErr {
    BadId,
}

pub type VVResult<T> = std::result::Result<T, VolVecErr>;
// VolVec is used to hold trades/ticks and add an id to each trade/tick
// Todo change to TickVec
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VolVec<T> {
    next_id: u64,
    arr: Vec<Item<T>>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Item<T> {
    pub id: u64,
    pub data: Box<T>,
}

impl<T: Clone> VolVec<T> {
    pub fn new() -> Self {
        Self {
            next_id: 0,
            arr: Vec::with_capacity(1000_000),
        }
    }

    pub fn transform_seq(items: &Vec<Item<T>>) -> Vec<T> {
        let mut arr = vec![];

        for t in items.iter() {
            arr.push(t.data.as_ref().clone())
        }
        arr
    }

    pub fn len(&self) -> usize {
        self.arr.len()
    }

    pub fn is_empty(&self) -> bool {
        self.arr.len() == 0
    }

    pub fn clear(&mut self) {
        self.arr.clear()
    }

    pub fn push(&mut self, item: T) -> VVResult<Item<T>> {
        let id = self.next_id;
        self.next_id += 1;

        let it = Item {
            id: id,
            data: Box::new(item),
        };
        // println!("1");
        self.arr.push(it.clone());
        // println!("2");
        Ok(it)
    }

    pub fn pop(&mut self) -> Option<Item<T>> {
        self.arr.pop()
    }

    pub fn first(&self) -> Option<&Item<T>> {
        self.arr.first()
    }

    pub fn last(&self) -> Option<&Item<T>> {
        self.arr.last()
    }

    pub fn iter(&self) -> Iter<'_, Item<T>> {
        self.arr.iter()
    }

    pub fn get_vec(&self) -> &Vec<Item<T>> {
        &self.arr
    }

    pub fn get_from(&self, id_start: u64) -> Vec<Item<T>> {
        self.get_range(id_start, u64::MAX)
    }

    pub fn get_range(&self, id_start: u64, id_end_ex: u64) -> Vec<Item<T>> {
        // println!("1");
        let idx = self.arr.binary_search_by(|p| p.id.cmp(&id_start));
        // println!("2");
        let mut id = match idx {
            Ok(id) => id,
            Err(id) => id,
        };

        let mut res = vec![];
        let len = self.arr.len();

        while id < len {
            // println!("3 {}", id);
            let v = self.arr.get(id).unwrap();
            if v.id >= id_end_ex {
                break;
            }
            res.push(v.clone());
            id += 1;
        }

        res
    }
}

impl<T: Clone> Default for VolVec<T> {
    fn default() -> VolVec<T> {
        VolVec::new()
    }
}
