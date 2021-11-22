use super::*;

impl Bot1 {
    pub fn go_long(&mut self) {
        println!("Open long postion");
        self.con.open_postion_req();
    }

    pub fn go_short(&mut self) {
        println!("Open short postion");
        self.con.open_postion_short_req();
    }
}
