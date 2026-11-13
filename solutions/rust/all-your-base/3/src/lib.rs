#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

///
/// Convert a number between two bases.
///
/// A number is any slice of digits.
/// A digit is any unsigned integer (e.g. u8, u16, u32, u64, or usize).
/// Bases are specified as unsigned integers.
///
/// Return the corresponding Error enum if the conversion is impossible.
///
///
/// You are allowed to change the function signature as long as all test still pass.
///
///
/// Example:
/// Input
///   `number`: &[4, 2]
///   `from_base`: 10
///   `to_base`: 2
/// Result
///   Ok(vec![1, 0, 1, 0, 1, 0])
///
/// The example corresponds to converting the number 42 from decimal
/// which is equivalent to 101010 in binary.
///
/// # Errors
/// Errors if either base is invalid (0 or 1), or if the digits of the number are impossible in the
/// input base.
///
/// Notes:
///  * The empty slice ( "[]" ) is equal to the number 0.
///  * Never output leading 0 digits, unless the input number is 0, in which the output must be
///    `[0]`. However, your function must be able to process input with leading 0 digits.
pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    if from_base <= 1 {
        Err(Error::InvalidInputBase)
    } else if to_base <= 1 {
        Err(Error::InvalidOutputBase)
    } else {
        fn to_value(number: &[u32], base: u32) -> Result<u32, Error> {
            number
                .iter()
                .rev()
                .try_fold((0, 1), |(sum, power), &digit| {
                    (digit < base)
                        .then_some((sum + power * digit, power * base))
                        .ok_or(Error::InvalidDigit(digit))
                })
                .map(|pair| pair.0)
        }

        fn from_value(value: u32, base: u32) -> Vec<u32> {
            let mut value = value;
            let mut number = Vec::new();

            while value > 0 {
                number.push(value % base);
                value /= base;
            }

            number.reverse();
            number
        }

        let value = to_value(number, from_base)?;
        let mut number_to_base = from_value(value, to_base);

        if number_to_base.is_empty() {
            number_to_base.push(0);
        }
        Ok(number_to_base)
    }
}
