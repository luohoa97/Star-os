#![no_std]
#![no_main]

mod time;
mod vga;
mod osalloc;
mod wait;
mod uxstring;
mod usize;
use crate::uxstring::uxstring;
use osalloc::{GlobalAllocatorWrapper, GLOBAL_ALLOCATOR};
use crate::wait::wait;
use crate::vga::textmode::*;
use crate::vga::colors::*;
use crate::time::*; 
use bootloader::BootInfo;
use acpi::address;
use core::ptr;
const Driver_TEST: &[u8] = include_bytes!("drivers/driver.bin");
use core::alloc::Layout;

#[global_allocator]
static GLOBAL_ALLOC: GlobalAllocatorWrapper = GlobalAllocatorWrapper;

mod keymapper;
mod ps2key;

use crate::keymapper::*;
use crate::ps2key::*;
use core::str;
use crate::usize::usize_to_str;

use heapless::String;

const REPEAT_DELAY: u64 = 500;  
const REPEAT_RATE: u64 = 5;    

static mut TYPED_STRING: String<1600> = String::new();
static mut OUTPUTSTRING: String<1600> = String::new();
static mut HELPMESSAGE: String<1600> = String::new();

#[no_mangle]
pub extern "C" fn _start(boot_info: &'static BootInfo) -> ! {
    unsafe {
        vga_clear(WHITE, BLACK);
        outadd("Loading drivers\n\n");
        outadd("/");
        wait(1000).swait();
        cls();
        outadd("Loading drivers\n\n");
        outadd("-");
        wait(1000).swait();
        cls();
        outadd("Loading drivers\n\n");
        outadd("\\");
        wait(1000).swait();
        cls();
        outadd("Loading drivers\n\n");
        outadd("-");
        wait(1000).swait();
        outadd("Drivers have finished initializing");
        wait(1000).swait();
        cls();
        outadd("Star os 2024-2024\n\n    *    \n   * *   \n  *   *  \n *     * \n* * * * *\n *     * \n  *   *  \n   * *   \n    *    ");
        outadd("\n\nThis \"SOFTWARE\" is under the protection of the Copyright Act 1968 and is covered under the GPL v3 license and is covered under ABSOLUTELY NO WARRANTY!")
    }

    // load_driver(Driver_TEST); I cannot find a way to make a driver for this.
    init_ps2();

    let mut last_key: Option<u8> = None;
    let mut last_key_press_time: u64 = 0;
    let mut repeat_delay_expired: bool = false;

    let mut previous_time = get_current_unix_time().as_milliseconds();

    loop {
        let current_time = get_current_unix_time().as_milliseconds();

        let scan_code = read_scan_code();
        if scan_code != 0 {
            if Some(scan_code) != last_key {

                last_key = Some(scan_code);
                last_key_press_time = current_time;
                repeat_delay_expired = false;  
                handle_key(scan_code);
            } else if Some(scan_code) == last_key {
                if !repeat_delay_expired {

                    if current_time - last_key_press_time > REPEAT_DELAY {
                        repeat_delay_expired = true;
                        handle_key(scan_code);
                        last_key_press_time = current_time;
                    }
                } else {

                    if current_time - last_key_press_time > REPEAT_RATE {
                        last_key_press_time = current_time;
                        handle_key(scan_code);
                    }
                }
            }
        } else {

            last_key = None;
            repeat_delay_expired = false;
        }

        if current_time - previous_time >= 10 {  
            previous_time = current_time;
        }
    }
}

/*
fn load_driver(driver_bin: &[u8]) {
    unsafe {
        let layout = Layout::from_size_align(MY_DRIVER.len(), 128).unwrap();
        let driver_mem = GLOBAL_ALLOCATOR.lock().alloc(layout); // use allocator to allocate the driver some memory(driver_bin.len());
        ptr::copy_nonoverlapping(driver_bin.as_ptr(), driver_mem as *mut u8, driver_bin.len());
        let driver_entry: extern "C" fn() = core::mem::transmute(driver_mem);
        driver_entry();
    }
}
*/

unsafe fn cmd(letter: &str, foreground: u8, background: u8) {
    TYPED_STRING.push_str(letter);
    reprint();
}

