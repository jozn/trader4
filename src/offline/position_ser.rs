use super::*;
use crate::helper::to_csv_out;
use csv::Writer;

type Holder = (Position, PositionTA);

pub fn serialize_position(poss: &Vec<Position>) -> String {
    let mut arr = vec![];
    for p in poss {
        let hold = (p.clone(), p.ta.clone());
        arr.push(hold);
    }
    let os = to_csv_out(&arr, false);
    let txt = format!("{}", os);
    txt
}
