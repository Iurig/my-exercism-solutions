use std::collections::HashSet;

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut numbers = HashSet::new();
    for &fac in factors {
        for j in 0..limit {
            if j.is_multiple_of(fac) {
                numbers.insert(j);
            }
        }
    }
    numbers.iter().sum()
}