unsafe fn outadd(string: &str) {
    OUTPUTSTRING.push_str("\n");
    OUTPUTSTRING.push_str(string);
    reprint();
}

unsafe fn remove_last_character() {
    TYPED_STRING.pop(); 
    reprint()
}

unsafe fn reprint() {
    vga_clear(WHITE, BLACK);

    if let Ok(()) = OUTPUTSTRING.push_str("\n") {

        let OUTSTR = OUTPUTSTRING.as_str();
        let INSTR = TYPED_STRING.as_str();

        vga_print_line(OUTSTR, WHITE, BLACK);
        vga_print_line(INSTR, WHITE, BLACK);
        OUTPUTSTRING.pop();
    } else {

        vga_print_line("Error appending to OUTPUTSTRING", BLACK, RED);
    }
}

unsafe fn parse_command(command: &str) -> (&str, [&str; 10]) {

    fn count_args(s: &str) -> usize {
        let mut count = 0;
        let mut in_space = true;
        let mut skip_null = false;
        for c in s.chars() {
            if c == ' ' {
                if !in_space {
                    if !skip_null {
                        count += 1;
                    }
                    skip_null = false;
                    in_space = true;
                }
            } else {
                if in_space {
                    if c != '.' {
                        in_space = false;
                    }
                }
                if !in_space && s.get(c.len_utf8()..).unwrap_or("").starts_with(".null") {
                    skip_null = true;
                }
            }
        }
        if !in_space && !skip_null {
            count += 1; 
        }
        count
    }

    let mut args: [&str; 10] = Default::default(); 
    let mut current_arg = 0;
    let mut start_index = 0;
    let mut in_space = true;
    let mut skip_null = false;

    for (i, c) in command.char_indices() {
        if c == ' ' {
            if !in_space {
                if !skip_null {
                    args[current_arg] = &command[start_index..i];
                    current_arg += 1;
                }
                skip_null = false;
                in_space = true;
            }
        } else {
            if in_space {
                start_index = i;
                in_space = false;
            }
            if !in_space && c == '.' {
                if command.get(i..i + 5) == Some(".null") {
                    skip_null = true;
                }
            }
        }
    }

    if !in_space && !skip_null {
        args[current_arg] = &command[start_index..];
    }

    let command_part = args[0];
    let num_args = count_args(command);

    let mut filtered_args: [&str; 10] = Default::default();
    let mut arg_count = 0;
    for i in 1..num_args {
        if args[i] != ".null" {
            filtered_args[arg_count] = args[i];
            arg_count += 1;
        }
    }

    (command_part, filtered_args)
}

unsafe fn cls() {
    vga_clear(WHITE, BLACK);
    OUTPUTSTRING.clear();
    TYPED_STRING.clear();
}

unsafe fn execute(command: &str) {
    let (cmd, args) = parse_command(command);

    match cmd {
        "echo" => {
            if !args.is_empty() {
                outadd(args[0]);
            }
        }
        "time" => {
            let unix_time = get_current_unix_time();
            let timestamp_in_milliseconds: u64 = unix_time.timestamp / 1000;
            outadd(uxstring(timestamp_in_milliseconds));
        }
        "cls" => {
            cls();
        }
        "help" => {
            HELPMESSAGE.push_str("Available commands:\n");
            HELPMESSAGE.push_str("echo <text> - Outputs the text\n");
            HELPMESSAGE.push_str("time - Displays the current Unix timestamp\n");
            HELPMESSAGE.push_str("cls - Clears the screen [useful incase of error appending to buffer]\n");
            HELPMESSAGE.push_str("help - Displays this message\n");
            HELPMESSAGE.push_str("available - Displays the available memory\n");
            HELPMESSAGE.push_str("locked - Displays the amount of memory used for system processes\n");
            HELPMESSAGE.push_str("allocated - Displays the amount of memory allocated to the system\n");
            HELPMESSAGE.push_str("ram - Displays the anount of ram you have in your system\n");
            outadd(HELPMESSAGE.as_str());
            HELPMESSAGE.clear();
        }
        "allocated" => {
            outadd(usize_to_str(GLOBAL_ALLOCATOR.lock().heap_size()));
        }
        "available" => {
            outadd(usize_to_str(GLOBAL_ALLOCATOR.lock().remaining_memory()));
        }
        "locked" => {
            outadd(usize_to_str(GLOBAL_ALLOCATOR.lock().used_memory()));
        }
        "ram" => {
            outadd(usize_to_str(GLOBAL_ALLOCATOR.lock().available_memory()));
        }
        "refresh" => {
            reprint();
        }
        "license" => {
            outadd("Star os 2024-2024\n\n    *    \n   * *   \n  *   *  \n *     * \n* * * * *\n *     * \n  *   *  \n   * *   \n    *    ");
        outadd("\n\nThis \"SOFTWARE\" is under the protection of the Copyright Act 1968 and is covered under the GPL v3 license and is covered under ABSOLUTELY NO WARRANTY!")
        }
        _ => {
            outadd("Exec: Unknown command, type 'help' for commands");
        }
    }
}

