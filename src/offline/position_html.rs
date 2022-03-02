use super::*;
use std::iter::repeat_with;
use std::ops::Add;
pub fn to_html_table_bk(pos: &Vec<Position>) -> String {
    let mut out = String::new();
    let csv = position_ser::serialize_position_v4(pos);
    let rows: Vec<&str> = csv.split("\n").collect();
    for r in rows {
        let fields: Vec<&str> = r.split(",").collect();
        // println!("{} {}",fields.len(), r);
        out.push_str(&format!("{} pos: {} -- {}", fields.len(), pos.len(), r));
    }

    out
}

pub fn to_html_table(pos: &Vec<Position>) -> String {
    if pos.len() == 0 {
        return "".to_string();
    }
    let mut out = "<table><tr>".to_string();
    let csv = position_ser::serialize_position_v5_html(pos);
    let rows: Vec<&str> = csv.split("\n").collect();

    // Header
    let header_str = rows.first().unwrap();
    let header_fields: Vec<&str> = header_str.split(",").collect();
    for r in header_fields {
        out.push_str(&format!("<th>{}</th>", r));
    }
    out.push_str("</tr>");

    // Each positions
    for r in rows.iter().skip(1) {
        let fields: Vec<&str> = r.split(",").collect();

        out.push_str("<tr>");
        for r in fields {
            out.push_str(&format!("<td>{}</td>", r));
        }
        out.push_str("</tr>");
    }

    out
}
