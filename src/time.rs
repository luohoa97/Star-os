// time rs STD for kernel comp programmed by luohoa97

#![no_std]

extern crate core;

use core::arch::asm;

pub struct UnixTime {
    pub seconds: u64,
}

impl UnixTime {
    pub fn new() -> Self {
        Self { seconds: 0 }
    }

    pub fn from_seconds(seconds: u64) -> Self {
        Self { seconds }
    }

    pub fn as_seconds(&self) -> u64 {
        self.seconds
    }

    pub fn update(&mut self) {
        self.seconds = get_system_time_seconds();
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
    let result: u8;
    unsafe {
        asm!(
            "outb %al, %dx",
            "inb %dx, %al",
            in("dx") CMOS_ADDRESS,
            inout("al") register => _,
            out("al") result,
        );
    }
    result
}

// Convert BCD to binary
pub fn bcd_to_binary(bcd: u8) -> u8 {
    (bcd & 0x0F) + ((bcd >> 4) * 10)
}

pub fn get_system_time_seconds() -> u64 {
    let seconds = bcd_to_binary(read_rtc(SECONDS)) as u64;
    let minutes = bcd_to_binary(read_rtc(MINUTES)) as u64;
    let hours = bcd_to_binary(read_rtc(HOURS)) as u64;
    let day = bcd_to_binary(read_rtc(DAY)) as u64;
    let month = bcd_to_binary(read_rtc(MONTH)) as u64;
    let year = bcd_to_binary(read_rtc(YEAR)) as u64;
    let century = bcd_to_binary(read_rtc(CENTURY)) as u64;
    let full_year = century * 100 + year;
    convert_to_unix_time(full_year, month, day, hours, minutes, seconds)
}

pub fn convert_to_unix_time(year: u64, month: u64, day: u64, hours: u64, minutes: u64, seconds: u64) -> u64 {
    let days_since_epoch = (year - 1970) * 365 + ((month - 1) * 30) + (day - 1);
    days_since_epoch * 86400 + hours * 3600 + minutes * 60 + seconds
}

pub fn get_current_unix_time() -> UnixTime {
    let system_time_seconds = get_system_time_seconds();
    UnixTime::from_seconds(system_time_seconds)
}
