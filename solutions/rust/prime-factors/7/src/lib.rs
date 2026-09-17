pub fn factors(n: u64) -> Vec<u64> {
    std::iter::once(2)
        .chain((3..(n.isqrt() + 1)).step_by(2))
        .fold(vec![n], |mut factorization, i| {
            while factorization[factorization.len() - 1].is_multiple_of(i) {
                let curr_n = factorization.pop().unwrap() / i;
                factorization.extend([i, curr_n]);
            }
            factorization.pop_if(|n| *n == 1);
            factorization
        })
}
