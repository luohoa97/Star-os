// TODO: 
// Replace text mode with pixel based system for more colors
// Instead of breaking and deleting upper lines, add array so you can scroll left, right, top, bottom


pub mod colors {
    pub const BLACK: u8 = 0x00;
    pub const BLUE: u8 = 0x01;
    pub const GREEN: u8 = 0x02;
    pub const CYAN: u8 = 0x03;
    pub const RED: u8 = 0x04;
    pub const MAGENTA: u8 = 0x05;
    pub const BROWN: u8 = 0x06;
    pub const LIGHT_GRAY: u8 = 0x07;
    pub const DARK_GRAY: u8 = 0x08;
    pub const LIGHT_BLUE: u8 = 0x09;
    pub const LIGHT_GREEN: u8 = 0x0A;
    pub const LIGHT_CYAN: u8 = 0x0B;
    pub const LIGHT_RED: u8 = 0x0C;
    pub const LIGHT_MAGENTA: u8 = 0x0D;
    pub const LIGHT_YELLOW: u8 = 0x0E;
    pub const WHITE: u8 = 0x0F;

    pub const YELLOW: u8 = 0x0E;
    pub const DARK_RED: u8 = 0x10;
    pub const DARK_GREEN: u8 = 0x11;
    pub const DARK_BLUE: u8 = 0x12;
    pub const DARK_CYAN: u8 = 0x13;
    pub const DARK_MAGENTA: u8 = 0x14;
    pub const DARK_YELLOW: u8 = 0x15;

    pub const TRANSPARENT: u8 = 0x16;
}

pub mod textmode {
    use crate::vga::colors::*;

    const VGA_ADDRESS: usize = 0xB8000;
    const VGA_WIDTH: usize = 80;
    const VGA_HEIGHT: usize = 25;

    static mut VGA_MEMORY: *mut u16 = VGA_ADDRESS as *mut u16;
    static mut CURSOR_COL: usize = 0;
    static mut CURSOR_ROW: usize = 0;

    fn create_attr_byte(fg: u8, bg: u8) -> u8 {
        (bg << 4) | (fg & 0x0F)
    }

    pub unsafe fn vga_print_char(character: char, col: usize, row: usize, fg: u8, bg: u8) {
        if col >= VGA_WIDTH || row >= VGA_HEIGHT {
            return;
        }

        let offset = (row * VGA_WIDTH + col) as usize;

        let attribute_byte = create_attr_byte(fg, bg);
        let color = (attribute_byte as u16) << 8 | character as u16;

        if bg == TRANSPARENT {
            let current_cell = VGA_MEMORY.add(offset).read_volatile();
            let new_color = (attribute_byte as u16) << 8 | character as u16;
            VGA_MEMORY.add(offset).write_volatile(new_color);
        } else if fg == TRANSPARENT {
            let new_color = (attribute_byte as u16) << 8 | b' ' as u16;
            VGA_MEMORY.add(offset).write_volatile(new_color);
        } else {
            VGA_MEMORY.add(offset).write_volatile(color);
        }
    }

    pub unsafe fn vga_clear(fg: u8, bg: u8) {
        let clear_value = ((create_attr_byte(fg, bg) as u16) << 8) | b' ' as u16;
        
        for y in 0..VGA_HEIGHT {
            for x in 0..VGA_WIDTH {
                let offset = (y * VGA_WIDTH + x) as usize;
                VGA_MEMORY.add(offset).write_volatile(clear_value);
            }
        }

        CURSOR_COL = 0;
        CURSOR_ROW = 0;
    }

    pub unsafe fn vga_print_line(text: &str, fg: u8, bg: u8) {
        for c in text.chars() {
            match c {
                '\n' => {
                    CURSOR_COL = 0;
                    CURSOR_ROW += 1;
                    if CURSOR_ROW >= VGA_HEIGHT {
                        scroll_up();
                    }
                },
                _ => {
                    if CURSOR_COL >= VGA_WIDTH {
                        CURSOR_COL = 0;
                        CURSOR_ROW += 1;
                    }
                    if CURSOR_ROW >= VGA_HEIGHT {
                        scroll_up();
                    }
                    vga_print_char(c, CURSOR_COL, CURSOR_ROW, fg, bg);
                    CURSOR_COL += 1;
                }
            }
        }
    }

    pub unsafe fn move_cursor(col: usize, row: usize) {
        if col < VGA_WIDTH && row < VGA_HEIGHT {
            CURSOR_COL = col;
            CURSOR_ROW = row;
        }
    }

    pub unsafe fn check_char(x: usize, y: usize) -> bool {
        if x >= VGA_WIDTH || y >= VGA_HEIGHT {
            return false;
        }

        let offset = (y * VGA_WIDTH + x) as usize;
        let color = VGA_MEMORY.add(offset).read_volatile();

        let character = (color & 0xFF) as u8;

        character != b' '
    }

    pub unsafe fn print_at(x: usize, y: usize, text: &str, fg: u8, bg: u8) {
        for (i, c) in text.chars().enumerate() {
            let pos_x = x + i;
            if pos_x < VGA_WIDTH && y < VGA_HEIGHT {
                vga_print_char(c, pos_x, y, fg, bg);
            } else {
                break;
            }
        }
    }

    pub unsafe fn printp(text: &str, fg: u8, bg: u8) {
        let (mut x, mut y) = get_pointer_position();
        for c in text.chars() {
            match c {
                '\n' => {
                    x = 0;
                    y += 1;
                    if y >= VGA_HEIGHT {
                        scroll_up();
                    }
                },
                _ => {
                    if x >= VGA_WIDTH {
                        x = 0;
                        y += 1;
                    }
                    if y >= VGA_HEIGHT {
                        scroll_up();
                    }
                    vga_print_char(c, x, y, fg, bg);
                    x += 1;
                }
            }
        }
        move_cursor(x, y);
    }

    pub unsafe fn get_pointer_position() -> (usize, usize) {
        (CURSOR_COL, CURSOR_ROW)
    }

    fn scroll_up() {
        unsafe {
            let clear_value = ((create_attr_byte(WHITE, BLACK) as u16) << 8) | b' ' as u16;
            let line_size = VGA_WIDTH;
            let total_size = VGA_WIDTH * (VGA_HEIGHT - 1);

            for i in 0..total_size {
                let src = VGA_MEMORY.add(i + line_size);
                let dst = VGA_MEMORY.add(i);
                dst.write_volatile(src.read_volatile());
            }

            for i in (VGA_HEIGHT - 1) * VGA_WIDTH..VGA_HEIGHT * VGA_WIDTH {
                VGA_MEMORY.add(i).write_volatile(clear_value);
            }
        }
    }
}
