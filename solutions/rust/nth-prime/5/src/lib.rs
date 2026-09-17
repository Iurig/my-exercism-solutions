pub fn nth(n: u32) -> u32 {
    let n_f64 = (n + 1).max(6) as f64;
    let upper_bound = (n_f64 * (n_f64.ln() + n_f64.ln().ln())) as usize;
    let upper_bound_sqrt = upper_bound.isqrt();
    let mut primes = vec![true; upper_bound];

    (0..n).fold(2, |prime, _| {
        if prime <= upper_bound_sqrt {
            for i in (2 * prime..upper_bound).step_by(prime) {
                primes[i] = false;
            }
        }
        (prime + 1..).find(|&n| primes[n]).unwrap()
    }) as u32
}
