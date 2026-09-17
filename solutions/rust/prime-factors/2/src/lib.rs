pub fn factors(n: u64) -> Vec<u64> {
    let mut divisors = Vec::new();
    let mut curr_n = n;
    for i in 2..n.isqrt() + 1 {
        while curr_n.is_multiple_of(i) {
            divisors.push(i);
            curr_n /= i;
        }
    }
    if curr_n != 1 {
        divisors.push(curr_n);
    }
    divisors
}
