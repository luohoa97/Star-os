// time rs STD for kernel comp programmed by luohoa97

#![no_std]

extern crate x86_64;

use x86_64::instructions::port::Port;

pub struct UnixTime {
    pub timestamp: u64, // Number of milliseconds since the Unix epoch
}

impl UnixTime {
    pub fn new() -> Self {
        Self { timestamp: 0 }
    }

    pub fn from_timestamp(timestamp: u64) -> Self {
        Self { timestamp }
    }

    pub fn as_seconds(&self) -> u64 {
        self.timestamp / 1_000
    }

    pub fn as_milliseconds(&self) -> u64 {
        self.timestamp
    }

    pub fn update(&mut self) {
        self.timestamp = get_system_time_milliseconds();
    }
}

pub const CMOS_ADDRESS: u16 = 0x70;
pub const CMOS_DATA: u16 = 0x71;

pub const SECONDS: u8 = 0x00;
pub const MINUTES: u8 = 0x02;
pub const HOURS: u8 = 0x04;
pub const DAY: u8 = 0x07;
pub const MONTH: u8 = 0x08;
pub const YEAR: u8 = 0x09;
pub const CENTURY: u8 = 0x32;

pub fn read_rtc(register: u8) -> u8 {
    let mut port = Port::new(CMOS_ADDRESS);
    unsafe {
        port.write(register); // Write register to CMOS_ADDRESS
    }

    let mut port_data = Port::new(CMOS_DATA);
    let result: u8;
    unsafe {
        result = port_data.read(); // Read the value from CMOS_DATA
    }

    result
}

pub fn bcd_to_binary(bcd: u8) -> u8 {
    (bcd & 0x0F) + ((bcd >> 4) * 10)
}

pub fn get_system_time_milliseconds() -> u64 {
    let seconds = bcd_to_binary(read_rtc(SECONDS)) as u64;
    let minutes = bcd_to_binary(read_rtc(MINUTES)) as u64;
    let hours = bcd_to_binary(read_rtc(HOURS)) as u64;
    let day = bcd_to_binary(read_rtc(DAY)) as u64;
    let month = bcd_to_binary(read_rtc(MONTH)) as u64;
    let year = bcd_to_binary(read_rtc(YEAR)) as u64;
    let century = bcd_to_binary(read_rtc(CENTURY)) as u64;

    let full_year = century * 100 + year;
    let total_seconds = convert_to_unix_time(full_year, month, day, hours, minutes, seconds);

    total_seconds * 1_000 // Convert seconds to milliseconds
}

pub fn convert_to_unix_time(year: u64, month: u64, day: u64, hours: u64, minutes: u64, seconds: u64) -> u64 {
    let days_since_epoch = (year - 1970) * 365 + ((month - 1) * 30) + (day - 1);
    days_since_epoch * 86_400 + hours * 3_600 + minutes * 60 + seconds
}

pub fn get_current_unix_time() -> UnixTime {
    let timestamp = get_system_time_milliseconds();
    UnixTime::from_timestamp(timestamp)
}
