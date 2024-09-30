#![no_std]

pub fn uxstring(value: u64) -> &'static str {
    static mut BUFFER: [u8; 20] = [0; 20];

    let mut idx = 19;
    let mut num = value;

    if num == 0 {
        unsafe {
            BUFFER[idx] = b'0';
            return core::str::from_utf8_unchecked(&BUFFER[idx..]);
        }
    }

    while num > 0 {
        let digit = (num % 10) as u8;
        unsafe {
            BUFFER[idx] = b'0' + digit;
        }
        idx -= 1;
        num /= 10;
    }

    let start_idx = idx + 1;
    let length = 20 - start_idx;

    unsafe {
        core::ptr::copy_nonoverlapping(&BUFFER[start_idx], &mut BUFFER[0], length);
        BUFFER[length] = b'\0'; // Null-terminate the string
        core::str::from_utf8_unchecked(&BUFFER[..length])
    }
}
