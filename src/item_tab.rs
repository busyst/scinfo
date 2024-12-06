use crate::{console_menu::{ConsoleMenu, ConsoleMenusPosition}, item_list::{acquire_quality, item_index, ITEMS}, Tab};

pub struct ItemTab{
    index:i32,
    offset_y:usize,
    update:bool,
}

impl ItemTab {
    pub fn new() -> Self {
        Self {index:-1, offset_y: 0, update: false}
    }
}
impl Tab for ItemTab {
    fn update(&self) -> bool {
        self.update
    }

    fn show(&mut self, menu: &mut ConsoleMenu, _: &crate::market_structs::AucPriceList) {
        self.index = self.index.clamp(-1, ITEMS.len() as i32 - 1);
        // Clear the entire menu
        for x in 0..menu.menu_h() {
            menu.clear(x, ' ', ConsoleMenusPosition::Left);
        }
    

        // Adjust index offset to keep the selected item visible
        if self.index >= 0 {
            let mut show_index = self.index;
            let mut index_offset = 0;
            while (show_index as usize).saturating_sub(index_offset) > ((3 * menu.menu_h()) / 4) {
                index_offset += menu.menu_h() / 2;
            }
            show_index -= index_offset as i32;
            self.offset_y = index_offset;
        }
        let mut iterator: std::iter::Skip<std::slice::Iter<'_, crate::item_list::Item>> = ITEMS.iter().skip(self.offset_y);
        menu.clear_colors();
        for iter in 0..menu.menu_h() {
            if let Some(x) = iterator.next() {
                // Prepare item details with owned strings
                let sell_price_str = if x.sell_price() == 0 {format!("-N/S-")} else{x.sell_price().to_string()};
                let index_str = item_index(x.name()).to_string();
                let mut fixed_index_str = String::new();
                for _ in 1..(4usize.saturating_sub(index_str.len())) {
                    fixed_index_str.push(' ');
                }
                fixed_index_str.push_str(&index_str);
                
                let id_name = if self.index >= 0 && iter == (self.index as usize - self.offset_y){
                    format!("{} {}{}",fixed_index_str,'>',x.name())
                }else {
                    format!("{} {}{}",fixed_index_str,' ',x.name())
                };
                let q = 5 + (menu.width()/8) as u32;
                match acquire_quality(x.tags()) {
                    crate::item_list::Quality::Picklock => {}
                    crate::item_list::Quality::Newbie => {menu.colorize(5, 3+iter as u32, q, 3+iter as u32, (16,255,16));}
                    crate::item_list::Quality::Stalker => {menu.colorize(5, 3+iter as u32, q, 3+iter as u32, (27,25,255));}
                    crate::item_list::Quality::Veteran => {menu.colorize(5, 3+iter as u32, q, 3+iter as u32, (255,14,255));}
                    crate::item_list::Quality::Master => {menu.colorize(5, 3+iter as u32, q, 3+iter as u32, (255,8,8));}
                    crate::item_list::Quality::Legend => {menu.colorize(5, 3+iter as u32, q, 3+iter as u32, (255,215,0));}
                }
                // Prepare item details for display
                let details = vec![
                    id_name.as_str(),
                    sell_price_str.as_str(),
                ];
        
                // Write item details to menu
                menu.write_divided_into_rows(iter, ' ', details.as_slice(), ConsoleMenusPosition::Left);
            }else {
                menu.clear(iter, ' ', ConsoleMenusPosition::Left);
                menu.clear(iter, ' ', ConsoleMenusPosition::Right);
            }
            menu.clear(iter, ' ', ConsoleMenusPosition::Right);
        }
        if self.index >=0 && self.index < ITEMS.len() as i32{
            menu.write_line(0, format!("Rank:{}",acquire_quality(ITEMS[self.index as usize].tags()) as u32).as_str(), 0,' ',  ConsoleMenusPosition::Right);
            menu.write_line(1, format!("ID:{}",ITEMS[self.index as usize].id()).as_str(), 0,' ',  ConsoleMenusPosition::Right);

        }
        
    }

    fn input(&mut self, text: &str) {
        self.update = true;
        match text {
            "w" => {if self.index <= -1 {return;}self.index-=1;},
            "s" => {self.index+=1;},
            
            "d" => {}
            _ => {
                if text.len() == 0{
                    self.update = false;
                    return;
                }
                let mut set = true;
                let c = text.chars().next().unwrap();
                if c == '+' || c == '-'{
                    set = false;
                }
                if let Ok(x) = text.parse::<i32>(){
                    if set{
                        self.index = x;
                    }else {
                        self.index += x;
                    }
                    self.index = self.index.clamp(-1, ITEMS.len().try_into().unwrap());
                    
                }else {
                    self.update = false
                }
            }
        }
    }
}
