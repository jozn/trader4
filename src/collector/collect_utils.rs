use crate::online::pb;
use crate::online::pb::TickData;

// Notes: <pb::TickData> data is decending, means we have newer time in the first time
pub fn trans_ticks(arr: &Vec<pb::TickData>) -> Vec<pb::TickData> {
    // let mut arr = arr.clone();
    let first = arr.first();

    match first {
        None => {
            vec![]
        }
        Some(v) => {
            let mut res = vec![];

            let mut price = v.tick;
            let mut time = v.timestamp;
            res.push(v.clone());

            for t in arr.iter().skip(1) {
                price += t.tick;
                time += t.timestamp;

                res.push(TickData {
                    timestamp: time,
                    tick: price,
                })
            }

            // Reverse data so we have the oldest in the first of zero index
            res.reverse();
            res
        }
    }
}
