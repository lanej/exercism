pub fn series(digits: &str, len: usize) -> Vec<String> {
    if len > digits.len() {
        return vec![];
    }

    return (0..=digits.len() - len)
        .map(|x| digits[x..x + len].to_string())
        .collect();
}
