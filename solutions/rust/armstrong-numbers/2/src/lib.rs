pub fn is_armstrong_number(num: u32) -> bool {
    let size = num.to_string().len() as u32;
    num == num
        .to_string()
        .chars()
        .fold(0, |sum, c| sum + c.to_digit(10).unwrap().pow(size))
}
