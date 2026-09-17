pub fn is_armstrong_number(num: u32) -> bool {
    let size = num.checked_ilog10().unwrap_or(0) + 1;
    num == std::iter::successors(Some(num), |&n| (n > 0).then_some(n / 10))
        .map(|n| (n % 10).pow(size))
        .sum()
}
