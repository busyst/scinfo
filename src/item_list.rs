pub enum Quality {
    Picklock,
    Newbie,
    Stalker,
    Veteran,
    Master,
    Legend,
}

pub const fn get_tags(q: Quality) -> u32 {
    match q {
        Quality::Picklock => 1 << 0,       // 2^0
        Quality::Newbie => 1 << 1,         // 2^1
        Quality::Stalker => 1 << 2,        // 2^2
        Quality::Veteran => 1 << 3,        // 2^3
        Quality::Master => 1 << 4,         // 2^4
        Quality::Legend => 1 << 5,         // 2^5
    }
 }
 
 pub const fn acquire_quality(n: u32) -> Quality {
    match n {
        x if x & (1 << 0) != 0 => Quality::Picklock,
        x if x & (1 << 1) != 0 => Quality::Newbie,
        x if x & (1 << 2) != 0 => Quality::Stalker,
        x if x & (1 << 3) != 0 => Quality::Veteran,
        x if x & (1 << 4) != 0 => Quality::Master,
        x if x & (1 << 5) != 0 => Quality::Legend,
        _ => Quality::Picklock, // Default/fallback case
    }
 }
 


pub const ITEMS: &[Item] = &[
    //Item::new("Water Carrier", "m22k", 30, 0),
    // ------------------ Leafs -------------------------------------------------------------------------------
    Item::new("Pale-Leaf", "", 5, get_tags(Quality::Picklock)), // Done 23/11/2024
    Item::new("Acid Nettle", "", 10, get_tags(Quality::Newbie)), // Done 23/11/2024
    Item::new("Water Carrier", "", 30, get_tags(Quality::Stalker)), // Done 23/11/2024
    Item::new("Gnosis", "", 70, get_tags(Quality::Veteran)), // Done 23/11/2024
    Item::new("Hellroot", "", 195, get_tags(Quality::Master)), // Done 23/11/2024
    Item::new("Amber Wormwood", "", 480, get_tags(Quality::Legend)), // Done 23/11/2024
    // ------------------ Minerals ----------------------------------------------------------------------------
    Item::new("Dim Minerals", "", 20, get_tags(Quality::Picklock)), // Done 23/11/2024
    Item::new("Emerald Minerals", "", 35, get_tags(Quality::Newbie)), // Done 23/11/2024
    Item::new("Azure Minerals", "", 65, get_tags(Quality::Stalker)), // Done 23/11/2024
    Item::new("Purple Minerals", "", 145, get_tags(Quality::Veteran)), // Done 23/11/2024
    Item::new("Scarlet Minerals", "", 345, get_tags(Quality::Master)), // Done 23/11/2024
    Item::new("Golden Minerals", "", 870, get_tags(Quality::Legend)), // Done 23/11/2024
    // ------------------ Pulps -------------------------------------------------------------------------------
    Item::new("Saltweed Pulp", "", 35, get_tags(Quality::Picklock)), // Done 23/11/2024
    Item::new("Mintfruit Pulp", "", 75, get_tags(Quality::Newbie)), // Done 23/11/2024
    Item::new("Sweettooth Pulp", "", 160, get_tags(Quality::Stalker)), // Done 23/11/2024
    Item::new("Spiritfruit Pulp", "", 280, get_tags(Quality::Veteran)), // Done 23/11/2024
    Item::new("Cubemelon Pulp", "", 580, get_tags(Quality::Master)), // Done 23/11/2024
    Item::new("Lemongrass Pulp", "", 1000, get_tags(Quality::Legend)), // Done 23/11/2024
    // ------------------ Food processing ---------------------------------------------------------------------
    Item::new("Flour", "", 6, get_tags(Quality::Newbie)), // Done 23/11/2024
    Item::new("Set of Spieces", "", 116, get_tags(Quality::Newbie)), // Done 23/11/2024
    Item::new("Yeast", "", 14, get_tags(Quality::Picklock)), // Done 23/11/2024
    Item::new("Bottle of Pure Water", "", 54, 0), // Done 23/11/2024
    Item::new("Dough", "", 80, get_tags(Quality::Newbie)), // Done 23/11/2024
    Item::new("Set of Spices", "", 116, get_tags(Quality::Newbie)), // Done 23/11/2024
    Item::new("Can of Pasta", "", 85, get_tags(Quality::Newbie)), // Done 23/11/2024
    Item::new("Animal Fat", "", 90, get_tags(Quality::Newbie)), // Done 23/11/2024
    Item::new("Pickles", "", 308, get_tags(Quality::Newbie)), // Done 23/11/2024
    Item::new("Tomato", "", 29, get_tags(Quality::Picklock)), // Done 23/11/2024
    Item::new("Garlic", "", 50, get_tags(Quality::Picklock)), // Done 23/11/2024
    Item::new("Pureed Vegetables", "", 86, get_tags(Quality::Newbie)), // Done 23/11/2024
    Item::new("Sausage Slices", "", 0, get_tags(Quality::Stalker)), // Done 27/11/2024
    Item::new("Meat Preserves", "", 0, get_tags(Quality::Stalker)), // Done 27/11/2024
    Item::new("Excellent Canned Meat", "", 0, get_tags(Quality::Stalker)), // Done 27/11/2024
    Item::new("Navy-Style Pasta", "", 0, get_tags(Quality::Stalker)), // Done 27/11/2024
    // ------------------ Meats -------------------------------------------------------------------------------
    Item::new("Boar Meat", "", 285, get_tags(Quality::Picklock)), // Done 23/11/2024
    Item::new("Ground Boar Meat", "", 171, get_tags(Quality::Newbie)), // Done 23/11/2024
    Item::new("Mutt Meat", "", 285, get_tags(Quality::Picklock)), // Done 23/11/2024
    Item::new("Ground Dog Meat", "", 171, get_tags(Quality::Newbie)), // Done 23/11/2024
    Item::new("Piggy Meat", "", 285, get_tags(Quality::Picklock)), // Done 23/11/2024
    Item::new("Ground Piggy Meat", "", 171, get_tags(Quality::Newbie)), // Done 23/11/2024
    Item::new("Fish Fillet", "", 138, get_tags(Quality::Newbie)), // Done 23/11/2024
    // ------------------ Moonshining -------------------------------------------------------------------------
    Item::new("Juice", "", 62, get_tags(Quality::Newbie)), // Done 23/11/2024
    Item::new("Anomalous Yeast", "", 45, get_tags(Quality::Newbie)), // Done 23/11/2024
    Item::new("Braga", "", 387, get_tags(Quality::Newbie)), // Done 23/11/2024
    Item::new("Industrial Alcohol", "", 38, get_tags(Quality::Newbie)), // Done 23/11/2024
    Item::new("Wort", "", 54, get_tags(Quality::Newbie)), // Done 23/11/2024
    Item::new("Base Wine", "", 85, get_tags(Quality::Newbie)), // Done 23/11/2024
    Item::new("Tirage Liqueur", "", 79, get_tags(Quality::Newbie)), // Done 23/11/2024
    Item::new("Wine with Sediment", "", 380, get_tags(Quality::Newbie)), // Done 23/11/2024
    Item::new("Children's Champagne", "", 0, get_tags(Quality::Newbie)), // Done 30/11/2024
    Item::new("Sparkling Champagne", "", 0, get_tags(Quality::Veteran)), // Done 30/11/2024
    Item::new("Anomalous Champagne", "", 0, get_tags(Quality::Veteran)), // Done 30/11/2024
    // ------------------ <Woodworking> -----------------------------------------------------------------------
    Item::new("Rotten Boards", "", 90, get_tags(Quality::Picklock)),
    Item::new("Boards", "", 13, get_tags(Quality::Newbie)),
    // ------------------ <Misc> ------------------------------------------------------------------------------
    Item::new("Plastic Bottle", "", 20, get_tags(Quality::Picklock)), // Done 23/11/2024
    Item::new("Half-rotten Fish", "", 200, get_tags(Quality::Picklock)), // Done 23/11/2024
    Item::new("Thermal Mixture", "", 63, get_tags(Quality::Newbie)), // Done 23/11/2024
    Item::new("Oil", "", 880, get_tags(Quality::Picklock)), // Done 27/11/2024
    Item::new("Reagents", "", 35, get_tags(Quality::Newbie)), // Done 27/11/2024
    Item::new("Petroleum Coke", "", 148, get_tags(Quality::Newbie)), // Done 27/11/2024
    Item::new("Iron", "", 522, get_tags(Quality::Newbie)), // Done 27/11/2024
    Item::new("Tin Can", "", 60, get_tags(Quality::Picklock)), // Done 27/11/2024
    Item::new("Anomalous Dust", "", 250, get_tags(Quality::Stalker)), // Done 30/11/2024
    Item::new("Changedust", "", 1250, get_tags(Quality::Veteran)), // Done 30/11/2024
    Item::new("Glowing Sugar", "", 3663, get_tags(Quality::Stalker)), // Done 30/11/2024
];
pub const fn item_index(x: &'static str) -> usize {
    let mut i = 0;
    while i < ITEMS.len() {
        if const_str::equal!(ITEMS[i].name_stat(), x){
            return i;
        }
        i += 1;
    }

    panic!("{}", x)
}


