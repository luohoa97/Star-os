#![no_std]

use core::fmt::Write;
use heapless::String;

pub struct AttributeCombiner {
    buffer: String<256>, // Adjust the buffer size as needed
    separator: char,
}

impl AttributeCombiner {
    pub fn new(separator: char) -> Self {
        AttributeCombiner {
            buffer: String::new(),
            separator,
        }
    }

    pub fn add_attribute(&mut self, attr: &str) {
        if !self.buffer.is_empty() {
            self.buffer.push(self.separator).ok(); // Add separator
        }
        self.buffer.push_str(attr).ok(); // Append attribute
    }

    pub fn combine(&self) -> &String<256> {
        &self.buffer
    }
}
