use std::fmt::Display;

pub struct Luhn(Option<Vec<i8>>);

impl Luhn {
    #[allow(clippy::cast_possible_truncation)]
    fn digit_value(i: usize, d: i8) -> i8 {
        let position_factor = 1 + (i as i8) % 2; // 2 or 1 depending on the position parity
        let to_range_0_to_9 = |number| (number - 1) % 9 + 1; // We use that n % m is in the range `(-m + 1)..1` if n is negative, so `(-1) % 9 = -1`
        to_range_0_to_9(position_factor * d)
    }

    pub fn is_valid(&self) -> bool {
        match &self.0 {
            None => false,
            Some(vec) if vec.len() <= 1 => false,
            Some(vec) => {
                vec.iter()
                    .rev()
                    .enumerate()
                    .map(|(i, &d)| Self::digit_value(i, d))
                    .sum::<i8>()
                    % 10
                    == 0
            }
        }
    }
}

impl<T: Display> From<T> for Luhn {
    fn from(input: T) -> Self {
        let input_string = input.to_string().replace(' ', "");
        Luhn(
            input_string
                .chars()
                .map(|c| {
                    c.to_digit(10).map(|d| {
                        i8::try_from(d).expect("to_digit() returned a value that doesn't fit a u8")
                    })
                })
                .collect(),
        )
    }
}
