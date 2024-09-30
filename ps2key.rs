/*
use x86_64::instructions::port::Port;
use crate::time::{UnixTime, get_current_unix_time};

const PS2_DATA_PORT: u16 = 0x60;
const PS2_COMMAND_PORT: u16 = 0x64;

const PS2_CMD_DISABLE_KEYBOARD: u8 = 0xAD;
const PS2_CMD_ENABLE_KEYBOARD: u8 = 0xAE;

const MAX_KEYS: usize = 256; // Maximum number of keys

// Data structures for tracking key presses
#[repr(C)]
struct KeyPressEntry {
    timestamp: u64,
    is_pressed: bool,
}

struct KeyPressTracker {
    entries: [KeyPressEntry; MAX_KEYS],
}

impl KeyPressTracker {
    pub fn new() -> Self {
        KeyPressTracker {
            entries: [KeyPressEntry { timestamp: 0, is_pressed: false }; MAX_KEYS],
        }
    }

    pub fn key_press(&mut self, scan_code: u8) {
        let now = get_current_unix_time();
        let index = scan_code as usize;
        self.entries[index].timestamp = now.as_milliseconds();
        self.entries[index].is_pressed = true;
    }

    pub fn key_release(&mut self, scan_code: u8) {
        let now = get_current_unix_time();
        let index = scan_code as usize;
        if self.entries[index].is_pressed {
            let duration = now.as_milliseconds() - self.entries[index].timestamp;
            self.entries[index].is_pressed = false;
            // Print or log the duration for debugging purposes
            // e.g., println!("Key {} was pressed for {} ms", scan_code, duration);
        }
    }

    pub fn get_key_press_duration(&self, scan_code: u8) -> Option<u64> {
        let index = scan_code as usize;
        if self.entries[index].is_pressed {
            None
        } else {
            Some(self.entries[index].timestamp)
        }
    }
}

fn outb(port: u16, value: u8) {
    let mut port = Port::new(port);
    unsafe {
        port.write(value);
    }
}

fn inb(port: u16) -> u8 {
    let mut port = Port::new(port);
    unsafe {
        port.read()
    }
}

pub fn init_ps2() {
    // Disable the keyboard
    outb(PS2_COMMAND_PORT, PS2_CMD_DISABLE_KEYBOARD);

    // Wait for the controller to be ready
    while inb(PS2_COMMAND_PORT) & 0x02 != 0 {}

    // Enable the keyboard
    outb(PS2_COMMAND_PORT, PS2_CMD_ENABLE_KEYBOARD);

    // Wait for the controller to be ready
    while inb(PS2_COMMAND_PORT) & 0x02 != 0 {}
}

pub fn read_scan_code(tracker: &mut KeyPressTracker) -> u8 {
    let scan_code = inb(PS2_DATA_PORT);
    if scan_code == 0xF0 { // Key release code
        let release_code = inb(PS2_DATA_PORT);
        tracker.key_release(release_code);
    } else {
        tracker.key_press(scan_code);
    }
    scan_code
}
*/

// ps2_keyboard.rs

use x86_64::instructions::port::Port;

const PS2_DATA_PORT: u16 = 0x60;
const PS2_COMMAND_PORT: u16 = 0x64;

const PS2_CMD_DISABLE_KEYBOARD: u8 = 0xAD;
const PS2_CMD_ENABLE_KEYBOARD: u8 = 0xAE;

fn outb(port: u16, value: u8) {
    let mut port = Port::new(port);
    unsafe {
        port.write(value);
    }
}

fn inb(port: u16) -> u8 {
    let mut port = Port::new(port);
    unsafe {
        port.read()
    }
}

pub fn init_ps2() {
    // Disable the keyboard
    outb(PS2_COMMAND_PORT, PS2_CMD_DISABLE_KEYBOARD);

    // Wait for the controller to be ready
    while inb(PS2_COMMAND_PORT) & 0x02 != 0 {}

    // Enable the keyboard
    outb(PS2_COMMAND_PORT, PS2_CMD_ENABLE_KEYBOARD);

    // Wait for the controller to be ready
    while inb(PS2_COMMAND_PORT) & 0x02 != 0 {}
}

pub fn read_scan_code() -> u8 {
    inb(PS2_DATA_PORT)
}