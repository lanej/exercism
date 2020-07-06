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
///  * Never output leading 0 digits. However, your function must be able to
///     process input with leading 0 digits.
///
pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    let mut remainder = number.iter().enumerate().fold(0u32, |sum, (index, val)| {
        sum + from_base.pow(index as u32) * val
    }) as i32;

    dbg!(to_base);
    dbg!((remainder as f64).log(to_base as f64));
    let positions = f64::ceil((remainder as f64).log(to_base as f64)) as u32;

    let mut answer = Vec::new();

    for position in (1..positions).rev() {
        dbg!(position);
        let taken = to_base.pow(position) as i32;
        dbg!(taken);
        let position_value = remainder - taken;
        dbg!(position_value);

        if position_value < 0 {
            answer.push(0);
        } else {
            answer.push(to_base - 1);
            remainder -= taken;
        }
    }

    if remainder > 0 {
        answer.push(remainder as u32)
    }

    Ok(answer)
}
