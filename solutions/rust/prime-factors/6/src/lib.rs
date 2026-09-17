pub fn factors(n: u64) -> Vec<u64> {
    (2..(n.isqrt() + 1).max(3)).fold(vec![n], |mut factorization, i| {
        while factorization[factorization.len() - 1].is_multiple_of(i) {
            let fac_len = factorization.len();
            factorization.push(factorization[fac_len - 1] / i);
            factorization[fac_len - 1] = i;
        }
        factorization.pop_if(|n| *n == 1);
        factorization
    })
}
