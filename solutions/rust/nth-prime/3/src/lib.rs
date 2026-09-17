pub fn nth(n: u32) -> u32 {
    let n_f64 = (std::cmp::max(n + 1, 6)) as f64;
    let upper_bound = (n_f64 * (n_f64.ln() + n_f64.ln().ln())) as usize;
    let upper_bound_sqrt = upper_bound.isqrt();
    (2..upper_bound)
        .fold(vec![true; upper_bound], |mut primes, candidate| {
            if primes[candidate] && candidate <= upper_bound_sqrt {
                for j in 2..(upper_bound / candidate) {
                    primes[candidate * j] = false;
                }
            }

            primes
        })
        .iter()
        .enumerate()
        .filter(|&(idx, &is_prime)| idx > 1 && is_prime)
        .nth(n as usize)
        .map(|(idx, _)| idx)
        .unwrap() as u32
}