unsafe fn omitoutput() {
    static mut TEMPOMIT: String<300> = String::new();
    TEMPOMIT.clear();
    let typed_str = TYPED_STRING.as_str();
    if typed_str.ends_with(" .null") {}
    else {
        OUTPUTSTRING.push_str(typed_str);
    }
    TEMPOMIT.push_str(typed_str);
    TYPED_STRING.clear();
    execute(TEMPOMIT.as_str());
}

fn handle_key(scan_code: u8) {
    match scan_code {
        KEY_A => unsafe { cmd("a", WHITE, BLACK); },
        KEY_B => unsafe { cmd("b", WHITE, BLACK); },
        KEY_C => unsafe { cmd("c", WHITE, BLACK); },
        KEY_D => unsafe { cmd("d", WHITE, BLACK); },
        KEY_E => unsafe { cmd("e", WHITE, BLACK); },
        KEY_F => unsafe { cmd("f", WHITE, BLACK); },
        KEY_G => unsafe { cmd("g", WHITE, BLACK); },
        KEY_H => unsafe { cmd("h", WHITE, BLACK); },
        KEY_I => unsafe { cmd("i", WHITE, BLACK); },
        KEY_J => unsafe { cmd("j", WHITE, BLACK); },
        KEY_K => unsafe { cmd("k", WHITE, BLACK); },
        KEY_L => unsafe { cmd("l", WHITE, BLACK); },
        KEY_M => unsafe { cmd("m", WHITE, BLACK); },
        KEY_N => unsafe { cmd("n", WHITE, BLACK); },
        KEY_O => unsafe { cmd("o", WHITE, BLACK); },
        KEY_P => unsafe { cmd("p", WHITE, BLACK); },
        KEY_Q => unsafe { cmd("q", WHITE, BLACK); },
        KEY_R => unsafe { cmd("r", WHITE, BLACK); },
        KEY_S => unsafe { cmd("s", WHITE, BLACK); },
        KEY_T => unsafe { cmd("t", WHITE, BLACK); },
        KEY_U => unsafe { cmd("u", WHITE, BLACK); },
        KEY_V => unsafe { cmd("v", WHITE, BLACK); },
        KEY_W => unsafe { cmd("w", WHITE, BLACK); },
        KEY_X => unsafe { cmd("x", WHITE, BLACK); },
        KEY_Y => unsafe { cmd("y", WHITE, BLACK); },
        KEY_Z => unsafe { cmd("z", WHITE, BLACK); },

        KEY_ESC => unsafe { cmd("[ESC]", WHITE, BLACK); },
        KEY_ENTER => unsafe { omitoutput(); },
        KEY_BACKSPACE => unsafe { remove_last_character(); },
        KEY_TAB => unsafe { cmd("\t", WHITE, BLACK); },
        KEY_SPACE => unsafe { cmd(" ", WHITE, BLACK); },
        KEY_MINUS => unsafe { cmd("-", WHITE, BLACK); },
        KEY_EQUALS => unsafe { cmd("=", WHITE, BLACK); },
        KEY_LEFT_BRACKET => unsafe { cmd("[", WHITE, BLACK); },
        KEY_RIGHT_BRACKET => unsafe { cmd("]", WHITE, BLACK); },
        KEY_BACKSLASH => unsafe { cmd("\\", WHITE, BLACK); },
        KEY_SEMICOLON => unsafe { cmd(";", WHITE, BLACK); },
        KEY_APOSTROPHE => unsafe { cmd("'", WHITE, BLACK); },
        KEY_GRAVE => unsafe { cmd("`", WHITE, BLACK); },
        KEY_COMMA => unsafe { cmd(",", WHITE, BLACK); },
        KEY_DOT => unsafe { cmd(".", WHITE, BLACK); },
        KEY_SLASH => unsafe { cmd("/", WHITE, BLACK); },

        KEY_F1 => unsafe { cmd("[F1]", WHITE, BLACK); },
        KEY_F2 => unsafe { cmd("[F2]", WHITE, BLACK); },
        KEY_F3 => unsafe { cmd("[F3]", WHITE, BLACK); },
        KEY_F4 => unsafe { cmd("[F4]", WHITE, BLACK); },
        KEY_F5 => unsafe { cmd("[F5]", WHITE, BLACK); },
        KEY_F6 => unsafe { cmd("[F6]", WHITE, BLACK); },
        KEY_F7 => unsafe { cmd("[F7]", WHITE, BLACK); },
        KEY_F8 => unsafe { cmd("[F8]", WHITE, BLACK); },
        KEY_F9 => unsafe { cmd("[F9]", WHITE, BLACK); },
        KEY_F10 => unsafe { cmd("[F10]", WHITE, BLACK); },
        KEY_F11 => unsafe { cmd("[F11]", WHITE, BLACK); },
        KEY_F12 => unsafe { cmd("[F12]", WHITE, BLACK); },

        KEY_LEFT => unsafe { cmd("[LEFT]", WHITE, BLACK); },
        KEY_RIGHT => unsafe { cmd("[RIGHT]", WHITE, BLACK); },
        KEY_UP => unsafe { cmd("[UP]", WHITE, BLACK); },
        KEY_DOWN => unsafe { cmd("[DOWN]", WHITE, BLACK); },

        KEY_INSERT => unsafe { cmd("[INSERT]", WHITE, BLACK); },
        KEY_DELETE => unsafe { cmd("[DELETE]", WHITE, BLACK); },
        KEY_HOME => unsafe { cmd("[HOME]", WHITE, BLACK); },
        KEY_END => unsafe { cmd("[END]", WHITE, BLACK); },
        KEY_NUM_LOCK => unsafe { cmd("[NUM LOCK]", WHITE, BLACK); },
        KEY_CAPS_LOCK => unsafe { cmd("[CAPS LOCK]", WHITE, BLACK); },
        KEY_SCROLL_LOCK => unsafe { cmd("[SCROLL LOCK]", WHITE, BLACK); },

        KEY_0 => unsafe { cmd("0", WHITE, BLACK); },
        KEY_1 => unsafe { cmd("1", WHITE, BLACK); },
        KEY_2 => unsafe { cmd("2", WHITE, BLACK); },
        KEY_3 => unsafe { cmd("3", WHITE, BLACK); },
        KEY_4 => unsafe { cmd("4", WHITE, BLACK); },
        KEY_5 => unsafe { cmd("5", WHITE, BLACK); },
        KEY_6 => unsafe { cmd("6", WHITE, BLACK); },
        KEY_7 => unsafe { cmd("7", WHITE, BLACK); },
        KEY_8 => unsafe { cmd("8", WHITE, BLACK); },
        KEY_9 => unsafe { cmd("9", WHITE, BLACK); },
        KEY_KP_DIVIDE => unsafe { cmd("/", WHITE, BLACK); },
        KEY_KP_MULTIPLY => unsafe { cmd("*", WHITE, BLACK); },
        KEY_MINUS => unsafe { cmd("-", WHITE, BLACK); },
        KEY_ENTER => unsafe { cmd("[NUMPAD ENTER]", WHITE, BLACK); },
        KEY_DOT => unsafe { cmd(".", WHITE, BLACK); },

        _ => (),
    }
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    unsafe {

        if let Some(message) = info.payload().downcast_ref::<&str>() {

            vga_print_line(message, RED, BLACK);
        } else {

            vga_print_line("Panic occurred!", RED, BLACK);
        }
    }
    loop {}
}
