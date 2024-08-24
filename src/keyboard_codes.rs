// keyboard_codes.rs

// Define some commonly used keycodes for convenience.
pub const KEY_A: u8 = 0x1E;
pub const KEY_B: u8 = 0x30;
pub const KEY_C: u8 = 0x2E;
pub const KEY_D: u8 = 0x20;
pub const KEY_E: u8 = 0x12;
pub const KEY_F: u8 = 0x21;
pub const KEY_G: u8 = 0x22;
pub const KEY_H: u8 = 0x23;
pub const KEY_I: u8 = 0x17;
pub const KEY_J: u8 = 0x24;
pub const KEY_K: u8 = 0x25;
pub const KEY_L: u8 = 0x26;
pub const KEY_M: u8 = 0x32;
pub const KEY_N: u8 = 0x31;
pub const KEY_O: u8 = 0x18;
pub const KEY_P: u8 = 0x19;
pub const KEY_Q: u8 = 0x10;
pub const KEY_R: u8 = 0x13;
pub const KEY_S: u8 = 0x1F;
pub const KEY_T: u8 = 0x14;
pub const KEY_U: u8 = 0x16;
pub const KEY_V: u8 = 0x2F;
pub const KEY_W: u8 = 0x11;
pub const KEY_X: u8 = 0x2D;
pub const KEY_Y: u8 = 0x15;
pub const KEY_Z: u8 = 0x2C;

pub const KEY_ESC: u8 = 0x01;
pub const KEY_ENTER: u8 = 0x1C;
pub const KEY_BACKSPACE: u8 = 0x0E;
pub const KEY_TAB: u8 = 0x0F;
pub const KEY_SPACE: u8 = 0x39;
pub const KEY_MINUS: u8 = 0x0D;
pub const KEY_EQUALS: u8 = 0x0D;
pub const KEY_LEFT_BRACKET: u8 = 0x1A;
pub const KEY_RIGHT_BRACKET: u8 = 0x1B;
pub const KEY_BACKSLASH: u8 = 0x2B;
pub const KEY_SEMICOLON: u8 = 0x27;
pub const KEY_APOSTROPHE: u8 = 0x28;
pub const KEY_GRAVE: u8 = 0x29;
pub const KEY_COMMA: u8 = 0x33;
pub const KEY_DOT: u8 = 0x34;
pub const KEY_SLASH: u8 = 0x35;

pub const KEY_CAPS_LOCK: u8 = 0x3A;
pub const KEY_F1: u8 = 0x3B;
pub const KEY_F2: u8 = 0x3C;
pub const KEY_F3: u8 = 0x3D;
pub const KEY_F4: u8 = 0x3E;
pub const KEY_F5: u8 = 0x3F;
pub const KEY_F6: u8 = 0x40;
pub const KEY_F7: u8 = 0x41;
pub const KEY_F8: u8 = 0x42;
pub const KEY_F9: u8 = 0x43;
pub const KEY_F10: u8 = 0x44;
pub const KEY_F11: u8 = 0x57;
pub const KEY_F12: u8 = 0x58;

pub const KEY_UP: u8 = 0x48;
pub const KEY_DOWN: u8 = 0x50;
pub const KEY_LEFT: u8 = 0x4B;
pub const KEY_RIGHT: u8 = 0x4D;

pub const KEY_NUM_LOCK: u8 = 0x45;
pub const KEY_SCROLL_LOCK: u8 = 0x46;
pub const KEY_PAUSE: u8 = 0x45;

// More weird keys

pub const KEY_LEFT_CTRL: u8 = 0x1D;

pub const SELECT_ALL: [u8; 2] = [KEY_LEFT_CTRL, KEY_A];
pub const COPY: [u8; 2] = [KEY_LEFT_CTRL, KEY_C];
pub const SAVE: [u8; 2] = [KEY_LEFT_CTRL, KEY_S];
pub const CUT: [u8; 2] = [KEY_LEFT_CTRL, KEY_X];
pub const PASTE: [u8; 2] = [KEY_LEFT_CTRL, KEY_V];
