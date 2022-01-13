use super::*;
use crate::online::ctrader::*;
use crate::online::pb;
// use crate::pb::TickData;
use crate::online::pb::Symbol;
use crate::ta::Stoch;
use std::fs;

pub fn run_play_api() {
    collecto_symbols();
}

// todo: waht is asset_class in pb

fn get_cfg() -> Config {
    Config {
        host: "demo.ctraderapi.com".to_string(),
        port: 5035,
        client_id: "3042_mso8gOm4NPAzIYizUC0gp941QCGvnXcRPJzTrNjVZNG0EeRFYT".to_string(),
        client_secret: "geDkrRiRyfbanU6OUwZMXKIjr4vKQyfs1Ete0unffXtS8Ah14o".to_string(),
        client_token: "lcd2-Q0fEVUMMILzkyjwcG1YKmj1vsI1HFRZJx5ETCw".to_string(),
        ctid: 23382349,
    }
}

fn collecto_symbols() {
    let cfg = get_cfg();

    let x = CTrader::connect2(&cfg);
    // let (mut cti, rc_event) = (x.conn;
    // let mut ct = cti.lock().unwrap();
    let mut ct = x.conn;
    ct.application_auth_req(&cfg.client_id, &cfg.client_secret);

    std::thread::sleep(std::time::Duration::new(2, 0));

    ct.category_list_req();
    // ct.version_req();

    // ct.list_symbols_req();

    // ct.symbol_by_id_req(vec![1, 22398]);

    // ct.subscribe_spots_req(vec![22397,22398]);

    // ct.get_trendbars_req();
    // ct.get_tick_data_req_old_bk();

    let mut builder = Builder::default();
    // event handling
    for e in x.response_chan {
        match e.clone() {
            _ => {
                // println!("EVENT: {:?}", e);
            }
        };

        match e {
            ResponseEvent::Refresh => {
                // println!("refresh")
            }

            ResponseEvent::SymbolCategoryListRes(r) => {
                println!("> SymbolCategoryListRes {:#?}", r);
                builder.categories = r.symbol_category.clone();
                ct.asset_class_list_req();
                // ct.list_assets_req();
            }

            ResponseEvent::AssetClassListRes(r) => {
                println!("> AssetClassListRes {:#?}", r);
                builder.assets_class = r.asset_class.clone();
                ct.list_assets_req();
            }

            ResponseEvent::AssetListRes(r) => {
                let mut ass_arr = vec![];
                builder.assets = r.asset.clone();
                ct.list_symbols_req();

                for a in r.asset {
                    let o = TAsset {
                        name: a.name,
                        asset_id: a.asset_id,
                        digits: a.digits.unwrap(),
                    };
                    ass_arr.push(o);
                }
                let s = format!("{:#?}", ass_arr);
                // println!("> AssetListRes {}", s);
                // fs::write("./ctrader_dubg/assets_list.txt", s);
            }
            ResponseEvent::SymbolsListRes(r) => {
                let s = format!("{:#?}", &r);
                builder.symbols_light = r.symbol.clone();

                // fs::write("./ctrader_dubg/symbols_list.txt", s);

                // println!("> SymbolsListRes {}", s);

                // get symbols details
                let mut symols = vec![];
                for s in r.symbol {
                    symols.push(s.symbol_id);
                }
                ct.symbol_by_id_req(symols);
            }
            ResponseEvent::SymbolByIdRes(r) => {
                builder.symbols_det = r.symbol.clone();
                let s = format!("{:#?}", &r);
                // println!("> SymbolByIdRes {}", s);

                builder.build();

                // fs::write("./ctrader_dubg/symbols_details.txt", s);
            }
            /*ResponseEvent::DealListRes(r) => {
                println!("{:?}", r);
            }*/
            _ => {
                println!("{:#?}", e);
            }
        };
    }

    std::thread::sleep(std::time::Duration::new(100000, 0));
    /* */
}

