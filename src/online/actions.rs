use super::*;

#[derive(Debug)]
pub struct Actor {
    pub con: Arc<CTrader>,
}

impl Actor {
    pub fn go_long(&mut self, symbol_id: i64) {
        println!("Open long postion");
        self.con.open_postion_req(symbol_id);
    }

    pub fn go_short(&mut self, symbol_id: i64) {
        println!("Open short postion");
        self.con.open_postion_short_req(symbol_id);
    }
}
