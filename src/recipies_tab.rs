use std::{collections::{HashMap, HashSet}, ops::{Add, AddAssign}};

use crate::{console_menu::{ConsoleMenu, ConsoleMenusPosition}, item_list::{item_index, Item, ITEMS, RECIPES}, Tab};
fn levenshtein_distance(s1: &str, s2: &str) -> usize {
    let s1_chars: Vec<char> = s1.chars().collect();
    let s2_chars: Vec<char> = s2.chars().collect();
    
    let m = s1_chars.len();
    let n = s2_chars.len();
    
    // Create a matrix of size (m+1) x (n+1)
    let mut dp = vec![vec![0; n + 1]; m + 1];
    
    // Initialize first row and column
    for i in 0..=m {
        dp[i][0] = i;
    }
    for j in 0..=n {
        dp[0][j] = j;
    }
    
    // Fill the matrix
    for i in 1..=m {
        for j in 1..=n {
            if s1_chars[i-1] == s2_chars[j-1] {
                dp[i][j] = dp[i-1][j-1];
            } else {
                dp[i][j] = 1 + std::cmp::min(
                    dp[i-1][j-1], // substitution
                    std::cmp::min(
                        dp[i-1][j],   // deletion
                        dp[i][j-1]    // insertion
                    )
                );
            }
        }
    }
    
    dp[m][n]
}
fn normalized_distance(word1: &str, word2: &str) -> f32 {
    let nw1 = &word1.to_lowercase();
    let nw2 = &word2.to_lowercase();
    let distance = levenshtein_distance(nw1, nw2) as f32;
    let max_length = std::cmp::max(word1.len(), word2.len()) as f32;
    // Normalize the distance by dividing by the length of the longer string
    (distance / max_length) * if nw1.ends_with(nw2) || nw2.ends_with(nw1) || nw1.starts_with(nw2) || nw2.starts_with(nw1) {0.5} else {1.0}
}

enum RecipeStep{

    // Item id, count
    TakeRem(usize,u32),
    // Item id, count
    Take(usize,u32),
    // Item id, count
    Buy(usize,u32),
    // Craft id, count
    Craft(usize,u32),
}

pub struct RecipesTab{
    index:i32,
    update:bool,
    number:u32,
    mode:u8,
}

impl RecipesTab {
    pub fn new() -> Self {
        Self {index:-1, mode: 1, number: 1, update: false}
    }
    
