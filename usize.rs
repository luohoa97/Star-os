use heapless::String;


pub fn usize_to_str(num: usize) -> &'static str {
    static mut BUFFER: [u8; 20] = [0; 20]; // Buffer to hold the string representation
    let mut i = 0;

    // Handle the special case of zero
    if num == 0 {
        unsafe {
            BUFFER[i] = b'0';
            i += 1;
        }
    } else {
        let mut n = num;
        while n > 0 {
            unsafe {
                BUFFER[i] = b'0' + (n % 10) as u8; // Get last digit
                n /= 10;                           // Remove last digit
                i += 1;
            }
        }
    }

    // Reverse the buffer
    for j in 0..i / 2 {
        unsafe {
            let temp = BUFFER[j];
            BUFFER[j] = BUFFER[i - 1 - j];
            BUFFER[i - 1 - j] = temp;
        }
    }

    // Convert to &str and return
    unsafe {
        core::str::from_utf8_unchecked(&BUFFER[..i])
    }
}