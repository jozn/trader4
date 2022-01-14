use std::collections::{HashMap, HashSet};
use trader3;

// This simple scrip automate extracting names of symbols to be used in Pair enum.
fn main() {
    let symbols = trader3::configs::gen::pepperstone::get_symbols_list();
    // let mut mp = HashMap::new();
    let mut cats = HashSet::new();
    let mut class_list = HashSet::new();
    for c in &symbols {
        cats.insert(c.category);
        class_list.insert(c.class);
    }

    println!("Categories: \n{:#?}", cats);
    println!("Class: \n{:#?}", class_list);

    // List of
    let mut list = vec![];
    for c in &symbols {
        // if c.category == "FX Minors" {
        if c.category == "FX Exotics" {
            // if c.category == "Disableds" {
            // if c.category == "Default Category" {
            list.push(c.name);
        }
    }

    // print it manualy-- println! prints " too.
    // println!("{:#?}", list);
    println!("size: {:}", list.len());
    for name in list {
        // println!("{:},", name);
    }

    for class in class_list {
        let mut list = vec![];
        for sym in &symbols {
            if sym.class == class {
                list.push(sym.name);
            }
        }
        println!("\n\n\n================================");
        println!("{} : {:} \n", class, list.len());
        print_it(&list);
    }

    // List of assets based class types

    let classify = |class: &str| {
        let mut list_out = vec![];
        for c in &symbols {
            if c.class == class {
                list_out.push(stock_to_name(c.name));
            }
        }
        list_out
    };

    println!("\n\n\n================================");
    // List of stock name normizle
    let mut list_stocks = vec![];
    for c in &symbols {
        if c.class == "US Equities (CFDs)" {
            // if c.class == "GER Equities (CFDs)" {
            list_stocks.push(c.name);
        }
    }
    for st in list_stocks {
        let n = stock_to_name(st);
        println!("{}          <=            {}", n, st);
        // println!("{}", n);
    }
    let usa_stocks = classify("US Equities (CFDs)");
    let ger_stocks = classify("GER Equities (CFDs)");
    let eu_stocks = classify("EU Equities (CFDs)");
    let au_stocks = classify("AU Equities (CFDs)");

    print_it2("USA Stocks", &usa_stocks);
    print_it2("GER Stocks", &ger_stocks);
    print_it2("EU Stocks", &eu_stocks);
    print_it2("AU Stocks", &au_stocks);
}

fn print_it(list: &Vec<&str>) {
    // println!("size: {:}", list.len());
    for name in list {
        println!("{:},", name);
    }
}

fn print_it2(title: &str, list: &Vec<String>) {
    println!("\n\n\n=============\n>>> {} {:}\n", title, list.len());
    for name in list {
        println!("{:},", name);
    }
    println!("\n\n");
    // for enum string
}
//  Uber_Technologies_(UBER.N) =>  Uber_Technologies
fn stock_to_name(name: &str) -> String {
    let r = regex::Regex::new(r#"_*\(.*"#).unwrap();
    // let r = regex::Regex::new(r#"_.*"#).unwrap();
    let n = r.replace_all(name, "").to_string();
    let n = n.replace(" ", "_");
    let n = n.replace(".", "_");
    let n = n.replace("&", "_");
    let n = n.replace("-", "_");
    let n = n.replace("+", "_");
    let n = n.replace("__", "_");
    let n = n.replace("_Inc", "");
    let n = n.replace("_Ltd", "");
    let n = n.replace("_Corporation", "");
    let n = n.replace("_Corp", "");
    let n = n.replace("_Co", "");
    // Europe
    let n = n.replace("_AG", "");
    let n = n.replace("_SE", "");
    let n = n.replace("_SA", "");
    let n = n.replace("_Gmbh", "");
    let n = n.replace("___", "_");
    let n = n.replace("__", "_");
    let n = n.trim_matches('_');
    n.to_string()
}

/*

categories:
{
    "Default Category",
    "FX Minors",
    "Disabled",
    "FX Majors",
    "FX Exotics",
    "FX Crosses",
}
class:
{
    "GER Equities (CFDs)",
    "Metals (Spot)",
    "Currency Index (Spot)",
    "US Equities (CFDs)",
    "Commodities (Cash)",
    "ETFs",
    "EU Equities (CFDs)",
    "Energies (Spot)",
    "Forex (Spot)",
    "TEST",
    "AU Equities (CFDs)",
    "Thematics",
    "Crypto Currency (Spot)",
    "Indices (Spot)",
}
*/
