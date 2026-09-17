pub fn factors(n: u64) -> Vec<u64> {
    (2..(n.isqrt() + 1).max(3)).fold(vec![n], |mut factorization, i| {
        while factorization[factorization.len() - 1].is_multiple_of(i) {
            let curr_n = factorization.pop().unwrap() / i;
            factorization.extend([i, curr_n]);
        }
        if factorization[factorization.len() - 1] == 1 {
            factorization.pop();
        }
        factorization
    })
}
