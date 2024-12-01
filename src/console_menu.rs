use core::fmt;

pub enum ConsoleMenusPosition {
    Left,
    Right,
    Center,
}
pub struct ConsoleMenu {
    width: usize,
    height: usize,
    frame: Vec<char>,
    colors: Vec<(u32,u32,(u8,u8,u8))>,
}
impl ConsoleMenu {
    pub fn empty() -> Self {
        ConsoleMenu {
            width: 0,
            height: 0,
            frame: vec![],
            colors: Vec::new(),
        }
    }
    fn left_menu_w(&self) -> usize{ self.width / 2 }
    //fn right_menu_w(&self) -> usize{ (self.width / 2) - if self.width % 2 == 0 { 1 } else { 0 } }
    pub fn set_size(&mut self,width: usize,height: usize){
        self.width = width;
        self.height = height;
        self.frame = vec![' '; (width + 1) * height];
    }
    pub fn colorize(&mut self,start_x: u32,start_y: u32,end_x: u32,end_y: u32,col:(u8,u8,u8)){
        if col == (255,255,255){
            return;
        }
        let start_index = start_x + (start_y * (self.width as u32 + 1));
        let end_index = (end_x + (end_y * (self.width as u32 + 1))).min(self.frame.len() as u32);
        if end_index.saturating_sub(start_index) == 0 {
            return;
        }
        self.colors.push((start_index,end_index,col));
    }
    pub fn clear_colors(&mut self){
        self.colors.clear();
    }
    fn colored(r: u8, g: u8, b: u8, text: &str) -> String {
        return format!("\x1B[38;2;{};{};{}m{}\x1B[0m", r, g, b, text);
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

    pub fn write_line(&mut self, l: usize, text: &str, offset: usize, empty_fill: char, menu: ConsoleMenusPosition) {
        if l >= self.menu_h() {
            return;
        }
        let (from, to) = match menu {
            ConsoleMenusPosition::Right => (self.left_menu_w() + 1, self.width),
            ConsoleMenusPosition::Left => (0, self.left_menu_w()),
            _ => (0, self.width),
        };
        let mut c = text.chars();
        let mut index = 0;
        for i in from..to {
            if index < text.len() && i >= offset {
                self.frame[i + (self.width + 1) * (l + 3)] = c.next().unwrap();
            } else if empty_fill != '\0' {
                self.frame[i + (self.width + 1) * (l + 3)] = empty_fill;
            }
            index+=1;
        }
    }
    pub fn write_in_middle(&mut self, l: usize, text: &str, offset: i32, empty_fill: char, menu: ConsoleMenusPosition) {
        if l >= self.menu_h() {
            return;
        }
        let (from, to) = match menu {
            ConsoleMenusPosition::Right => (self.left_menu_w() + 1, self.width),
            ConsoleMenusPosition::Left => (0, self.left_menu_w()),
            _ => (0, self.width),
        };
        let start = ((to - from) as i32 - text.len() as i32) / 2 + from as i32 + offset;
        let mut c = text.chars();
        for i in from..to {
            if (i as i32 - start) >= 0 && (i as i32 - start) < text.len() as i32 {
                self.frame[i + (self.width + 1) * (l + 3)] = c.next().unwrap();
            } else if empty_fill != '\0' {
                self.frame[i + (self.width + 1) * (l + 3)] = empty_fill;
            }
            //index += 1;
        }
     }
    pub fn clear(&mut self, l: usize, fill: char,menu: ConsoleMenusPosition) {
        if l >= self.menu_h() {
            return;
        }
        let (from, to) = match menu {
            ConsoleMenusPosition::Right => (self.left_menu_w() + 1, self.width),
            ConsoleMenusPosition::Left => (0, self.left_menu_w()),
            _ => (0, self.width),
        };
        for i in from..to {
            if fill != '\0'{
                self.frame[i + (self.width + 1) * (l + 3)] = fill;
            }
        }
    }
    pub fn clear_pages(&mut self, fill: char) {
        for i in 0..self.menu_h() {
            self.clear(i, fill, ConsoleMenusPosition::Left);
            self.clear(i, fill, ConsoleMenusPosition::Right);
        }
    }
    pub fn write_divided_into_rows(&mut self, l: usize, fill: char, texts: &[&str],menu: ConsoleMenusPosition) {
        if l >= self.menu_h() {
            return;
        }
        if texts.is_empty() {
            self.clear(l, fill,ConsoleMenusPosition::Left);
            return;
        }

        let mut pointer = 0;
        let mut chars_iter = texts[pointer].chars();
    
        let (from, to) = match menu {
            ConsoleMenusPosition::Right => (self.left_menu_w() + 1, self.width),
            ConsoleMenusPosition::Left => (0, self.left_menu_w()),
            _ => (0, self.width),
        };
        let div = (to - from + 1) / texts.len();
        for i in from..to {
            let o = i - from;
            if o / div != pointer {
                pointer += 1;
                if pointer>=texts.len(){break;}
                chars_iter = texts[pointer].chars();
            }
    
            if o % div < texts[pointer].len() {
                if let Some(ch) = chars_iter.next() {
                    self.frame[i + (self.width + 1) * (l + 3)] = ch;
                } else {
                    if fill!= '\0'{
                        self.frame[i + (self.width + 1) * (l + 3)] = fill;
                    }
                }
            } else {
                if fill!= '\0'{
                    self.frame[i + (self.width + 1) * (l + 3)] = fill;
                }
            }
        }
    }

    pub fn write_divided_into_rows_middle(&mut self, l: usize, fill: char, texts: &[&str],menu: ConsoleMenusPosition) {
        if l >= self.menu_h() {
            return;
        }
        if texts.is_empty() {
            return;
        }

        let mut pointer = 0;
        let mut acclen = 0;
        let (from, to) = match menu {
            ConsoleMenusPosition::Right => (self.left_menu_w() + 1, self.width),
            ConsoleMenusPosition::Left => (0, self.left_menu_w()),
            _ => (0, self.width),
        };
        let div = (to - from + 1) / texts.len();
        if div == 0 {
            return;
        }
        for i in from..to {
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
                if fill != '\0'{
                    self.frame[i + (self.width + 1) * (l + 3)] = fill;
                }
            }
        }
    }
    
