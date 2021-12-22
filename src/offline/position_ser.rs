use super::*;
use crate::helper::to_csv_out;
use csv::Writer;

type Holder = (Position, PositionTA);
/*
fn serilze_pos(pos: &Position) -> String{
    let hold = (pos.clone(),pos.ta.clone());
    let mut wtr = Writer::from_writer(vec![]);
    wtr.serialize(d);
    let s = String::from_utf8(wtr.into_inner().unwrap()).unwrap();
    s
}
*/

pub fn serilize_postions(poss: &Vec<Position>) -> String {
    let mut arr = vec![];
    for p in poss {
        let hold = (p.clone(), p.ta.clone());
        arr.push(hold);
    }
    let os = to_csv_out(&arr, false);
    let txt = format!("{}", os);
    txt
}
