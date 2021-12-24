use super::*;
use crate::helper::to_csv_out;
use csv::Writer;
use crate::ta::*;

type Holder = (Position, PositionTA);

// v1: is contains PositionTA as its indicators
pub fn serialize_position_v1(poss: &Vec<Position>) -> String {
    let mut arr = vec![];
    for p in poss {
        let hold = (p.clone(), p.ta.clone());
        arr.push(hold);
    }
    let os = to_csv_out(&arr, false);
    let txt = format!("{}", os);
    txt
}

// v2: contains TA2 data for Donchain Channel
pub fn serialize_position_v2(poss: &Vec<Position>) -> String {
    let mut arr = vec![];
    for p in poss {
        let ta2 = p.new_pos.ta_med.ta2.clone();
        let hold = (p.clone(), ta2.dc, ta2.vel2 );
        arr.push(hold);
    }
    let os = to_csv_out(&arr, false);
    let txt = format!("{}", os);
    txt
}