pub const RECIPES: &[Recipe] = &[
    // ------------------ <Cooking> ---------------------------------------------------------------------------
    Recipe {  // Done 23/11/2024        Result "Bottle of Pure Water"
        ingredients: &[
            Ingredient { index: item_index("Plastic Bottle"), count: 5 },
            Ingredient { index: item_index("Water Carrier"), count: 10 },
        ],
        result: &Ingredient { index: item_index("Bottle of Pure Water"), count: 5 },
        energy_count: 100,
        level_reward: LevelReward::Cooking(10)
    },
    Recipe {  // Done 23/11/2024        Result "Fish Fillet"
        ingredients: &[
            Ingredient { index: item_index("Half-rotten Fish"), count: 1 },
            Ingredient { index: item_index("Water Carrier"), count: 10 },
        ],
        result: &Ingredient { index: item_index("Fish Fillet"), count: 4 },
        energy_count: 100,
        level_reward: LevelReward::Cooking(10)
    },
    Recipe {  // Done 23/11/2024        Result "Flour"
        ingredients: &[
            Ingredient { index: item_index("Pale-Leaf"), count: 5 },
        ],
        result: &Ingredient { index: item_index("Flour"), count: 7 },
        energy_count: 100,
        level_reward: LevelReward::Cooking(10)
    },
    Recipe { // Done 23/11/2024        Result "Set of Spices"
        ingredients: &[ 
            Ingredient { index: item_index("Saltweed Pulp"), count: 6 },
            Ingredient { index: item_index("Pale-Leaf"), count: 10 },
            Ingredient { index: item_index("Acid Nettle"), count: 7 },
            Ingredient { index: item_index("Sweettooth Pulp"), count: 6 },
        ],
        result: &Ingredient { index: item_index("Set of Spices"), count: 10 },
        energy_count: 100,
        level_reward: LevelReward::Cooking(10)
    },
    Recipe { // Done 23/11/2024        Result "Dough"
        ingredients: &[ 
            Ingredient { index: item_index("Yeast"), count: 5 },
            Ingredient { index: item_index("Flour"), count: 3 },
            Ingredient { index: item_index("Bottle of Pure Water"), count: 3 },
        ],
        result: &Ingredient { index: item_index("Dough"), count: 3 },
        energy_count: 300,
        level_reward: LevelReward::Cooking(30)
    },
    Recipe { // Done 23/11/2024        Result "Ground Boar Meat"
        ingredients: &[ 
            Ingredient { index: item_index("Boar Meat"), count: 2 },
            Ingredient { index: item_index("Set of Spices"), count: 5 },
        ],
        result: &Ingredient { index: item_index("Ground Boar Meat"), count: 5 },
        energy_count: 300,
        level_reward: LevelReward::Cooking(30)
    },
    Recipe { // Done 23/11/2024        Result "Ground Dog Meat"
        ingredients: &[ 
            Ingredient { index: item_index("Mutt Meat"), count: 2 },
            Ingredient { index: item_index("Set of Spices"), count: 5 },
        ],
        result: &Ingredient { index: item_index("Ground Boar Meat"), count: 5 },
        energy_count: 300,
        level_reward: LevelReward::Cooking(30)
    },
    Recipe { // Done 23/11/2024        Result "Ground Piggy Meat"
        ingredients: &[ 
            Ingredient { index: item_index("Piggy Meat"), count: 2 },
            Ingredient { index: item_index("Set of Spices"), count: 5 },
        ],
        result: &Ingredient { index: item_index("Ground Piggy Meat"), count: 5 },
        energy_count: 300,
        level_reward: LevelReward::Cooking(30)
    },
    Recipe { // Done 23/11/2024        Result "Animal Fat"
        ingredients: &[
            Ingredient { index: item_index("Boar Meat"), count: 1 },
        ],
        result: &Ingredient { index: item_index("Animal Fat"), count: 3 },
        energy_count: 100,
        level_reward: LevelReward::Cooking(10)
    },
    Recipe { // Done 23/11/2024        Result "Animal Fat"
        ingredients: &[
            Ingredient { index: item_index("Piggy Meat"), count: 1 },
        ],
        result: &Ingredient { index: item_index("Animal Fat"), count: 3 },
        energy_count: 100,
        level_reward: LevelReward::Cooking(10)
    },
    Recipe { // Done 23/11/2024        Result "Animal Fat"
        ingredients: &[
            Ingredient { index: item_index("Mutt Meat"), count: 1 },
        ],
        result: &Ingredient { index: item_index("Animal Fat"), count: 3 },
        energy_count: 100,
        level_reward: LevelReward::Cooking(10)
    },
    // Nobody uses fried meats, just give up
    Recipe { // Done 23/11/2024        Result "Pickles"
        ingredients: &[
            Ingredient { index: item_index("Saltweed Pulp"), count: 8 },
            Ingredient { index: item_index("Pale-Leaf"), count: 1 },
            Ingredient { index: item_index("Tomato"), count: 1 },
            Ingredient { index: item_index("Sweettooth Pulp"), count: 3 },
        ],
        result: &Ingredient { index: item_index("Pickles"), count: 3 },
        energy_count: 100,
        level_reward: LevelReward::Cooking(10)
    },
    Recipe { // Done 23/11/2024        Result "Pureed Vegetables"
        ingredients: &[
            Ingredient { index: item_index("Tomato"), count: 10 },
            Ingredient { index: item_index("Garlic"), count: 2 },
            Ingredient { index: item_index("Pale-Leaf"), count: 10 },
            Ingredient { index: item_index("Acid Nettle"), count: 10 },
        ],
        result: &Ingredient { index: item_index("Pureed Vegetables"), count: 5 },
        energy_count: 100,
        level_reward: LevelReward::Cooking(10)
    },
    Recipe { // Done 23/11/2024        Result "Can of Pasta"
        ingredients: &[
            Ingredient { index: item_index("Dough"), count: 3 },
            Ingredient { index: item_index("Flour"), count: 1 },
            Ingredient { index: item_index("Bottle of Pure Water"), count: 1 },
            Ingredient { index: item_index("Thermal Mixture"), count: 1 },
        ],
        result: &Ingredient { index: item_index("Can of Pasta"), count: 7 },
        energy_count: 300,
        level_reward: LevelReward::Cooking(30)
    },
    Recipe { // Done 27/11/2024        Result "Meat Preserves"
        ingredients: &[
            Ingredient { index: item_index("Tin Can"), count: 5 },
            Ingredient { index: item_index("Ground Dog Meat"), count: 1 },
            Ingredient { index: item_index("Ground Piggy Meat"), count: 1 },
        ],
        result: &Ingredient { index: item_index("Meat Preserves"), count: 5 },
        energy_count: 400,
        level_reward: LevelReward::Cooking(40)
    },
    Recipe { // Done 27/11/2024        Result "Excellent Canned Meat"
        ingredients: &[
            Ingredient { index: item_index("Tin Can"), count: 3 },
            Ingredient { index: item_index("Set of Spices"), count: 1 },
            Ingredient { index: item_index("Animal Fat"), count: 1 },
            Ingredient { index: item_index("Meat Preserves"), count: 1 },
            Ingredient { index: item_index("Sausage Slices"), count: 1 },
        ],
        result: &Ingredient { index: item_index("Excellent Canned Meat"), count: 4 },
        energy_count: 600,
        level_reward: LevelReward::Cooking(60)
    },
    Recipe { // Done 27/11/2024        Result "Navy-Style Pasta"
        ingredients: &[
            Ingredient { index: item_index("Can of Pasta"), count: 1 },
            Ingredient { index: item_index("Bottle of Pure Water"), count: 1 },
            Ingredient { index: item_index("Set of Spices"), count: 1 },
            Ingredient { index: item_index("Excellent Canned Meat"), count: 2 },
        ],
        result: &Ingredient { index: item_index("Navy-Style Pasta"), count: 3 },
        energy_count: 800,
        level_reward: LevelReward::Cooking(80)
    },
    // ------------------ <Moonshining> -----------------------------------------------------------------------
    Recipe { // Done 30/11/2024        Result "Anomalous Yeast"
        ingredients: &[
            Ingredient { index: item_index("Yeast"), count: 10 },
            Ingredient { index: item_index("Anomalous Dust"), count: 1 },
            Ingredient { index: item_index("Changedust"), count: 1 },
        ],
        result: &Ingredient { index: item_index("Anomalous Yeast"), count: 13 },
        energy_count: 100,
        level_reward: LevelReward::Moonshining(10)
    },
    Recipe { // Done 23/11/2024        Result "Juice"
        ingredients: &[ 
            Ingredient { index: item_index("Bottle of Pure Water"), count: 2 },
            Ingredient { index: item_index("Sweettooth Pulp"), count: 4 },
            Ingredient { index: item_index("Cubemelon Pulp"), count: 2 },
        ],
        result: &Ingredient { index: item_index("Juice"), count: 13 },
        energy_count: 100,
        level_reward: LevelReward::Moonshining(10)
    },
    Recipe { // Done 23/11/2024        Result "Braga"
        ingredients: &[
            Ingredient { index: item_index("Spiritfruit Pulp"), count: 5 },
            Ingredient { index: item_index("Cubemelon Pulp"), count: 5 },
        ],
        result: &Ingredient { index: item_index("Braga"), count: 10 },
        energy_count: 100,
        level_reward: LevelReward::Moonshining(10)
    },
    Recipe { // Done 23/11/2024        Result "Industrial Alcohol"
        ingredients: &[
            Ingredient { index: item_index("Yeast"), count: 5 },
            Ingredient { index: item_index("Wort"), count: 1 },
            Ingredient { index: item_index("Spiritfruit Pulp"), count: 3 },
        ],
        result: &Ingredient { index: item_index("Industrial Alcohol"), count: 11 },
        energy_count: 400,
        level_reward: LevelReward::Moonshining(40)
    },
    Recipe { // Done 23/11/2024        Result "Wort"
        ingredients: &[
            Ingredient { index: item_index("Flour"), count: 1 },
            Ingredient { index: item_index("Bottle of Pure Water"), count: 1 },
        ],
        result: &Ingredient { index: item_index("Wort"), count: 1 },
        energy_count: 300,
        level_reward: LevelReward::Moonshining(30)
    },
    Recipe { // Done 23/11/2024        Result "Base Wine"
        ingredients: &[
            Ingredient { index: item_index("Yeast"), count: 5 },
            Ingredient { index: item_index("Braga"), count: 2 },
            Ingredient { index: item_index("Bottle of Pure Water"), count: 2 },
            Ingredient { index: item_index("Spiritfruit Pulp"), count: 5 },
        ],
        result: &Ingredient { index: item_index("Base Wine"), count: 10 },
        energy_count: 300,
        level_reward: LevelReward::Moonshining(30)
    },
    Recipe { // Done 23/11/2024        Result "Tirage Liqueur"
        ingredients: &[
            Ingredient { index: item_index("Base Wine"), count: 5 },
            Ingredient { index: item_index("Anomalous Yeast"), count: 5 },
            Ingredient { index: item_index("Mintfruit Pulp"), count: 10 },
        ],
        result: &Ingredient { index: item_index("Tirage Liqueur"), count: 6 },
        energy_count: 400,
        level_reward: LevelReward::Moonshining(40)
    },
    Recipe { // Done 23/11/2024        Result "Wine with Sediment"
        ingredients: &[
            Ingredient { index: item_index("Base Wine"), count: 5 },
            Ingredient { index: item_index("Tirage Liqueur"), count: 10 },
        ],
        result: &Ingredient { index: item_index("Wine with Sediment"), count: 6 },
        energy_count: 600,
        level_reward: LevelReward::Moonshining(60)
    },
    Recipe { // Done 30/11/2024        Result "Children's Champagne"
        ingredients: &[
            Ingredient { index: item_index("Juice"), count: 4 },
            Ingredient { index: item_index("Bottle of Pure Water"), count: 3 },
            Ingredient { index: item_index("Braga"), count: 2 },
            Ingredient { index: item_index("Glowing Sugar"), count: 1 },
        ],
        result: &Ingredient { index: item_index("Children's Champagne"), count: 5 },
        energy_count: 400,
        level_reward: LevelReward::Moonshining(40)
    },
    Recipe { // Done 30/11/2024        Result "Sparkling Champagne"
        ingredients: &[
            Ingredient { index: item_index("Base Wine"), count: 5 },
            Ingredient { index: item_index("Anomalous Yeast"), count: 1 },
            Ingredient { index: item_index("Mintfruit Pulp"), count: 7 },
            Ingredient { index: item_index("Sweettooth Pulp"), count: 15 },
            Ingredient { index: item_index("Wine with Sediment"), count: 2 },
        ],
        result: &Ingredient { index: item_index("Sparkling Champagne"), count: 6 },
        energy_count: 700,
        level_reward: LevelReward::Moonshining(70)
    },
    Recipe { // Done 30/11/2024        Result "Anomalous Champagne"
        ingredients: &[
            Ingredient { index: item_index("Anomalous Yeast"), count: 1 },
            Ingredient { index: item_index("Glowing Sugar"), count: 1 },
            Ingredient { index: item_index("Sweettooth Pulp"), count: 4 },
            Ingredient { index: item_index("Cubemelon Pulp"), count: 10 },
        ],
        result: &Ingredient { index: item_index("Anomalous Champagne"), count: 5 },
        energy_count: 900,
        level_reward: LevelReward::Moonshining(90)
    },
    // ------------------ <Lab> ---------------------------------------------------------------------------
    Recipe { // Done 27/11/2024        Result "Bottle of Pure Water"
        ingredients: &[
            Ingredient { index: item_index("Plastic Bottle"), count: 1 },
        ],
        result: &Ingredient { index: item_index("Bottle of Pure Water"), count: 1 },
        energy_count: 200,
        level_reward: LevelReward::None
    },
    Recipe { // Done 27/11/2024        Result "Plastic Bottle"
        ingredients: &[
            Ingredient { index: item_index("Bottle of Pure Water"), count: 1 },
        ],
        result: &Ingredient { index: item_index("Plastic Bottle"), count: 1 },
        energy_count: 100,
        level_reward: LevelReward::None
    },
    Recipe { // Done 27/11/2024        Result "Petroleum Coke"
        ingredients: &[
            Ingredient { index: item_index("Oil"), count: 1 },
            Ingredient { index: item_index("Reagents"), count: 3 },
        ],
        result: &Ingredient { index: item_index("Petroleum Coke"), count: 6 },
        energy_count: 100,
        level_reward: LevelReward::RawMaterials(10)
    },
    // ------------------ <Workbench> -------------------------------------------------------------------------
    Recipe { // Done 27/11/2024        Result "Tin Can"
        ingredients: &[
            Ingredient { index: item_index("Iron"), count: 1 },
        ],
        result: &Ingredient { index: item_index("Tin Can"), count: 3 },
        energy_count: 100,
        level_reward: LevelReward::Engeneering(10)
    },
    Recipe { // Done 27/11/2024        Result "Boards"
        ingredients: &[
            Ingredient { index: item_index("Rotten Boards"), count: 1 },
        ],
        result: &Ingredient { index: item_index("Boards"), count: 2 },
        energy_count: 10,
        level_reward: LevelReward::Engeneering(10)
    },
];


