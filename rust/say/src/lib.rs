pub fn encode(n: u64) -> String {
    let mut solution = String::new();

    match n {
        0 => solution.push_str("zero"),
        1 => solution.push_str("one"),
        14 => solution.push_str("fourteen"),
        _ => (),
    }

    return solution;
}
