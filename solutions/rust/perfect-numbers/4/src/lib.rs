#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    let aliquot_sum = (1..num.isqrt() + 1)
        .filter(|&div| num.is_multiple_of(div))
        .map(|div| div + if div * div != num { num / div } else { 0 })
        .sum::<u64>()
        - num;
    match aliquot_sum {
        x if x > num => Some(Classification::Abundant),
        x if x < num => Some(Classification::Deficient),
        _ if num == 0 => None,
        _ => Some(Classification::Perfect),
    }
}
