/// Check a Luhn checksum.
pub fn digit_value(i: usize, c: char) -> i32{
    let digit = c.to_digit(10).unwrap() as i32;
    ((1 + (i as i32) % 2) * digit - 1) % 9 + 1
}
pub fn is_valid(code: &str) -> bool {
    let code_without_spaces = code.replace(" ", "");
    if code_without_spaces.len() <= 1 || !code_without_spaces.chars().all(|c| c.is_ascii_digit()){
        false
    }
    else {
        let mut sum: i32 = 0;
        for (i, c) in code_without_spaces.chars().rev().enumerate(){
            sum += digit_value(i, c);
        }
        sum % 10 == 0
    }
}
