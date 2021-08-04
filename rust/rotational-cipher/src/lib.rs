pub fn rotate(input: &str, key: i8) -> String {
    let offset = key.rem_euclid(26) as u8;
    let rotate = |a: u8, c: u8| a + ((c - a) + offset) % 26;
    input
        .chars()
        .map(|c| match c {
            'a'..='z' => rotate('a' as u8, c as u8) as char,
            'A'..='Z' => rotate('A' as u8, c as u8) as char,
            _ => c,
        })
        .collect::<String>()
}
