use std::collections::HashMap;

pub fn brackets_are_balanced(string: &str) -> bool {
    let brackets: HashMap<char,char> = [
        (']','['),
        (')','('),
        ('}','{'),
    ].iter().cloned().collect();

    let mut stack: Vec<char> = Vec::new();

    for c in string.chars() {
        let last_bracket = stack.last().unwrap_or(&'\t');

        if brackets.get(&c).unwrap_or(&'\n') == last_bracket {
            stack.pop();
            continue;
        } else if brackets.values().any(|b| b == &c) {
            stack.push(c);
            continue;
        } 

        match brackets.get(&c) {
            Some(_) => return false,
            _ => continue,
        }
    }

    return stack.is_empty();
}
