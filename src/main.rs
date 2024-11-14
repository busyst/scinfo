use std::{fmt, fs::File, io::{stdin, Write}, path::Path};

mod structs;
mod net;

struct ConsoleMenu {
    width: usize,
    height: usize,
    frame: Vec<char>,
    menu_h: usize,
}
fn colored(r: i32, g: i32, b: i32, text: &str) -> String {
    return format!("\x1B[38;2;{};{};{}m{}\x1B[0m", r, g, b, text);
}
impl ConsoleMenu {
    fn empty() -> Self {
        ConsoleMenu {
            width: 0,
            height: 0,
            frame: vec![],
            menu_h: 0,
        }
    }
    fn left_menu_w(&self) -> usize{ self.width / 2 }
    fn right_menu_w(&self) -> usize{ (self.width / 2) - if self.width % 2 == 0 { 1 } else { 0 } }
    pub fn set_size(&mut self,width: usize,height: usize){
        let line_width = width + 1;
        let frame = vec![' '; line_width * height];
        let menu_h = height - 4;

        self.frame = frame;
        self.menu_h = menu_h;
        self.width = width;
        self.height = height;
        
    }
    pub fn re_render(&mut self){
        let lmw = self.left_menu_w();
        let frame = &mut self.frame;
        let height = self.height;
        let width = self.width;
        let line_width = self.width + 1;



        for y in 0..height {
            frame[(y + 1) * line_width - 1] = '\n';
            for x in 0..width {
                let mut c = ' ';
                if y == 0 || y == 2 || y == height - 1 {
                    c = '-';
                } else if y == 1 {
                    if x < width / 5 || x > 4 * width / 5 {
                        c = '=';
                    } else {
                        let index = (x as i32) - width as i32 / 2 + ("George".len() as i32 + 0) / 2;
                        c = if index >= 0 && index < "George".len() as i32 {
                            "George".chars().nth(index as usize).unwrap()
                        } else if index < 0 {
                            '>'
                        } else {
                            '<'
                        };
                    }
                } else if y >= 3 && x == width / 2 {
                    c = '|';
                } else if x < lmw {
                    c = '#';
                } else if x > width / 2 && x < width {
                    c = '&';
                }
                frame[x + line_width * y] = c;
            }
        }
    }

    fn write_line(&mut self, l: usize, text: &str, offset: usize, empty_fill: char, _trunkate: usize) {
        if l > self.menu_h {
            return;
        }
        for i in 0..self.left_menu_w() {
            if i < text.len() {
                self.frame[i + (self.width + 1) * (l + 3)] = text.chars().nth(i).unwrap();
            } else {
                self.frame[i + (self.width + 1) * (l + 3)] = empty_fill;
            }
        }
    }

    fn write_in_middle(&mut self, l: usize, text: &str, empty_fill: char, _trunkate: usize) {
        if l > self.menu_h {
            return;
        }
        let start = (self.left_menu_w() as i32 - text.len() as i32) / 2;
        for i in 0..self.left_menu_w() {
            if (i as i32 - start as i32) >= 0 && (i as i32 - start as i32) < text.len() as i32 {
                self.frame[i + (self.width + 1) * (l + 3)] = text.chars().nth((i as i32 - start) as usize).unwrap();
            } else if empty_fill != '\0' {
                self.frame[i + (self.width + 1) * (l + 3)] = empty_fill;
            }
        }
    }

    fn clear(&mut self, l: usize, fill: char) {
        for i in 0..self.left_menu_w() {
            self.frame[i + (self.width + 1) * (l + 3)] = fill;
        }
    }

    fn write_divided_into_rows(&mut self, l: usize, fill: char, texts: &[&str]) {
        let div = self.left_menu_w() / texts.len();
        let mut pointer = 0;
        let mut acclen = 0;
        for i in 0..self.left_menu_w() {
            if i - acclen > div {
                pointer += 1;
                acclen = i;
            }
            if i - acclen < texts[pointer].len() {
                self.frame[i + (self.width + 1) * (l + 3)] = texts[pointer]
                    .chars()
                    .nth(i - acclen)
                    .unwrap();
            } else {
                self.frame[i + (self.width + 1) * (l + 3)] = fill;
            }
        }
    }

