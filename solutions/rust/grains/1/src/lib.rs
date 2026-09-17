pub fn square(s: u32) -> u64 {
    if s == 0 || s > 64 {
        panic!();
    } else {
        2u64.pow(s - 1)
    }
}

pub fn total() -> u64 {
    u64::MAX
}
