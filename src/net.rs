use chrono::DateTime;
use reqwest::StatusCode;

use crate::structs::{ClanInfo, Price, Region};


pub async fn get_list_of_regions() -> Result<Vec<Region>, Box<dyn std::error::Error>> {
    let regions: Vec<serde_json::Value> = reqwest::get("https://eapi.stalcraft.net/regions")
        .await?
        .json()
        .await?;

    let regions: Vec<Region> = regions
        .iter()
        .filter_map(|region| {
            let obj = region.as_object()?;
            Some(Region::new(
                obj.get("id")?.as_str()?.to_string().to_ascii_lowercase(),
                obj.get("name")?.as_str()?.to_string().to_ascii_lowercase(),
            ))
        })
        .collect();

    if regions.is_empty() {
        return Err("API returned no regions - possible API change".into());
    }

    Ok(regions)
}

pub async fn get_clans_list(region: &Region,auth: &str,limit: usize) -> Result<Vec<ClanInfo>,Box<dyn std::error::Error>>{
    let resp = reqwest::Client::new()
    .get(format!("https://dapi.stalcraft.net/{}/clans?offset={}&limit={}", region.id(),0, 0))
    .header("Authorization", auth)
    .header("Content-Type", "application/json")
    .send()
    .await?;
    if resp.status() == StatusCode::UNAUTHORIZED{
        return Err("App Access Token not valid".into());
    }
    let resp = resp.text().await?;

    let x: Vec<serde_json::Value> = serde_json::from_str(&resp)?;
    if x.len()!=1{
        panic!("They changed api, program may be unstable! Occured at get_clans_list. Expected 1 object got {}",x.len());
    }
    let x = x.first().unwrap().as_object().expect("Expected 1 object");
    let total_clans_count = x.get("totalClans").expect("Total class count not found").as_i64().unwrap_or(0);
    let clans_info = x.get("data").expect("Clans data was not found").as_array().unwrap();
    // TODO: Implement listing 
    let mut clan_list: Vec<ClanInfo> = vec![]; 
    for clan_data in clans_info {
        let clan_obj = clan_data.as_object().expect("Invalid clan data format");
    
        let clan = ClanInfo::new(
            clan_obj.get("id").and_then(|v| v.as_str()).unwrap_or("").to_string(),
            clan_obj.get("name").and_then(|v| v.as_str()).unwrap_or("").to_string(),
            clan_obj.get("tag").and_then(|v| v.as_str()).unwrap_or("").to_string(),
            clan_obj.get("level").and_then(|v| v.as_i64()).unwrap_or(0) as i32,
            clan_obj.get("level_points").and_then(|v| v.as_i64()).unwrap_or(0) as i32,
            clan_obj.get("registration_date").and_then(|v| v.as_u64()).unwrap_or(0),
            clan_obj.get("alliance").and_then(|v| v.as_str()).unwrap_or("").to_string(),
            clan_obj.get("description").and_then(|v| v.as_str()).unwrap_or("").to_string(),
            clan_obj.get("leader").and_then(|v| v.as_str()).unwrap_or("").to_string(),
            clan_obj.get("member_count").and_then(|v| v.as_i64()).unwrap_or(0) as i32
        );
        
        clan_list.push(clan);
        
    }
    
    return Ok(clan_list);
}

pub async fn get_item_price_history(region: &Region,item_id: &str,auth: &str,limit: usize) -> Result<Vec<Price>,Box<dyn std::error::Error>>{
    let resp = reqwest::Client::new()
    .get(format!("https://eapi.stalcraft.net/{}/auction/{}/history?limit={}&additional={}", region.id(),item_id,0,0))
    .header("Authorization", auth)
    .header("Content-Type", "application/json")
    .send()
    .await?;
    if resp.status() == StatusCode::UNAUTHORIZED{
        return Err("App Access Token not valid".into());
    }
    let resp = resp.text().await?;
    

    let x: Vec<serde_json::Value> = serde_json::from_str(&resp)?;
    if x.len()!=1{
        panic!("They changed api, program may be unstable! Occured at get_clans_list. Expected 1 object got {}",x.len());
    }
    let x = x.first().unwrap().as_object().expect("Expected 1 object");
    let total_entry_count = x.get("total").expect("Total item count not found").as_i64().unwrap_or(0);
    let prices_info = x.get("prices").expect("Prices data was not found").as_array().unwrap();
    // TODO: Implement listing 
    let mut price_list: Vec<Price> = vec![]; 
    for price_data in prices_info {
        let price_obj = price_data.as_object().expect("Invalid clan data format");
    

        let price = Price::new(
            price_obj.get("amount")
                .and_then(|v| v.as_u64())
                .unwrap_or(0) as i32,
            price_obj.get("price")
                .and_then(|v| v.as_u64())
                .unwrap_or(0) as i64,
            price_obj.get("time")
                .and_then(|v| v.as_str())
                .and_then(|time_str| DateTime::parse_from_rfc3339(time_str).ok())
                .map(|dt| dt.timestamp() as u64)
                .unwrap_or(0),
            price_obj.get("additional")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string()
        );
        
        price_list.push(price);
        
    }
    
    return Ok(price_list);
}