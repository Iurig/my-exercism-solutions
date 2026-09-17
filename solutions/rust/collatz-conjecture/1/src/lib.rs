pub fn collatz(n: u64) -> Option<u64> {
    if n == 0 {
        None
    } else if n == 1 {
        Some(0)
    } else if n.is_multiple_of(2) {
        Some(collatz(n / 2)? + 1)
    } else {
        Some(collatz(3 * n + 1)? + 1)
    }
}
