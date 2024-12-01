use crate::{console_menu::ConsoleMenusPosition, item_list::ITEMS, market_structs::AucPriceList, ConsoleMenu, Tab };

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

        let x = ["Name","AW","AM","MW","MM"].as_slice();
        menu.write_divided_into_rows(0, ' ', x,ConsoleMenusPosition::Center);
        
        
        for i in 0.. menu.menu_h() - 1 {
            let index = i + self.offset_y;
            if index >= list.len(){
                menu.clear(i + 1, '@',ConsoleMenusPosition::Center);
                continue;
            }
            let x = ITEMS[index].name();
            let y = list.values().nth(i).unwrap();

            let avg_week = y.avg_weekly().to_string();
            let avg_mnt = y.avg_monthly().to_string();
            let med_week = y.median_weekly().to_string();
            let med_mnt = y.median_monthly().to_string();
            let vec = vec![x,&avg_week,&avg_mnt,&med_week,&med_mnt];
            menu.write_divided_into_rows(index + 1, ' ', vec.as_slice(),ConsoleMenusPosition::Center);
        }
    }

    fn input(&mut self, _: &str) {
        
    }
}

