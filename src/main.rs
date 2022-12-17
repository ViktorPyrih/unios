#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points
#![feature(abi_x86_interrupt)]

use core::panic::PanicInfo;
use pc_keyboard::DecodedKey;

mod vga_buf;
mod interrupts;
mod shell;
mod file_system;

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("----------------------------------------------");
    println!("{}", _info);
    println!("----------------------------------------------");
    loop {}
}

fn my_keyboard_handler(key: DecodedKey) {
    shell::handle_keyboard_interrupt(key);
}

fn my_timer_handler() {
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    print_product_details();
    interrupts::set_keyboard_interrupt_handler(my_keyboard_handler);
    interrupts::set_timer_interrupt_handler(my_timer_handler);
    interrupts::init();
    loop {}
}

fn print_product_details() {
    print!("CLI v1.0");
    println!();
}