#[derive(Debug, Default)]
struct Builder {
    categories: Vec<pb::SymbolCategory>,
    assets_class: Vec<pb::AssetClass>,
    assets: Vec<pb::Asset>,
    symbols_light: Vec<pb::LightSymbol>,
    symbols_det: Vec<pb::Symbol>,
}

impl Builder {
    fn build(&self) {
        // Assets
        let mut ass_arr = vec![];
        for a in self.assets.clone() {
            let o = TAsset {
                name: a.name,
                asset_id: a.asset_id,
                digits: a.digits.unwrap(),
            };
            ass_arr.push(o);
        }
        let assets_str = format!("{:#?}", &ass_arr);

        // Symbols
        let mut sym_arr = vec![];
        for s in self.symbols_light.clone() {
            let base = self.get_asset(s.base_asset_id.unwrap());
            let quote = self.get_asset(s.quote_asset_id.unwrap());
            let symbol = self.get_symbol(s.symbol_id);
            let cat = self.get_category(s.symbol_category_id.unwrap());
            let class = self.get_class(s.symbol_category_id.unwrap());
            let o = TSymbol {
                name: s.symbol_name.unwrap(),
                symbol_id: s.symbol_id,
                base_asset: base.name,
                quote_asset: quote.name,
                category: cat.name.clone(),
                class: class.name.unwrap().clone(),
                description: s.description.unwrap(),
                digits: symbol.digits,
                pip: symbol.pip_position,
            };
            sym_arr.push(o);
        }
        let sym_str = format!("{:#?}", sym_arr);

        // println!("Assets {}", assets_str);
        // println!("Symbols {}", sym_str);

        let ass_fn_str = print_assets(&ass_arr);
        let sym_fn_str = print_symbols(&sym_arr);

        println!("templ Ass {}", ass_fn_str);
        println!("templ Sym {}", sym_fn_str);
        let out = format!(
            "use super::super::*; \n\n {} \n\n {} \n",
            sym_fn_str, ass_fn_str
        );
        std::fs::write("./src/configs/gen/pepperstone.rs", out);
    }

    fn get_asset(&self, aid: i64) -> pb::Asset {
        for a in &self.assets {
            if a.asset_id == aid {
                return a.clone();
            }
        }
        panic!("Asset not found {}", aid)
    }

    fn get_symbol(&self, sid: i64) -> pb::Symbol {
        for s in &self.symbols_det {
            if s.symbol_id == sid {
                return s.clone();
            }
        }
        panic!("Symobl not found {}", sid)
    }

    fn get_category(&self, cid: i64) -> pb::SymbolCategory {
        for c in &self.categories {
            if c.id == cid {
                return c.clone();
            }
        }
        panic!("Category not found {}", cid)
    }

    fn get_class(&self, cid: i64) -> pb::AssetClass {
        let sym_cat = self.get_category(cid);
        let asset_class_id = sym_cat.asset_class_id;
        for c in &self.assets_class {
            if c.id.unwrap() == asset_class_id {
                return c.clone();
            }
        }
        panic!("Class list not found {}", cid)
    }
}

fn print_assets(ass_arr: &Vec<TAsset>) -> String {
    let assets_str = format!("{:#?}", ass_arr);
    format!(
        "pub fn get_assets_list() -> Vec<TAsset> {{ \n vec!{} \n }}",
        assets_str
    )
}

fn print_symbols(syms_arr: &Vec<TSymbol>) -> String {
    let syms_str = format!("{:#?}", syms_arr);
    format!(
        "pub fn get_symbols_list() -> Vec<TSymbol> {{ \n vec!{} \n }}",
        syms_str
    )
}

#[derive(Debug)]
struct TAsset {
    name: String,
    asset_id: i64,
    digits: i32,
}

#[derive(Debug)]
struct TSymbol {
    name: String,
    symbol_id: i64,
    base_asset: String,
    quote_asset: String,
    category: String,
    class: String,
    description: String,
    digits: i32,
    pip: i32,
}
