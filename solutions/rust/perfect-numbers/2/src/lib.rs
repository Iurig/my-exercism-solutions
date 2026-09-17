#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    let mut aliquot_sum: u64 = 0;
    for i in 1..num {
        if num.is_multiple_of(i) {
            aliquot_sum += i;
        }
    }
    match aliquot_sum {
        x if x > num => Some(Classification::Abundant),
        x if x < num => Some(Classification::Deficient),
        _ if num == 0 => None,
        _ => Some(Classification::Perfect),
    }
}
