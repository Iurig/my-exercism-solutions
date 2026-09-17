pub fn factors(n: u64) -> Vec<u64> {
    (2..n.isqrt() + 2).fold(vec![n], |mut divisors, i| {
        while divisors[divisors.len() - 1].is_multiple_of(i) {
            let curr_n = divisors.pop().unwrap() / i;
            divisors.extend([i, curr_n]);
        }
        if divisors[divisors.len() - 1] == 1 {
            divisors.pop();
        }
        divisors
    })
}
