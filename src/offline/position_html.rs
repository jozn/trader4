use super::*;
use crate::helper::to_csv_out_v2;
use std::fmt::format;
// use crate::sky_eng::MarkerJson;
use crate::json_output::MarkerJson;
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

pub fn to_html_table_bk2(pos: &Vec<Position>) -> String {
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

// Note: this formula is like is postions_ser
pub fn position_to_csv(poss: &Position, header: bool) -> String {
    let mut arr = vec![];
    let mut p = poss.clone();
    let frame = p.new_pos.new_pos.frame.to_csv();
    p.fid = frame.0.seq as u64;
    arr.push(p.clone());
    let os = to_csv_out_v2(&arr, false, header);
    let txt = format!("{}", os);
    txt
}

pub fn to_html_table(pos: &Vec<Position>) -> String {
    if pos.len() == 0 {
        return "".to_string();
    }
    let mut out = "<table><tr>".to_string();

    // Header
    let header_str_all = position_to_csv(pos.get(0).unwrap(), true);
    let header_str: Vec<&str> = header_str_all.split("\n").collect();
    let header_str = header_str.first().unwrap();
    let header_fields: Vec<&str> = header_str.split(",").collect();
    for r in header_fields {
        out.push_str(&format!("<th>{}</th>", r));
    }
    out.push_str("</tr>");

    // Each positions
    for p in pos {
        let r = position_to_csv(p, false);
        let fields: Vec<&str> = r.split(",").collect();
        if p.won > 0 {
            out.push_str(r#"<tr class="won">"#);
        } else {
            out.push_str(r#"<tr class="lost">"#);
        }

        for r in fields {
            out.push_str(&format!("<td>{}</td>", r));
        }
        out.push_str("</tr>");
    }

    out
}

pub fn to_json_marker(pos: &Vec<Position>) -> Vec<MarkerJson> {
    let mut out = vec![];
    for p in pos {
        let color = if p.is_won() { "#00ff00" } else { "#ff0000" };

        if p.is_long() {
            let m = MarkerJson {
                time: p.open_time as i64,
                marker_key: format!("p_{}", p.pos_id),
                position: "belowBar".to_string(),
                color: color.to_string(),
                shape: "arrowUp".to_string(),
                // text: format!(""),
                text: format!("{}", p.pos_id),
                // text: format!("{}   {}",p.pos_id, p.open_time_str),
            };
            out.push(m);
        }
    }
    out
}
