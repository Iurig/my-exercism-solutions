/// # Panics
///
/// Panics if the input is not a valid chess board square
pub fn square(s: u32) -> u64 {
    assert!(
        s != 0 && s <= 64,
        "input must be a valid square on a chess board (0 < input <= 64)"
    );

    2u64.pow(s - 1)
}

pub fn total() -> u64 {
    u64::MAX
}
