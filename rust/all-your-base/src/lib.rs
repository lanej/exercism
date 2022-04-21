use std::convert::TryInto;

#[derive(Debug, PartialEq)]
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
/// Return an `Err(.)` if the conversion is impossible.
/// The tests do not test for specific values inside the `Err(.)`.
///
///
/// You are allowed to change the function signature as long as all test still pass.
///
///
/// Example:
/// Input
///   number: &[4, 2]
///   from_base: 10
///   to_base: 2
/// Result
///   Ok(vec![1, 0, 1, 0, 1, 0])
///
/// The example corresponds to converting the number 42 from decimal
/// which is equivalent to 101010 in binary.
///
///
/// Notes:
///  * The empty slice ( "[]" ) is equal to the number 0.
///  * Never output leading 0 digits, unless the input number is 0, in which the output must be `[0]`.
///    However, your function must be able to process input with leading 0 digits.
///
pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    if number.is_empty() {
        return Ok(vec![0]);
    }
    let mut base_10 = number
        .iter()
        .rev()
        .enumerate()
        .fold(0, |acc, (pos, value)| {
            acc + (value * from_base.pow(pos.try_into().unwrap()))
        });

    dbg!(base_10);

    Ok((0..31)
        .rev()
        .map(|pos| match to_base.checked_pow(pos) {
            None => 0,
            Some(pos_value) => {
                if pos_value > base_10 {
                    0
                } else {
                    let (quo, rem) = (base_10 / pos_value, base_10 % pos_value);
                    base_10 = rem;
                    quo
                }
            }
        })
        .skip_while(|x| 0 == *x)
        .collect())
}