    fn write_divided_into_rows_middle(&mut self, l: usize, fill: char, texts: &[&str]) {
        if texts.is_empty() {
            return;
        }
        if l > self.menu_h {
            return;
        }
        let div = self.left_menu_w() / texts.len();
        if div == 0 {
            return;
        }
        let mut pointer = 0;
        let mut acclen = 0;
        for i in 0..self.left_menu_w() {
            if i - acclen > div {
                pointer += 1;
                acclen = i;
                if pointer >= texts.len() {
                    break;
                }
            }
            let current_text = texts.get(pointer).unwrap_or(&"");
            let center_offset = (div - current_text.len()) / 2;
            let relative_pos = i - acclen;
            if relative_pos >= center_offset && relative_pos < center_offset + current_text.len() {
                self.frame[i + (self.width + 1) * (l + 3)] = current_text
                    .chars()
                    .nth(relative_pos - center_offset)
                    .unwrap();
            } else {
                self.frame[i + (self.width + 1) * (l + 3)] = fill;
            }
        }
    }
}

impl fmt::Display for ConsoleMenu {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.frame.iter().collect::<String>())
    }
}
struct InfoTab{
}
impl InfoTab {
    fn new() -> Self {
        Self {  }
    }
    
    pub fn show(&self,menu: &mut ConsoleMenu){
        for x in 0..menu.menu_h {
            menu.clear(x, ' ');
        }
        let middle = menu.menu_h / 2 - 3;
        menu.clear(middle + 0, '>');
        menu.write_in_middle(middle + 1, "Program made by JenixDay", '\0', 0);
        menu.write_in_middle(middle + 2, "cuz Stalcraft", '\0', 0);
        menu.clear(middle + 3, '<');
    }
}
fn get_settings_file() -> std::io::Result<File> {
    let file = if Path::new("./settings.txt").exists() {
        File::options()
            .read(true)
            .write(true)
            .open("./settings.txt")?
    } else {
        File::create("./settings.txt")?
    };
    Ok(file)
}
fn write_line(x: &File,) -> std::io::Result<File> {
    let file = if Path::new("./settings.txt").exists() {
        File::options()
            .read(true)
            .write(true)
            .open("./settings.txt")?
    } else {
        File::create("./settings.txt")?
    };
    Ok(file)
}
#[tokio::main]
async  fn main() {
    
    let mut menu = ConsoleMenu::empty();
    menu.set_size(120, 18);
    println!("{}", menu);
    
    let info_tab = InfoTab::new();
    //menu.write_in_middle(0, "George",' ',0);
    //menu.write_divided_into_rows(1, ' ', &["Name", "Price", "Date", "Kat"]);
    //menu.write_divided_into_rows(2, ' ', &["Cacak", "2Kk", "Heute", "Op"]);

    info_tab.show(&mut menu);


    // let batareyka = "zy32";

    let regions = net::get_list_of_regions().await.expect("Eoorr");
    for region in &regions {
        println!("{} {}", region.id(), region.name());
    }
    println!("Total regions: {}", regions.len());
    
    let mut input_buffer = String::new();


    loop {
        println!("{}", menu);
        let _ = stdin().read_line(&mut input_buffer).unwrap_or(0);
        let mut it = input_buffer.chars();
        let fc = it.next().unwrap_or('\0');
        let sc = it.next().unwrap_or('\0');
        
        match (fc,sc) {
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
                }
            },
            _ => {}
        }
        input_buffer.clear();

    }
    //net::get_item_price_history(regions.first().unwrap(), "zy32", "kys", 0).await?;

    
    println!("{}", menu);
}