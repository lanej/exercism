/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    let mut digits = isbn.chars().filter(|c| c.is_ascii_alphanumeric()).rev();

    let check_digit = digits.next().unwrap_or(' ');
    let mut sum = 0 as usize;

    match check_digit {
        'X' => sum += 10,
        '0'..='9' => sum += check_digit.to_digit(10).unwrap() as usize,
        _ => return false,
    };

    let mut size = 1;

    for (i, digit) in digits.enumerate() {
        match digit {
            '0'..='9' => sum += (i + 2) * digit.to_digit(10).unwrap() as usize,
            _ => return false,
        };
        size = size + 1;
    }

    if size != 10 {
        return false;
    }

    return sum % 11 == 0;
}
