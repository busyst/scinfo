use std::collections::HashMap;

pub struct AucPriceList{
                // Name  (item, avg weekly, monthly, mod weekly, monthly )
    list: HashMap<String,(String,i32,i32,i32,i32,)>
    
}

impl AucPriceList {
    pub fn new(list: HashMap<String,(String,i32,i32,i32,i32,)>) -> Self {
        Self { list }
    }
    
    pub fn list(&self) -> &HashMap<String,(String,i32,i32,i32,i32,)> {
        &self.list
    }
    pub fn price_per_energy_unit(&self) -> f32{ // Gas placeholder price
        8000.0/2000.0
    }
}