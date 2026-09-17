#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    let aliquot_sum = (1..num).fold(0, |sum, index| {
        if num.is_multiple_of(index) {
            sum + index
        } else {
            sum
        }
    });
    match aliquot_sum {
        x if x > num => Some(Classification::Abundant),
        x if x < num => Some(Classification::Deficient),
        _ if num == 0 => None,
        _ => Some(Classification::Perfect),
    }
}
