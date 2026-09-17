pub fn is_armstrong_number(num: u32) -> bool {
    let num_digits = num.to_string();
    let mut sum = 0;
    for d in num_digits.chars(){
        sum += d.to_digit(10).unwrap().pow(num_digits.len() as u32);
    }
    sum == num
}
