use std::collections::HashMap;
pub struct PriceSection{
    avg_weekly : usize,
    median_weekly : usize,
    avg_monthly : usize,
    median_monthly : usize,
}

impl PriceSection {
    pub fn avg_weekly(&self) -> usize {
        self.avg_weekly
    }
    
    pub fn median_weekly(&self) -> usize {
        self.median_weekly
    }
    
    pub fn avg_monthly(&self) -> usize {
        self.avg_monthly
    }
    
    pub fn median_monthly(&self) -> usize {
        self.median_monthly
    }
}
pub struct AucPriceList{
                // Name  (item, avg weekly, monthly, mod weekly, monthly )
    list: HashMap<usize,PriceSection>
    
}

impl AucPriceList {
    pub fn new(list: HashMap<usize,PriceSection>) -> Self {
        Self { list }
    }
    
    pub fn list(&self) -> &HashMap<usize,PriceSection> {
        &self.list
    }
    pub fn price_per_energy_unit(&self) -> f32{ // Gas placeholder price
        8000.0/2000.0
    }
}