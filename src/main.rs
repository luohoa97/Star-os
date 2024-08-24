#![no_std]
#![no_main]

mod vga;
mod keyboard_service;
mod keyboard_codes;

use keyboard_service::init_keyboard;
use vga::textmode::{vga_print_line, vga_clear, move_cursor};
use crate::keyboard_codes::*;
use vga::colors::*;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let mut keyboard = init_keyboard();

    unsafe {
        vga_clear(0x0F, 0x00);
        move_cursor(0, 0);
        vga_print_line("Keyboard Test\n", RED, WHITE);
        vga_print_line("INIT\n\n", TRANSPARENT, WHITE);
        vga_print_line("Okay, seriously.", WHITE, BLACK);
    }

    loop {
        unsafe {
            keyboard.update();

            if keyboard.get_key_state(KEY_A) {
                move_cursor(0, 1);
                vga_print_line("Key 'A' Pressed", 0x0C, 0x00);
            }
        }
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}


// Example
