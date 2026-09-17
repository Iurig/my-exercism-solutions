#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    (num > 0).then(|| {
        let n_sqrt = num.isqrt();
        let aliquot_sum = 1
            + (2..n_sqrt + 1)
                .filter(|&div| num.is_multiple_of(div))
                .map(|div| div + num / div)
                .sum::<u64>()
            - if n_sqrt.pow(2) == num { n_sqrt } else { 0 };
        dbg!(num, aliquot_sum);
        match aliquot_sum.cmp(&num) {
            std::cmp::Ordering::Greater => Classification::Abundant,
            std::cmp::Ordering::Less => Classification::Deficient,
            std::cmp::Ordering::Equal => Classification::Perfect,
        }
    })
}
