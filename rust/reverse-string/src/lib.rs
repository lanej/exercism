pub fn reverse(input: &str) -> String {
    let parts: Vec<String> = input.chars().rev().map(|x| x.to_string()).collect();
    return parts.join("")
}
