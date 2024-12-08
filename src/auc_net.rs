use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{self, Write};
use reqwest::StatusCode;
use chrono::{DateTime, Utc};

use crate::item_list::ITEMS;

pub async fn auction_scraper_show() {
    clear_screen();
    println!("You are about to get Stalcraft's auction history.");
    
    let paths = match fs::read_dir("./") {
        Ok(paths) => paths,
        Err(e) => {
            eprintln!("Error reading directory: {}", e);
            return;
        }
    };
    
    let mut valid_paths: Vec<String> = Vec::new();
    
    println!("Searching for fallback files");
    
    for path in paths {
        let path = match path {
            Ok(p) => p,
            Err(e) => {
                eprintln!("Error processing path: {}", e);
                continue;
            }
        };
        
        let filename = path.file_name();
        let filename_str = filename.to_str().unwrap_or("");
        
        if filename_str.starts_with("au_") {
            if let Some(path_str) = path.path().to_str() {
                valid_paths.push(path_str.to_string());
                println!("Valid file: {} {}", valid_paths.len() - 1, filename_str);
            }
        }
    }
    let selected_file: Option<String>;
    match valid_paths.len() {
        0 => {
            println!("Valid files are not found (starts with au_)");
            selected_file = None
        },
        1 => {
            println!("Automatically selected this file: {}", valid_paths[0]);
            selected_file = Some(valid_paths[0].clone())
        },
        _ => {
            println!("Select a fallback file");
            println!("Write a number to select a numbered file");
            
            loop {
                print!("Enter your selection: ");
                io::stdout().flush().unwrap();
                
                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();
                let input = input.trim();
                
                match input {
                    num if num.parse::<usize>().is_ok() => {
                        let index = num.parse::<usize>().unwrap();
                        if index < valid_paths.len() {
                            selected_file = Some(valid_paths[index].clone());
                            break;
                        } else {
                            println!("Invalid file number. Please try again.");
                        }
                    },
                    _ => println!("Invalid input. Please enter a number'.")
                }
            }
        }
    };
    if let Some(x) = selected_file {
        println!("Selected file:{}",x);   
    }
    let regs_reqw= reqwest::get("https://eapi.stalcraft.net/regions").await.expect("huh");
    if regs_reqw.error_for_status().is_err(){
        println!("You recived error, probably you have no internet or you sent too many request in short time");
        return;
    }

    let mut region;
    loop {
        print!("Enter your region (ru,eu,na,sea): ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        region = input.trim().to_lowercase();
        match region.as_str() {
            "ru" | "eu" | "na" | "sea" => {break;}
            _ => {}
        }
    }
    
    let key ;
    loop {
        println!("Enter your autoriastion key: ");

        let mut input_k = String::new();
        io::stdin().read_line(&mut input_k).unwrap();
        input_k = input_k.trim().to_string().to_lowercase();
        println!("\"{}\"\nis your autorisaton key right? (Y/N/Leave)",input_k);

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input = input.trim().to_lowercase();
        match input.as_str() {
            "y" => {key = input_k;break;}
            "l" => {return;}
            _ => {}
        }
    }
    let client = reqwest::Client::builder()
    .default_headers({
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            reqwest::header::AUTHORIZATION, 
            reqwest::header::HeaderValue::from_str(&key).unwrap()
        );
        headers.insert(
            reqwest::header::CONTENT_TYPE, 
            reqwest::header::HeaderValue::from_static("application/json")
        );
        headers
    })
    .build()
    .expect("Failed to create HTTP client");
    let mut d: HashMap<String, (i64, i64, i64, i64, i64, i64)> = HashMap::new();
    let current_time = Utc::now();


    let mut file = File::create(format!("./au_{}",current_time).as_str()).expect("File was not created");
    let mut brk = false;
    for item in ITEMS {
        let mut index = 0;
        let mut lots: Vec<Lot> = vec![];
        loop {
            let url = format!("https://eapi.stalcraft.net/{}/auction/{}/history?offset={}&limit=200&additional=true",region,item.id(),index);
            let a = client.get(url.clone()).header("key", "value").build().expect("should be");
            let s = client.execute(a).await.expect("should be");
            if s.status() == StatusCode::UNAUTHORIZED{
                println!("Autorisation key is false. Aborting");
                return;
            } else if s.status() != StatusCode::OK{
                println!("Uncaught error. Aborting");
                return;
            }

            let resp = s.json::<serde_json::Value>().await.expect("msg");
            let total = resp.as_object().unwrap().get("total").unwrap().as_i64().unwrap();
            let prices = resp.as_object().unwrap().get("prices").unwrap().as_array().unwrap();
            for x in prices {
                if let Some(obj) = x.as_object() {
                    let count = obj.get("amount").unwrap().as_i64().unwrap() as i32;
                    let price = obj.get("price").unwrap().as_i64().unwrap();
                    let time = DateTime::parse_from_rfc3339(obj.get("time").unwrap().as_str().unwrap()).unwrap().with_timezone(&Utc);
                    let additional = obj.get("additional").unwrap().as_str().unwrap();
                    if (current_time - time).num_days() >= 30{
                        brk = true;
                        break;
                    }
                    lots.push(Lot::new(count, price, time, additional.to_string()));
                    index+=1;
                    //println!("{},{}x{},{},{}",url,amount,price,time,addit);
                }
            }
            println!("{},{},{}",url,total,prices.len());
            if brk{
                break;
            }
        }

        let mut prices_listm: HashMap<String, Vec<i64>> = HashMap::new();
        let mut prices_listw: HashMap<String, Vec<i64>> = HashMap::new();
    
        for lot in &lots {
            let id = item.id().to_string();
            let ts = current_time - lot.time;
            if ts.num_days() >= 30 {
                println!("{:?}", ts);
                continue;
            }
    
            d.entry(id.clone()).or_insert((0, 0, 0, 0, 0, 0));
            let price_per_one = (lot.price as f32/lot.count as f32).ceil() as i64;
            let entry = d.get_mut(&id).unwrap();
            *entry = (entry.0, entry.1 + price_per_one, entry.2, entry.3, entry.4, entry.5);
    
            prices_listm.entry(id.clone()).or_insert_with(Vec::new).push(price_per_one);
    
            if ts.num_days() >= 7 {
                continue;
            }
    
            let entry = d.get_mut(&id).unwrap();
            *entry = (entry.0 + price_per_one, entry.1, entry.2, entry.3, entry.4, entry.5);
            prices_listw.entry(id.clone()).or_insert_with(Vec::new).push(price_per_one);
        }
    
        println!("Med calc");
        for (id, value) in &mut d {
            let w = prices_listw.get(id).unwrap();
            let m = prices_listm.get(id).unwrap();
    
            if m.is_empty() {
                println!("{}", id);
                println!("use fallback prices");
                
                /*if && !fallback_list.iter().any(|y| y.0 == *id) {
                    println!("Alarm {} Not found in fallback list", id);
                    return;
                }
    
                let k = fallback_list.iter().find(|y| y.0 == *id).unwrap().1;
                *value = k;*/
                continue;
            }
    
            let mut w = w.clone();
            let mut m = m.clone();
            w.sort();
            m.sort();
    
            let w_med = if w.len() % 2 == 1 {
                w[w.len() / 2]
            } else {
                (w[w.len() / 2 - 1] + w[w.len() / 2]) / 2
            };
    
            let m_med = if m.len() % 2 == 1 {
                m[m.len() / 2]
            } else {
                (m[m.len() / 2 - 1] + m[m.len() / 2]) / 2
            };
    
            let w_avg = ((value.0 as f64) / (w.len() as f64)).ceil() as i64;
            let m_avg = ((value.1 as f64) / (m.len() as f64)).ceil() as i64;
    
            // Calculate lowest 25% median and average
            let w_low_25_idx = (w.len() as f64 * 0.25).floor() as usize;
            let m_low_25_idx = (m.len() as f64 * 0.25).floor() as usize;
    
            let w_low_25_median = if w.len() % 2 == 1 {
                w[w_low_25_idx]
            } else {
                (w[w_low_25_idx - 1] + w[w_low_25_idx]) / 2
            };
    
            let m_low_25_median = if m.len() % 2 == 1 {
                m[m_low_25_idx]
            } else {
                (m[m_low_25_idx - 1] + m[m_low_25_idx]) / 2
            };
    
            let w_low_25_avg = w[..=w_low_25_idx].iter().sum::<i64>() / (w_low_25_idx + 1) as i64;
            let m_low_25_avg = m[..=m_low_25_idx].iter().sum::<i64>() / (m_low_25_idx + 1) as i64;
    
            *value = (w_avg, m_avg, w_med, m_med, w_low_25_median, m_low_25_median);
            println!("{} {},{},{},{},{},{},{}", 
                id, w_avg, m_avg, w_med, m_med, w_low_25_median, w_low_25_avg, m_low_25_avg);
            let a = format!("{} {},{},{},{},{},{},{}\n", id, w_avg, m_avg, w_med, m_med, w_low_25_median, w_low_25_avg, m_low_25_avg);
            file.write(a.as_bytes()).expect("Failed to write to file");
        }
    }
    


}
#[derive(Clone)]
struct Lot {
    count: i32,
    price: i64,
    time: DateTime<Utc>,
    additional: String,
}

impl Lot {
    fn new(count: i32, price: i64, time: DateTime<Utc>, additional: String) -> Self {
        Self { count, price, time, additional }
    }
}
fn clear_screen() {
    print!("{esc}c", esc = 27 as char);
}