use super::*;
use crate::helper::to_csv_out;
use crate::ta::*;
use csv::Writer;

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
        let hold = (p.clone(), ta2.dc, ta2.vel2);
        arr.push(hold);
    }
    let os = to_csv_out(&arr, false);
    let txt = format!("{}", os);
    txt
}

// v3: contains FrameMem data for Donchain Channel
pub fn serialize_position_v3(poss: &Vec<Position>) -> String {
    let mut arr = vec![];
    for p in poss {
        let frame = p.new_pos.frame.to_csv();
        arr.push((p.clone(), frame));
    }
    let os = to_csv_out(&arr, false);
    let txt = format!("{}", os);
    txt
}

// v4: contains NEFrame data for New Engine
pub fn serialize_position_v4(poss: &Vec<Position>) -> String {
    let mut arr = vec![];
    for p in poss {
        let frame = p.new_pos.frame_ne.to_csv();
        arr.push((p.clone(), frame));
    }
    let os = to_csv_out(&arr, false);
    let txt = format!("{}", os);
    txt
}