#[derive(Copy, Clone)]
pub struct Item {
    name: &'static str,
    id: &'static str,
    sell_price: u32,
    tags: u32,
}
impl Item {
    const fn new(name: &'static str, id: &'static str, sell_price: u32, tags: u32) -> Self {
        Self { name, id, sell_price, tags }
    }
    pub const  fn name_stat(&self) -> &'static str {
        self.name
    }
    pub fn name(&self) -> &str {
        self.name
    }
    
    pub fn id(&self) -> &str {
        self.id
    }
    
    pub fn sell_price(&self) -> u32 {
        self.sell_price
    }
    
    pub fn tags(&self) -> u32 {
        self.tags
    }
}

#[derive(Copy, Clone)]
pub struct Ingredient {
   index: usize,
   count: u32,
}

impl Ingredient {
    pub fn index(&self) -> usize {
        self.index
    }
    pub fn count(&self) -> u32 {
        self.count
    }
}

#[derive(Clone)]
pub struct Recipe {
   ingredients: &'static [Ingredient],
   result: &'static Ingredient,
   energy_count: u32,
   level_reward: LevelReward,
}

impl Recipe {
    pub fn ingredients(&self) -> &[Ingredient] {
        self.ingredients
    }
    
    pub fn result(&self) -> &'static Ingredient {
        self.result
    }
    
    pub fn energy_count(&self) -> u32 {
        self.energy_count
    }
    
    pub fn level_reward(&self) -> &LevelReward {
        &self.level_reward
    }
    pub fn recipe_name(&self) ->&'static str{
        ITEMS[self.result.index].name
    }
}

#[derive(Clone)]
pub enum LevelReward {
    Cooking(u32),
    Moonshining(u32),
    RawMaterials(u32),
    Engeneering(u32),
    None,
}