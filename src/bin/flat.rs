extern crate csv;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde;

use csv::Writer;
use serde::{Serialize, Serializer};
use std::error::Error;
// use serde;

#[derive(Serialize)]
struct Row2 {
    id_num: u64,
    ma: u64,
}
#[derive(Serialize)]
struct Row<'a> {
    city: &'a str,
    country: &'a str,
    // Serde allows us to name our headers exactly,
    // even if they don't match our struct field names.
    #[serde(rename = "popcount")]
    population: u64,
    // #[serde(flatten)]
    // row: Row2,
}

#[derive(Serialize)]
struct Row3 {
    city: String,
    #[serde(rename = "popcount")]
    population: u64,
    #[serde(skip_serializing)]
    pooooo: u64,
    #[serde(skip_serializing)]
    row2: Row2,
}

type RR = (Row3, Row2);

// impl Serialize for Row {
//     fn serialize<S>(&self, serializer: S) -> Result<serde::ser::Ok, serde::ser::Error> where S: Serializer {
//         s
//     }
// }
fn main() {
    let re = example();
    println!("{:?}", re);

    let d = (
        Row3 {
            city: "Ag sdlgio".to_string(),
            population: 0,
            pooooo: 234234,
            row2: Row2 {
                id_num: 345,
                ma: 534,
            },
        },
        Row2 { id_num: 3, ma: 6 },
    );

    let mut wtr = Writer::from_writer(vec![]);
    wtr.serialize(d);
    let s = String::from_utf8(wtr.into_inner().unwrap()).unwrap();
    println!(">>>> {:}", s);
}

fn example() -> Result<(), Box<Error>> {
    let mut wtr = Writer::from_writer(vec![]);
    wtr.serialize(Row {
        city: "Boston",
        country: "United States",
        population: 4628910,
        // row: Row2 { id_num: 1, ma: 2 }
    })?;
    wtr.serialize(Row {
        city: "Concord",
        country: "United States",
        population: 42695,
        // row: Row2 { id_num: 4, ma: 27 }
    })?;

    let data = String::from_utf8(wtr.into_inner()?)?;
    assert_eq!(
        data,
        "\
city,country,popcount
Boston,United States,4628910
Concord,United States,42695
"
    );
    println!(">> {}", data);
    Ok(())
}
