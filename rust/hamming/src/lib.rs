/// Return the Hamming distance between the strings,
/// or None if the lengths are mismatched.
pub fn hamming_distance(s1: &str, s2: &str) -> Option<usize> {
    if s1.len() != s2.len() {
        return None;
    }

    let mut distance: usize = 0;
    let mut s2_chars = s2.chars();

    for l in s1.chars() {
        if s2_chars.next().unwrap() != l {
            distance += 1;
        }
    }

    Some(distance)
}