    pub fn menu_h(&self) -> usize {
        self.height - 4
    }
    
    pub fn width(&self) -> usize {
        self.width
    }
}

impl fmt::Display for ConsoleMenu {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.colors.is_empty() {
            return write!(f, "{}", self.frame.iter().collect::<String>());
        }

        let mut cols = self.colors.clone();
        cols.sort_by(|x, y| x.0.cmp(&y.0));
    
        let mut offset = 0;
        
        // Handle initial segment before first color
        if let Some(first_color) = cols.first() {
            if offset < first_color.0 {
                write!(f, "{}", self.frame[offset as usize..first_color.0 as usize].iter().collect::<String>().as_str())?;
                offset = first_color.0;
            }
    
            // Process colored segments
            for c in &cols {
                if offset < c.0 {
                    write!(f, "{}", self.frame[offset as usize..c.0 as usize].iter().collect::<String>().as_str())?;
                    offset = c.0;
                }
    
                write!(f, "{}", ConsoleMenu::colored(
                    c.2.0, c.2.1, c.2.2, 
                    self.frame[offset as usize..=c.1 as usize].iter().collect::<String>().as_str()
                ))?;
                offset = c.1 + 1;
            }
    
            // Handle any remaining uncolored segment
            if offset < self.frame.len() as u32 {
                write!(f, "{}", self.frame[offset as usize..].iter().collect::<String>().as_str())?;
            }
        }
    
        Ok(())
    }
}