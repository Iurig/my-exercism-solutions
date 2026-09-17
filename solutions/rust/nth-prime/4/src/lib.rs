pub fn nth(n: u32) -> u32 {
    let n_f64 = (std::cmp::max(n + 1, 6)) as f64;
    let upper_bound = (n_f64 * (n_f64.ln() + n_f64.ln().ln())) as usize;
    let upper_bound_sqrt = upper_bound.isqrt();

    (2..upper_bound)
        .fold(vec![true; upper_bound], |mut primes, candidate| {
            if primes[candidate] && candidate <= upper_bound_sqrt {
                for j in (2 * candidate..upper_bound).step_by(candidate) {
                    primes[j] = false;
                }
            }
            primes
        })
        .into_iter()
        .enumerate()
        .skip(2)
        .filter_map(|(idx, is_prime)| is_prime.then_some(idx))
        .nth(n as usize)
        .unwrap() as u32
}
