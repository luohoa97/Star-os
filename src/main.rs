#![no_std]
#![no_main]

mod time;
mod uxstring;
mod vga;
mod keyboard_service;
mod keyboard_codes;
mod wait;

use uxstring::uxstring;
use time::{get_current_unix_time, UnixTime};
use keyboard_service::init_keyboard;
use vga::textmode::{vga_print_line, vga_clear, move_cursor};
use vga::colors::*;
use wait::wait;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let mut keyboard = init_keyboard();

    unsafe {
        loop {
            vga_clear(0x0F, 0x00);
        move_cursor(0, 0);
        vga_print_line("Keyboard Test\n", GREEN, BLACK);
        vga_print_line("INIT\n\n", TRANSPARENT, WHITE);
        vga_print_line("Okay, seriously.", RED, BLACK);

        // Initialize and update Unix time
        let mut unix_time = get_current_unix_time();
        unix_time.update();
        let seconds = unix_time.as_seconds();
        let seconds_string = uxstring(seconds);

        vga_print_line(seconds_string, GREEN, BLACK);

        wait(1).swait(); // Wait for 1 milliseconds (0.001 second)
        }
    }

    loop {
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
