use crate::{console_menu::ConsoleMenusPosition, market_structs::AucPriceList, ConsoleMenu, Tab };

pub struct MarketTab{
    offset_y: usize,
}

impl MarketTab {
    pub fn new() -> Self {
        Self { offset_y:0 }
    }
}
impl Tab for MarketTab {
    fn update(&self) -> bool {
        false
    }

    fn show(&mut self,menu: &mut ConsoleMenu, market: &AucPriceList) {
        let list = market.list();

        let x = ["Name","ID","AW","AM","MW","MM"].as_slice();
        menu.write_divided_into_rows(0, ' ', x,ConsoleMenusPosition::Left);
        
        
        for i in 0.. menu.menu_h() - 1 {
            let index = i + self.offset_y;
            if index >= list.len(){
                menu.clear(i + 1, '@',ConsoleMenusPosition::Left);
                continue;
            }
            let x = list.keys().nth(index).unwrap().as_str();
            let y = list.values().nth(i).unwrap();

            let id = y.0.as_str();
            let avg_week = y.1.to_string(); let avg_week = &avg_week;
            let avg_mnt = y.2.to_string(); let avg_mnt = &avg_mnt;
            let med_week = y.3.to_string(); let med_week = &med_week;
            let med_mnt = y.4.to_string(); let med_mnt = &med_mnt;
            let vec = vec![x,id,avg_week,avg_mnt,med_week,med_mnt];
            menu.write_divided_into_rows(index + 1, ' ', vec.as_slice(),ConsoleMenusPosition::Center);
        }
    }

    fn input(&mut self, _: &str) {
        
    }
}

