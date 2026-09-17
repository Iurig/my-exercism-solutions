pub fn nth(n: u32) -> u32 {
    let n_f64 = (std::cmp::max(n + 1, 6)) as f64;
    let upper_bound = (n_f64 * (n_f64.ln() + n_f64.ln().ln())) as usize;
    let mut primes = vec![true; upper_bound];
    let mut count = 0;
    for i in 2..upper_bound {
        if primes[i] {
            count += 1;
        }
        if count == n + 1 {
            return i as u32;
        }
        if i <= upper_bound.isqrt() + 1 {
            for j in 2..(upper_bound / i) {
                primes[i * j] = false;
            }
        }
    }
    panic!()
}
