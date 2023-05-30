#[allow(dead_code)]

#[derive(Clone)]
pub enum Colors {
    Black = 0x0,
    Blue = 0x1,
    Green = 0x2,
    Cyan = 0x3,
    Red = 0x4,
    Purple = 0x5,
    Brown = 0x6,
    Gray = 0x7,
    LightGray = 0x8,
    LightBlue = 0x9,
    LightGreen = 0xa,
    LightCyan = 0xb,
    LightRed = 0xc,
    Pink = 0xd,
    Yellow = 0xe,
    White = 0xf,
}

#[derive(Clone)]
struct Char {
    character: char,
    color: Colors,
}

pub struct Console {
    num_columns: usize,
    num_rows: usize,
    
    current_column: usize,
    current_row: usize,

    buffer: *mut u8,
}

impl Console {
    pub fn new() -> Self {
        Console {
            num_columns: 80,
            num_rows: 25,
            
            current_column: 0,
            current_row: 0,
            
            buffer: 0xb8000 as *mut u8,
        }
    }
    
    pub fn clear_row(&self, row: usize) {
        for column in 0..self.num_columns {
            unsafe {
                *self.buffer.offset((column + self.num_columns * row) as isize * 2) = ' ' as u8;
                *self.buffer.offset((column + self.num_columns * row) as isize * 2 + 1) = Colors::White as u8;
            }
        }
    }
    
    pub fn clear(&self) {
        for row in 0..self.num_rows {
            self.clear_row(row);
        }
    }
    
    fn print_newline(&mut self) {
        self.current_column = 0;
        
        if self.current_row < self.num_rows - 1 {
            self.current_row += 1;
            return;
        }

        for row in 1..self.num_rows {
            for col in 0..self.num_columns {
                unsafe {
                    let character = self.buffer.offset((col + self.num_columns * row) as isize * 2).read();
                    *self.buffer.offset((col + self.num_columns * (row - 1)) as isize * 2) = character;
                }
            }
        }
        
        self.clear_row(self.num_columns - 1);
    }
    
    fn print_char(&mut self, character: char, color: Option<Colors>) {
        if character == '\n' {
            self.print_newline();
            return;
        }
        
        if self.current_column > self.num_columns {
            self.print_newline();
        }
        
        unsafe {
            *self.buffer.offset((self.current_column + self.num_columns * self.current_row) as isize * 2) = character as u8;
            *self.buffer.offset((self.current_column + self.num_columns * self.current_row) as isize * 2 + 1) = color.unwrap_or(Colors::White) as u8;
        }
        
        self.current_column += 1;
    }

    pub fn print(&mut self, string: &str, color: Option<Colors>) {
        for i in string.bytes() {
            self.print_char(i as char, color.clone());
        }
    }
    
    pub fn println(&mut self, string: &str, color: Option<Colors>) {
        self.print(string, color);
        self.print_newline();
    }
}
