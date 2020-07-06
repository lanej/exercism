/// Determine whether a sentence is a pangram.
pub fn is_pangram(sentence: &str) -> bool {
    let mut set = [0; 36];
    sentence
        .chars()
        .for_each(|c| c.to_ascii_lowercase());
    return false;
}
