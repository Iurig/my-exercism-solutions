use std::fmt::Display;

pub trait Luhn {
    fn valid_luhn(&self) -> bool;
}

impl<T: Display> Luhn for T {
    fn valid_luhn(&self) -> bool {
        let self_string = self.to_string().replace(" ", "");
        if self_string.len() <= 1 || !self_string.chars().all(|c| c.is_ascii_digit()) {
            false
        } else {
            let mut sum: i32 = 0;
            for (i, c) in self_string.chars().rev().enumerate() {
                fn digit_value(i: usize, c: char) -> i32 {
                    let digit = c.to_digit(10).unwrap() as i32;
                    ((1 + (i as i32) % 2) * digit - 1) % 9 + 1
                }
                sum += digit_value(i, c);
            }
            sum % 10 == 0
        }
    }
}
