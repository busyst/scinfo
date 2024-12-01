use crate::{console_menu::ConsoleMenusPosition, ConsoleMenu, Tab};

pub struct InfoTab{
}
impl InfoTab {
    pub fn new() -> Self {
        Self {  }
    }
}
impl Tab for InfoTab {
    fn update(&self) -> bool {
        false
    }

    fn show(&mut self,menu: &mut ConsoleMenu, _: &crate::market_structs::AucPriceList) {
        menu.clear_pages(' ');
        let middle = menu.menu_h() / 2 - 3;
        menu.clear(middle + 0, '>',ConsoleMenusPosition::Left);
        menu.write_in_middle(middle + 1, "Program made by JenixDay", 0, '\0',ConsoleMenusPosition::Left);
        menu.write_in_middle(middle + 2, "for Stalcraft",0 , '\0',ConsoleMenusPosition::Left);
        menu.clear(middle + 3, '<',ConsoleMenusPosition::Left);
        menu.write_line(0, "/r - Recipies tab",0 , '\0',ConsoleMenusPosition::Right);
        menu.write_line(1, "/i - Items tab",0 , '\0',ConsoleMenusPosition::Right);
        menu.write_line(2, "/h - Help (this tab)",0 , '\0',ConsoleMenusPosition::Right);
        menu.write_line(3, "/m - Market tab",0 , '\0',ConsoleMenusPosition::Right);

    }

    fn input(&mut self, _: &str) {
        
    }
}