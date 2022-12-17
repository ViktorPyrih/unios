use core::ptr::null_mut;
use crate::{print, println};
use crate::vga_buf::SCREEN;
use crate::file_system::{FileSystem, MAX_MESSAGE_SIZE, LINE_END, NAME_SIZE, str_name_to_arr, compare_text_arrs};
use pc_keyboard::DecodedKey;
use lazy_static::lazy_static;

pub const MAX_ARRAY_SIZE : usize = 2048;
pub const MAX_BUF_SIZE : usize = 76;

lazy_static! {
    static ref SH: spin::Mutex<Shell> = spin::Mutex::new({
        let mut sh = Shell::new();
        sh
    });
}

pub fn handle_keyboard_interrupt(key: DecodedKey) {
    match key {
        DecodedKey::Unicode(c) => SH.lock().on_key_pressed(c as u8),
        DecodedKey::RawKey(rk) => {}
    }
}

struct Shell {
    buf: [u8; MAX_BUF_SIZE],
    buf_len: usize,
    file_system_arr: [u8; MAX_ARRAY_SIZE]
}

impl Shell {

    pub fn new() -> Shell {
        print!(" $ ");
        Shell {
            buf: [0; MAX_BUF_SIZE],
            buf_len: 0,
            file_system_arr: [0; MAX_ARRAY_SIZE]
        }
    }

    pub fn on_key_pressed(&mut self, key: u8) {
        match key {
            b'\n' => {
                let mut sys = FileSystem::new(self.file_system_arr);
                let (text_left, text_left_count, text_right, text_right_count) = split_text(self.buf, self.buf_len);

                if compare_text_arrs(text_left, str_name_to_arr("clear")) {
                    SCREEN.lock().clear();
                    print!("CLI v1.0");
                    println!();
                } else {
                    println!();
                    sys.execute_command(text_left, text_right);
                }

                print!(" $ ");
                self.buf = [0; MAX_BUF_SIZE];
                self.buf_len = 0;
                self.file_system_arr = sys.get_arr();
            }
            8u8 =>{
                if (self.buf_len > 0) {
                    SCREEN.lock().remove();
                    self.buf_len -= 1;
                }
            }
            _ => {
                if (self.buf_len != MAX_BUF_SIZE) {
                    self.buf[self.buf_len] = key;
                    self.buf_len += 1;
                    print!("{}", key as char);
                } else {
                    SCREEN.lock().remove();
                    print!("{}", key as char);
                }
            }
        }
    }
}

fn split_text(buf: [u8; MAX_BUF_SIZE], buf_len: usize) -> ([u8; NAME_SIZE], usize, [u8; NAME_SIZE], usize) {

    let mut left_text = [0; NAME_SIZE];
    let mut left_text_count : usize = 0;
    let mut right_text = [0; NAME_SIZE];
    let mut right_text_count : usize = 0;

    let mut flag : bool = false;
    for i in 0..buf_len {
        if buf[i] == b' ' && !flag {
            flag = true;
        } else {
            if !flag {
                left_text[left_text_count] = buf[i];
                left_text_count += 1;
            } else {
                right_text[right_text_count] = buf[i];
                right_text_count += 1;
            }
        }
    }
    left_text[left_text_count] = LINE_END;
    right_text[right_text_count] = LINE_END;
    return (left_text, left_text_count, right_text, right_text_count);
}
