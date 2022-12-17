const BUF_ADDR: u32 = 0xb8000;
const HEIGHT: u32 = 25;
const WIDTH: u32 = 80;

#[derive(PartialEq)]
pub enum Alignment {
    Left,
    Right,
    Center
}

pub enum Color {
    LightGreen = 0xa,
    Black = 0x0,
    Blue = 0x1,
    Red = 0x4,
    Yellow = 0xe,
    White = 0xf
}

pub struct AsciiChar {
    pub char_byte: u8,
    pub color_byte: u8
}

pub struct Screen {
    buffer: *mut u8,
    color_background: u8,
    color_foreground: u8,
    align: Alignment,
    line_offset: u32,
    char_offset: u32,
    symbols_count: u32
}

impl core::fmt::Write for Screen {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.print(s);
        Ok(())
    }
}

impl Screen {

    pub fn new(foreground: Color, background: Color, align: Alignment) -> Screen {
        let mut screen = Screen {
            buffer: BUF_ADDR as *mut u8,
            color_background: background as u8,
            color_foreground: foreground as u8,
            align,
            line_offset: 0,
            char_offset: 0,
            symbols_count: 0
        };
        screen.char_offset = match screen.align {
            Alignment::Left => 0,
            Alignment::Right => WIDTH - 1,
            Alignment::Center => WIDTH / 2
        };
        return screen;
    }

    pub fn write_char(&self, offset: u32, char: AsciiChar) {
        unsafe {
            *self.buffer.offset(offset as isize * 2) = char.char_byte;
            if char.char_byte != 0{
                *self.buffer.offset(offset as isize * 2 + 1) = char.color_byte;
            }
            else{
                *self.buffer.offset(offset as isize * 2 + 1) = Color::Black as u8;
            }
        }
    }

    pub fn read_char(&self, offset: u32) -> AsciiChar {
        unsafe {
            return AsciiChar{
                char_byte: *self.buffer.offset(offset as isize * 2),
                color_byte: *self.buffer.offset(offset as isize * 2 + 1)
            }
        }
    }

    pub fn write_arr(&self, array: &[[u8; 80]; 25]) {
        for i in 0..25 {
            for j in 0..80 {
                self.write_char(i * WIDTH + j, AsciiChar{char_byte: array[i as usize][j as usize],
                    color_byte: (self.color_background << 4) | self.color_foreground});
            }
        }
    }

    pub fn print(&mut self, s: &str) {
        for char in s.bytes(){
            if char == ('\n' as u8) {
                self.new_line();
            }
            else{
                if self.symbols_count == WIDTH {
                    self.new_line();
                }
                let line_offset : u32 = if self.line_offset > HEIGHT - 1 { HEIGHT - 1 } else { self.line_offset };


                if self.align == Alignment::Center {
                    if &self.symbols_count % 2 != 0 {
                        self.move_line_to_left(line_offset);
                        self.char_offset -= 1;
                    }
                }
                else if self.align == Alignment::Right{
                    if self.symbols_count != 0{
                        self.move_line_to_left(line_offset);
                        self.char_offset -= 1;
                    }
                }

                self.write_char(line_offset * WIDTH + self.char_offset, AsciiChar{char_byte: char, color_byte: (self.color_background << 4) | self.color_foreground});
                self.symbols_count += 1;
                self.char_offset += 1;
            }
        }
    }

    fn new_line(&mut self) {
        if self.line_offset == HEIGHT {
            self.move_all_lines_to_top();
            self.clear_last_line();
        } else {
            self.line_offset += 1;
        }
        self.char_offset = match &self.align {
            Alignment::Left => 0,
            Alignment::Right => WIDTH - 1,
            Alignment::Center => WIDTH / 2
        };
        self.symbols_count = 0;
    }

    fn move_all_lines_to_top(&mut self) {
        for i in 0..HEIGHT-1 {
            for j in 0..WIDTH {
                self.write_char(i * WIDTH + j, self.read_char((i + 1) * WIDTH + j));
            }
        }
    }

    fn move_line_to_left(&mut self, line : u32) {
        for j in 0..WIDTH-1 {
            self.write_char(line * WIDTH + j, self.read_char(line * WIDTH + j + 1));
        }
    }

    fn clear_last_line(&mut self) {
        for j in 0..WIDTH {
            self.write_char((HEIGHT - 1) * WIDTH + j, AsciiChar{char_byte: 0, color_byte: (self.color_background << 4) | self.color_foreground});
        }
    }
}
