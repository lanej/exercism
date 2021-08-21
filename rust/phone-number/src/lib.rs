pub fn number(user_number: &str) -> Option<String> {
    let n = user_number
        .chars()
        .filter(|c| c.is_ascii_digit())
        .take(12)
        .collect::<String>();

    match (n.len(), n.chars().nth(0) {
        11 if n.chars().nth(0).unwrap() == '1' => Some(n),
        11 => None,
        9 if n.chars().nth(0).unwrap() == '2'..'9' => Some(n),
        _ => Some(n),
    }
}
