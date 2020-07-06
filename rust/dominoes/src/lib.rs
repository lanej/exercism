pub fn chain(input: &[(u8, u8)]) -> Option<Vec<(u8, u8)>> {
    let mut solution = Vec::with_capacity(input.len());
    for domino in input {
        solution.push(domino.clone());
    }

    if input.len() == 0 {
        return Some(vec![]);
    }

    if let (start, first_end) = input.first().unwrap() {
    } else {
        return Some(vec![]);
    }
    return None;
}