    pub fn set_index(&mut self, index: i32) {
        self.index = index;
    }
    fn is_recipe_recursive(recipe_index: usize, visited: &mut HashSet<usize>, ingr_ignore: &HashSet<usize>) -> bool {
        // Reset visited set for each new top-level recipe check
        visited.clear();
        
        // Recursive internal helper function to check recipe dependencies
        fn check_recursive_path(
            current_index: usize, 
            visited: &mut HashSet<usize>, 
            ingr_ignore: &HashSet<usize>
        ) -> bool {
            // If we've already seen this recipe in the current path, it's recursive
            if visited.contains(&current_index) {
                return true;
            }
            // Mark current recipe as visited
            visited.insert(current_index);
            
            if ingr_ignore.contains(&current_index){
                return false;
            }
            // Find the recipe for the current item
            if let Some(recipe) = RECIPES.iter().find(|r| r.result().index() == current_index) {
                // Check each ingredient recursively
                for ingredient in recipe.ingredients() {
                    // Recursively check if any ingredient leads to a cycle
                    if check_recursive_path(ingredient.index(), visited,ingr_ignore) {
                        return true;
                    }
                }
            }
            
            // No recursion found
            false
        }
        
        // Start the recursive check for the specific recipe
        check_recursive_path(recipe_index, visited,ingr_ignore)
    }
    fn recept_solver(
        index_for_item_and_count: (usize, u32),
        steps: &mut Vec<(RecipeStep, u16)>,
        extra_items: &mut HashMap<usize,u32>,
    ) {
        let mut items_to_craft = vec![index_for_item_and_count];
        //let mut processed_recipes: HashSet<usize> = HashSet::new();
        let mut deep = 0;
        
        let mut infinite_items: HashSet<usize> = HashSet::new();
        infinite_items.insert(item_index("Bottle of Pure Water"));

        while !items_to_craft.is_empty() {
            let mut current_item = items_to_craft.remove(0);
            if current_item == (0,0){
                deep -=1;
                continue;
            }
            if let Some(d) =  extra_items.get(&current_item.0){
                if *d != 0 {
                    let delta = *d as i32 - current_item.1 as i32;
                    if delta < 0 {
                        current_item.1 -= *d;
                        steps.push((RecipeStep::TakeRem(current_item.0, *d),deep));
                    }else {
                        steps.push((RecipeStep::TakeRem(current_item.0, current_item.1),deep));
                        *extra_items.get_mut(&current_item.0).unwrap() = delta as u32;
                        continue;
                    }
                }
                
            }
            // Check if the item has a recipe
            if let Some((idx, rec)) = RECIPES.iter().enumerate().find(|(_,rec)| rec.result().index() == current_item.0) {
                // Prevent processing the same recipe multiple times to avoid infinite loops
                /*if processed_recipes.contains(&idx) {
                    steps.push((RecipeStep::Buy(current_item.0, current_item.1), deep));
                    continue;
                }*/
                // Check if the recipe is recursive
                let mut recursion_check = HashSet::new();
                if Self::is_recipe_recursive(idx, &mut recursion_check,&infinite_items) {
                    // If recipe is recursive, buy the item directly
                    steps.push((RecipeStep::Buy(current_item.0, current_item.1), deep));
                    continue;
                }
                // Mark this recipe as processed
                //processed_recipes.insert(idx);
                let mut a_steps = vec![];
                let times = (current_item.1 + rec.result().count() - 1) / rec.result().count();
                for x in rec.ingredients().iter() {
                    if infinite_items.contains(&x.index()){
                        a_steps.push((x.index(), x.count()));
                    }else {
                        items_to_craft.insert(0, (x.index(),x.count()*times));
                    }
                }
                // Craft the requested item
                steps.push((RecipeStep::Craft(current_item.0, times), deep));
                let delta_rec = times * rec.result().count() - current_item.1;
                if delta_rec != 0{
                    if extra_items.contains_key(&current_item.0){
                        extra_items.get_mut(&current_item.0).unwrap().add_assign(delta_rec);
                    }else {
                        extra_items.insert(current_item.0, delta_rec);
                    }
                } 
                for x in &a_steps {
                    steps.push((RecipeStep::Take(x.0, x.1 * times),deep + 1));
                }
                if items_to_craft.len() == rec.ingredients().len() - 1{
                    items_to_craft.push((0,0));
                }else {
                    items_to_craft.insert(rec.ingredients().len(), (0,0));
                }

                deep+=1;
                continue;
            } else {
                // If no recipe exists for the item, buy it directly
                steps.push((RecipeStep::Buy(current_item.0,current_item.1), deep));
            }
        }

    }
    fn show_recipe_mode(&mut self,menu: &mut ConsoleMenu, market: &crate::market_structs::AucPriceList){
        for x in 0..menu.menu_h() {
            menu.clear(x, ' ', ConsoleMenusPosition::Left);
        }
        if self.index == -1 || self.index>= RECIPES.len() as i32 {
            menu.write_in_middle(menu.menu_h()/2, "No recipe currently selected", 0, '\0', ConsoleMenusPosition::Left);
            return;
        }
        let rec = &RECIPES[self.index as usize];
        let str = &ITEMS[rec.result().index()];
        menu.write_in_middle(0, &str.name_stat(), 0, '\0', ConsoleMenusPosition::Left);
        //menu.write_in_middle(1, format!("Count:{}   Sell price:{}   C*SP:{}",rec.result().count(),str.sell_price(),str.sell_price() * rec.result().count()).as_str(), 0, '\0', ConsoleMenusPosition::Left);
        menu.write_in_middle(1, "Ingredients", 0, '\0', ConsoleMenusPosition::Left);
        menu.write_divided_into_rows(2, '\0', &["Name","Count","Sell price"],ConsoleMenusPosition::Left);
        let ings: Vec<(&Item,u32)> = rec.ingredients().iter().map(|x| (&ITEMS[x.index()],x.count())).collect();
        let mut i = 0;
        let mut input_price: i32 = 0;
        for x in ings {
            let item = x.0;
            let count = x.1;
            if input_price != -1{
                input_price += (item.sell_price() * count) as i32;
                if item.sell_price() == 0 {
                    input_price = -1;
                }
            }

            let count_str = count.to_string();
            let sell_str = item.sell_price().to_string();

            let details = vec![
                item.name(),
                &count_str,
                &sell_str,
            ];
            menu.write_divided_into_rows(i + 3, '\0', details.as_slice(),  ConsoleMenusPosition::Left);
            i+=1;
        }
        let price_result = (str.sell_price() * rec.result().count()) as i32;
        menu.write_in_middle(i + 4, "Stats", 0,'\0',ConsoleMenusPosition::Left);
        menu.write_line(i+5, format!("Production:           {} {}(s)",rec.result().count(),str.name()).as_str(), 0, '\0', ConsoleMenusPosition::Left);
        menu.write_line(i+6, format!("Price per 1:          {} rubles",str.sell_price()).as_str(), 0, '\0', ConsoleMenusPosition::Left);
        menu.write_line(i+7, format!("Price result:         {} rubles", price_result).as_str(), 0, '\0', ConsoleMenusPosition::Left);
        menu.write_line(i+8, format!("Delta prod:           {} rubles",(price_result as i32) - (input_price as i32)).as_str(), 0, '\0', ConsoleMenusPosition::Left);
        if input_price == -1{
            menu.write_line(i+8, "Delta price: components not sellable", 0, '\0', ConsoleMenusPosition::Left);
        }
        menu.write_line(i+9, format!("Energy:                {} u",rec.energy_count()).as_str(), 0, '\0', ConsoleMenusPosition::Left);
        menu.write_line(i+10, format!("Price per EU:         {} rubles",market.price_per_energy_unit()).as_str(), 0, '\0', ConsoleMenusPosition::Left);
        let energy_price = (market.price_per_energy_unit() * rec.energy_count() as f32).ceil() as i32;
        menu.write_line(i+11, format!("Energy price:         {} rubles",energy_price).as_str(), 0, '\0', ConsoleMenusPosition::Left);
        menu.write_line(i+12, format!("Production delta:     {} rubles",((price_result as i32) - (input_price as i32)) - energy_price).as_str(), 0, '\0', ConsoleMenusPosition::Left);
    }
}
impl Tab for RecipesTab {
    fn update(&self) -> bool {
        self.update
    }
    
