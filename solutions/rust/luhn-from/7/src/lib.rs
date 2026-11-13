use std::fmt::Display;

pub struct Luhn(Vec<i8>);

impl Luhn {
    #[allow(clippy::cast_possible_truncation)]
    fn digit_value(i: usize, d: i8) -> i8 {
        let position_factor = 1 + (i as i8) % 2;
        let to_range_0_to_9 = |number| (number - 1) % 9 + 1; // We use that n % m is in the range `(-m + 1)..1` if n is negative, so `(-1) % 9 = -1`
        to_range_0_to_9(position_factor * d)
    }

    pub fn is_valid(&self) -> bool {
        self.0
            .iter()
            .rev()
            .enumerate()
            .map(|(i, &d)| Self::digit_value(i, d))
            .sum::<i8>()
            % 10
            == 0
    }
}

impl<T: Display> From<T> for Luhn {
    fn from(input: T) -> Self {
        let input_string = input.to_string().replace(' ', "");
        Luhn(
            input_string
                .chars()
                .map(|c| {
                    i8::try_from(
                        c.to_digit(10)
                            .unwrap_or_else(|| panic!("input contains non digit {c}")),
                    )
                    .expect("to_digit() returned a value that doesn't fit a u8")
                })
                .collect(),
        )
    }
}
