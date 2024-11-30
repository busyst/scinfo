use std::io;
use std::io::prelude::*;
use std::{collections::HashMap, io::{stdin, Write}};

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
pub trait Tab {
    fn update(&self) -> bool;
    fn show(&mut self,menu: &mut ConsoleMenu, market: &AucPriceList);
    fn input(&mut self, text: &str);
}
#[tokio::main]
async fn main() {
    let mut menu = ConsoleMenu::empty();
    menu.set_size(120, 28);
    println!("{}", menu);
    
    let mut map = HashMap::new();
    map.insert("Batareyka".to_string(), ("zy32".to_string(),32,32,32,32));
    let apl = AucPriceList::new(map);

    let mut info_tab = InfoTab::new();
    let mut market_tab = MarketTab::new();

    let mut items_tab = ItemTab::new();
    let mut recipes_tab = RecipesTab::new();
    let mut tab: &mut dyn Tab = &mut info_tab;

    //menu.write_in_middle(0, "George",' ',0);
    //menu.write_divided_into_rows(1, ' ', &["Name", "Price", "Date", "Kat"]);
    //menu.write_divided_into_rows(2, ' ', &["Cacak", "2Kk", "Heute", "Op"]);
    menu.re_render();
    //market_tab.show(&mut menu,&apl);
    

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
                println!("W:{} H:{}",menu.width(),menu.height());
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
                    }
                    menu.re_render();
                    tab.show(&mut menu,&apl);
                    
                }
            },
            ('/','i') => {
                menu.re_render();
                tab = &mut items_tab;
                tab.show(&mut menu, &apl);
            }
            ('/','r') => {
                menu.re_render();
                tab = &mut recipes_tab;
                tab.show(&mut menu, &apl);
            }
            ('/','h') => {
                menu.re_render();
                tab = &mut info_tab;
                tab.show(&mut menu, &apl);
            }
            ('/','m') => {
                menu.re_render();
                tab = &mut market_tab;
                tab.show(&mut menu, &apl);
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
    //net::get_item_price_history(regions.first().unwrap(), "zy32", "kys", 0).await?;

    
    println!("{}", menu);
}