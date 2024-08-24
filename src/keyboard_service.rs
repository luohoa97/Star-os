// TODO: Hyper combos
// Hyper combos makes it so you can use a key twice in a combo definition by adding... you guessed it... MORE KEYBOARDS!

use x86_64::instructions::port::Port;

const PS2_DATA_PORT: u16 = 0x60;
const PS2_STATUS_PORT: u16 = 0x64;

pub struct KeyboardState {
    pub key_states: [bool; 256],
    pub combo_state: [bool; 256],
}

impl KeyboardState {
    pub fn new() -> Self {
        Self {
            key_states: [false; 256],
            combo_state: [false; 256],
        }
    }

    pub fn update(&mut self) {
        while self.key_available() {
            let scan_code = self.read_scan_code();
            self.process_scan_code(scan_code);
        }
    }

    fn key_available(&self) -> bool {
        unsafe {
            let mut status_port: Port<u8> = Port::new(PS2_STATUS_PORT);
            let status: u8 = status_port.read();
            (status & 0x01) != 0
        }
    }

    fn read_scan_code(&self) -> u8 {
        unsafe {
            let mut data_port: Port<u8> = Port::new(PS2_DATA_PORT);
            data_port.read()
        }
    }

    fn process_scan_code(&mut self, scan_code: u8) {
        let is_key_release = scan_code & 0x80 != 0;
        let actual_code = scan_code & 0x7F;

        if actual_code < self.key_states.len() as u8 {
            self.key_states[actual_code as usize] = !is_key_release;
            self.update_combo_state(actual_code as usize, !is_key_release);
        }
    }

    fn update_combo_state(&mut self, scan_code: usize, is_pressed: bool) {
        if is_pressed {
            self.combo_state[scan_code] = true;
        } else {
            self.combo_state[scan_code] = false;
        }
    }

    pub fn is_combo_active(&self, combo: &[u8]) -> bool {
        combo.iter().all(|&key| self.combo_state[key as usize])
    }

    pub fn get_key_state(&self, scan_code: u8) -> bool {
        if scan_code < self.key_states.len() as u8 {
            self.key_states[scan_code as usize]
        } else {
            false
        }
    }
}

pub fn init_keyboard() -> KeyboardState {
    KeyboardState::new()
}
