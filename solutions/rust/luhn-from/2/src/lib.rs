use std::fmt::Display;

pub struct Luhn(String);

impl Luhn {
    pub fn digit_value(i: usize, c: char) -> i32 {
        let digit = c.to_digit(10).unwrap() as i32;
        ((1 + (i as i32) % 2) * digit - 1) % 9 + 1
    }
    pub fn is_valid(&self) -> bool {
        if self.0.len() <= 1 || !self.0.chars().all(|c| c.is_ascii_digit()) {
            false
        } else {
            let mut sum: i32 = 0;
            for (i, c) in self.0.chars().rev().enumerate() {
                sum += Self::digit_value(i, c);
            }
            sum % 10 == 0
        }
    }
}

impl<T: Display> From<T> for Luhn {
    fn from(input: T) -> Self {
        let input_string: String = input.to_string().replace(" ", "");
        Luhn(input_string)
    }
}
