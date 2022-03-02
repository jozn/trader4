use super::*;
use crate::helper::to_csv_out;
use crate::ta::*;
use csv::Writer;

// v4: contains NEFrame data for New Engine
pub fn serialize_position_v4(poss: &Vec<Position>) -> String {
    let mut arr = vec![];
    for mut p in poss.clone() {
        let frame = p.new_pos.new_pos.frame.to_csv();
        p.fid = frame.0.seq as u64;
        arr.push((p.clone(), frame));
    }
    let os = to_csv_out(&arr, false);
    let txt = format!("{}", os);
    txt
}

pub fn serialize_position_v5_html(poss: &Vec<Position>) -> String {
    let mut arr = vec![];
    for mut p in poss.clone() {
        let frame = p.new_pos.new_pos.frame.to_csv();
        p.fid = frame.0.seq as u64;
        // arr.push((p.clone(), frame));
        arr.push(p.clone());
    }
    let os = to_csv_out(&arr, false);
    let txt = format!("{}", os);
    txt
}
