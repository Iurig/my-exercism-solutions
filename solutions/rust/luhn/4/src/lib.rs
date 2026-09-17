#[allow(clippy::cast_possible_wrap)]
pub fn is_valid(code: &str) -> bool {
    code.chars()
        .filter(|c| !c.is_whitespace())
        .rev()
        .try_fold((0, 0), |(i, sum), c| {
            c.to_digit(10)
                .map(|digit| (i + 1, sum + ((1 + i % 2) * digit as i32 - 1) % 9 + 1))
        })
        .is_some_and(|(elements, sum)| elements > 1 && sum % 10 == 0)
}
