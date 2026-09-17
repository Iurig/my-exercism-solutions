use std::fmt::Display;

pub trait Luhn {
    fn valid_luhn(&self) -> bool;
}

impl<T: Display> Luhn for T {
    fn valid_luhn(&self) -> bool {
        let self_string = self.to_string().replace(' ', "");
        if self_string.len() <= 1 || !self_string.chars().all(|c| c.is_ascii_digit()) {
            false
        } else {
            let mut sum: u32 = 0;
            for (i, c) in self_string.chars().rev().enumerate() {
                #[allow(clippy::cast_possible_truncation)]
                fn digit_value(i: usize, c: char) -> u32 {
                    let digit = c.to_digit(10).unwrap();
                    (1 + (i as u32) % 2) * digit
                        - if (1 + (i as u32) % 2) * digit >= 9 {
                            9
                        } else {
                            0
                        }
                }
                sum += digit_value(i, c);
            }
            sum.is_multiple_of(10)
        }
    }
}