    fn show(&mut self,menu: &mut ConsoleMenu, market: &crate::market_structs::AucPriceList) {
        if self.mode == 0{
            self.show_recipe_mode(menu, market);
            return;
        }
        for x in 0..menu.menu_h() {
            menu.clear(x, ' ', ConsoleMenusPosition::Left);
        }
        if self.index == -1 || self.index>= RECIPES.len() as i32 {
            menu.write_in_middle(menu.menu_h()/2, "No recipe currently selected", 0, '\0', ConsoleMenusPosition::Left);
            return;
        }

        let rec = &RECIPES[self.index as usize];
        let str = &ITEMS[rec.result().index()];
        menu.write_in_middle(0, &str.name_stat(), 0, '\0', ConsoleMenusPosition::Left);
        menu.write_in_middle(1, format!("Goal:{} {}",self.number,str.name()).as_str(), 0, '\0', ConsoleMenusPosition::Left);
        menu.write_in_middle(2, "Craft tree", 0, '\0', ConsoleMenusPosition::Left);
        //menu.write_divided_into_rows(2, '\0', &["Name","Count","Sell price"],ConsoleMenusPosition::Left);
        let mut i = 0;

        let mut delta: HashMap<usize,u32> = HashMap::new();
        let mut buy: Vec<(usize, i32)> = vec![];

        let mut steps = vec![];
        RecipesTab::recept_solver((rec.result().index() as usize,self.number),&mut steps,&mut delta);
        for x in steps.iter() {
            let r = match x {
                (RecipeStep::Buy(i, a),d) => {
                    let mut spaces = String::new();
                    for _ in 0..*d {
                        spaces += " ";
                    }
                    format!("{}Buy:{} \"{}\"",spaces,a ,ITEMS[*i].name())
                }
                (RecipeStep::Craft(i, a),d) => {
                    let mut spaces = String::new();
                    for _ in 0..*d {
                        spaces += " ";
                    }
                    format!("{}Craft:\"{}\" {} times", spaces, ITEMS[*i].name(), a)
                }
                (RecipeStep::Take(i, a),d) => {
                    let mut spaces = String::new();
                    for _ in 0..*d {
                        spaces += " ";
                    }
                    format!("{}Take:{} \"{}\"",spaces,a ,ITEMS[*i].name())
                }
                (RecipeStep::TakeRem(i, a),d) => {
                    let mut spaces = String::new();
                    for _ in 0..*d {
                        spaces += " ";
                    }
                    format!("{}Take remaining:{} \"{}\"",spaces,a ,ITEMS[*i].name())
                }
                
            };
            menu.write_line(i + 3, &r, 0, '\0',  ConsoleMenusPosition::Left);
            i+=1;
        }
        menu.write_in_middle(i + 3, "Excess", 0, '\0',  ConsoleMenusPosition::Left);
        let mut offset = i + 4;
        for x in delta {
            let name = &ITEMS[x.0];
            let a = format!("{} {}",name.name(),x.1);
            menu.write_line(offset, &a, 0, '\0',  ConsoleMenusPosition::Left);
            offset+=1;
        }
        menu.write_in_middle(0, "Statistics", 0, '-',  ConsoleMenusPosition::Right);
        menu.write_line(1, format!("Complexity:    {}",steps.len()).as_str(), 0, '-',  ConsoleMenusPosition::Right);
        //menu.write_line(2, format!("Energy usage:  {}",stats.0).as_str(), 0, '-',  ConsoleMenusPosition::Right);
    }
    fn input(&mut self, x: &str) {
        self.update = true;
        match x.chars().next() {
            Some('m') =>{
                self.mode = 1 - self.mode;
            }
            Some('n') =>{
                if let Ok(number) = x.chars().skip(2).collect::<String>().parse::<u32>(){
                    self.number = number;
                }
            }
            Some('i') | Some('s') =>{
                if let Ok(number) = x.chars().skip(2).collect::<String>().parse::<u32>(){
                    self.index = number as i32;
                }else {
                    let str = x.chars().skip(2).collect::<String>();
                    let a = ITEMS.iter().map(|x| (normalized_distance(x.name(), &str),x.name())).min_by(|&a, &b| {
                        let dist_a = normalized_distance(&str, a.1);
                        let dist_b = normalized_distance(&str, b.1);
                        dist_a.partial_cmp(&dist_b).unwrap()
                    }).unwrap();
                    if let Some(x) = ITEMS.iter().enumerate().find(|x| x.1.name() == a.1){
                        if let Some(x) = RECIPES.iter().enumerate().find(|y| y.1.result().index() == x.0) {
                            self.index = x.0 as i32;
                        }
                    }
                }
            }
            _ =>{self.update = false;}
        }
    }
}