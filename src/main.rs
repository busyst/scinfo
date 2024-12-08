use std::io;
use std::io::prelude::*;
use std::{collections::HashMap, io::{stdin, Write}};

use auc_net::auction_scraper_show;
use console_menu::ConsoleMenu;
use info_tab::InfoTab;
use item_tab::ItemTab;
use market_structs::AucPriceList;
use market_tab::MarketTab;
use recipies_tab::RecipesTab;
mod structs;
mod market_structs;
mod info_tab;
mod market_tab;
mod item_list;
mod item_tab;
mod console_menu;
mod recipies_tab;
mod auc_net;
pub trait Tab {
    fn update(&self) -> bool;
    fn show(&mut self,menu: &mut ConsoleMenu, market: &AucPriceList);
    fn input(&mut self, text: &str);
}
fn main() {
    let mut menu = ConsoleMenu::empty();
    menu.set_size(120, 28);
    println!("{}", menu);
    
    let map = HashMap::new();
    let apl = AucPriceList::new(map);

    let mut info_tab = InfoTab::new();
    let mut market_tab = MarketTab::new();

    let mut items_tab = ItemTab::new();
    let mut recipes_tab = RecipesTab::new();
    let mut tab: &mut dyn Tab = &mut info_tab;
    
    menu.re_render();
    

    tab.show(&mut menu,&apl);
    let mut input_buffer = String::new();
    loop {
        println!("{}", menu);
        let _ = stdin().read_line(&mut input_buffer).unwrap_or(0);
        let mut it = input_buffer.chars();
        let fc = it.next().unwrap_or('\0');
        let sc = it.next().unwrap_or('\0');
        
        match (fc,sc) {
            ('/','I') => {
                println!("W:{} H:{}",menu.width(),menu.menu_h() + 4);
                // We want the cursor to stay at the end of the line, so we print without a newline and flush manually.
                write!(io::stdout(), "Press any key to continue...").unwrap();
                io::stdout().flush().unwrap();

                // Read a single byte and discard
                let _ = io::stdin().read(&mut [0u8]).unwrap();
            }
            ('/','R') => {
                // Get the rest of the string after '/R '
                if let Some(numbers_str) = input_buffer.get(3..) {
                    // Split by whitespace and parse numbers
                    let numbers: Vec<&str> = numbers_str.trim().split_whitespace().collect();
                    if numbers.len() >= 2 {
                        if let (Ok(width), Ok(height)) = (
                            numbers[0].parse::<u32>(),
                            numbers[1].parse::<u32>()
                        ) {
                            menu.set_size(width as usize, height as usize);
                        }
                    }else if Some('a') == input_buffer.chars().nth(2){
                        let x = termsize::get().unwrap();
                        menu.set_size((x.cols) as usize, (x.rows-1) as usize);
                    }
                    menu.re_render();
                    tab.show(&mut menu, &apl);
                }

            },
            ('/','i') => {
                menu.clear_colors();
                menu.re_render();
                tab = &mut items_tab;
                tab.show(&mut menu, &apl);
            }
            ('/','r') => {
                menu.clear_colors();
                menu.re_render();
                tab = &mut recipes_tab;
                tab.show(&mut menu, &apl);
            }
            ('/','h') => {
                menu.clear_colors();
                menu.re_render();
                tab = &mut info_tab;
                tab.show(&mut menu, &apl);
            }
            ('/','m') => {
                menu.clear_colors();
                menu.re_render();
                tab = &mut market_tab;
                tab.show(&mut menu, &apl);
            }
            ('/','_') => {
                tokio::runtime::Runtime::new().expect("msg").block_on(auction_scraper_show());
            }
            ('/','E') => {
                break;
            }
            _ => {
                tab.input(input_buffer.trim());
                if tab.update(){
                    tab.show(&mut menu, &apl);
                }
            }
        }
        input_buffer.clear();

    }
    
    println!("{}", menu);
}